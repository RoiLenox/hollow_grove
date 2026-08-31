use std::collections::BTreeMap;
use std::env;
use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::Command;

const IN_CLOSE_WRITE: u32 = 0x0000_0008;
const IN_MOVED_TO: u32 = 0x0000_0080;
const IN_CREATE: u32 = 0x0000_0100;
const IN_DELETE_SELF: u32 = 0x0000_0400;
const IN_MOVE_SELF: u32 = 0x0000_0800;
const IN_ISDIR: u32 = 0x4000_0000;
const IN_CLOEXEC: c_int = 0x0008_0000;

#[repr(C)]
#[derive(Clone, Copy)]
struct InotifyEvent {
    wd: c_int,
    mask: u32,
    cookie: u32,
    len: u32,
}

unsafe extern "C" {
    fn inotify_init1(flags: c_int) -> c_int;
    fn inotify_add_watch(fd: c_int, path: *const c_char, mask: u32) -> c_int;
    fn read(fd: c_int, buffer: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
}

struct InotifyFd(c_int);

impl Drop for InotifyFd {
    fn drop(&mut self) {
        // SAFETY: this value owns a valid descriptor returned by inotify_init1.
        unsafe {
            close(self.0);
        }
    }
}

fn add_directory(
    fd: c_int,
    path: &Path,
    watches: &mut BTreeMap<c_int, PathBuf>,
) -> Result<(), String> {
    let encoded = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| format!("watch path contains a NUL byte: {}", path.display()))?;
    // SAFETY: encoded is NUL-terminated and remains alive for the call.
    let watch = unsafe {
        inotify_add_watch(
            fd,
            encoded.as_ptr(),
            IN_CLOSE_WRITE | IN_MOVED_TO | IN_CREATE | IN_DELETE_SELF | IN_MOVE_SELF,
        )
    };
    if watch < 0 {
        return Err(format!(
            "could not watch {}: {}",
            path.display(),
            std::io::Error::last_os_error()
        ));
    }
    watches.insert(watch, path.to_path_buf());
    Ok(())
}

fn add_tree(fd: c_int, root: &Path, watches: &mut BTreeMap<c_int, PathBuf>) -> Result<(), String> {
    add_directory(fd, root, watches)?;
    for entry in
        fs::read_dir(root).map_err(|error| format!("could not scan {}: {error}", root.display()))?
    {
        let entry = entry.map_err(|error| format!("could not read asset directory: {error}"))?;
        let file_type = entry
            .file_type()
            .map_err(|error| format!("could not inspect {}: {error}", entry.path().display()))?;
        if file_type.is_dir() {
            add_tree(fd, &entry.path(), watches)?;
        }
    }
    Ok(())
}

fn is_aseprite_source(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(extension.to_ascii_lowercase().as_str(), "ase" | "aseprite")
        })
}

fn export_changed(repo_root: &Path, path: &Path) -> Result<(), String> {
    let status = Command::new(repo_root.join("export-aseprite.sh"))
        .arg(path)
        .status()
        .map_err(|error| format!("could not start Aseprite export: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("Aseprite export exited with {status}"))
    }
}

pub fn run(args: &[String]) -> Result<(), String> {
    let mut root = env::var_os("HOLLOW_GROVE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    match args {
        [] => {}
        [flag, value] if flag == "--root" => root = PathBuf::from(value),
        _ => return Err(String::from("asset-watch accepts only --root PATH")),
    }
    let root = root
        .canonicalize()
        .map_err(|error| format!("could not resolve {}: {error}", root.display()))?;
    let source_root = root.join("hueman_godot/assets/source");
    fs::create_dir_all(&source_root)
        .map_err(|error| format!("could not create {}: {error}", source_root.display()))?;

    // SAFETY: inotify_init1 has no pointer arguments and returns a new descriptor.
    let raw_fd = unsafe { inotify_init1(IN_CLOEXEC) };
    if raw_fd < 0 {
        return Err(format!(
            "could not initialize inotify: {}",
            std::io::Error::last_os_error()
        ));
    }
    let fd = InotifyFd(raw_fd);
    let mut watches = BTreeMap::new();
    add_tree(fd.0, &source_root, &mut watches)?;
    let header_size = std::mem::size_of::<InotifyEvent>();
    let mut buffer = vec![0_u8; 64 * 1024];

    loop {
        // SAFETY: buffer points to writable storage of exactly buffer.len() bytes.
        let bytes = unsafe { read(fd.0, buffer.as_mut_ptr().cast(), buffer.len()) };
        if bytes < 0 {
            return Err(format!(
                "could not read inotify events: {}",
                std::io::Error::last_os_error()
            ));
        }
        let mut offset = 0usize;
        while offset + header_size <= bytes as usize {
            // SAFETY: the kernel filled a complete fixed-size event header at this offset.
            let event = unsafe {
                buffer
                    .as_ptr()
                    .add(offset)
                    .cast::<InotifyEvent>()
                    .read_unaligned()
            };
            let event_size = header_size + event.len as usize;
            if offset + event_size > bytes as usize {
                break;
            }
            let Some(parent) = watches.get(&event.wd).cloned() else {
                offset += event_size;
                continue;
            };
            let name_bytes = &buffer[offset + header_size..offset + event_size];
            let name = if name_bytes.is_empty() {
                None
            } else {
                // SAFETY: inotify names are NUL-terminated within event.len bytes.
                Some(unsafe { CStr::from_ptr(name_bytes.as_ptr().cast()) }.to_bytes())
            };
            let path = name.map_or_else(
                || parent.clone(),
                |name| parent.join(std::ffi::OsStr::from_bytes(name)),
            );
            if event.mask & IN_ISDIR != 0 && event.mask & (IN_CREATE | IN_MOVED_TO) != 0 {
                if path.is_dir() {
                    add_tree(fd.0, &path, &mut watches)?;
                }
            } else if event.mask & (IN_CLOSE_WRITE | IN_MOVED_TO) != 0
                && path.is_file()
                && is_aseprite_source(&path)
            {
                export_changed(&root, &path)?;
            }
            offset += event_size;
        }
    }
}
