use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime};

static ATOMIC_WRITE_NONCE: AtomicU64 = AtomicU64::new(0);

struct ArtifactCache {
    contents: HashMap<PathBuf, CachedArtifact>,
    ensured_dirs: HashSet<PathBuf>,
}

struct CachedArtifact {
    contents: Vec<u8>,
    len: u64,
    modified: Option<SystemTime>,
}

thread_local! {
    static ARTIFACT_CACHE: RefCell<ArtifactCache> = RefCell::new(ArtifactCache {
            contents: HashMap::new(),
            ensured_dirs: HashSet::new(),
    });
}

fn cached_text_artifact_matches(path: &Path, contents: &str) -> bool {
    let target_bytes = contents.as_bytes();
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return false,
    };
    let modified = metadata.modified().ok();
    ARTIFACT_CACHE.with(|cache| {
        cache.borrow().contents.get(path).is_some_and(|cached| {
            cached.contents.as_slice() == target_bytes
                && cached.len == metadata.len()
                && cached.modified == modified
        })
    })
}

fn cache_artifact(path: &Path, contents: Vec<u8>) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    let cached = CachedArtifact {
        contents,
        len: metadata.len(),
        modified: metadata.modified().ok(),
    };
    ARTIFACT_CACHE.with(|cache| {
        cache
            .borrow_mut()
            .contents
            .insert(path.to_path_buf(), cached);
    });
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactFlushRecord {
    pub path: PathBuf,
    pub elapsed: Duration,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ArtifactSession {
    staged: Vec<(PathBuf, String)>,
}

impl ArtifactSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_text_artifact(&self, path: &Path) -> io::Result<String> {
        self.staged
            .iter()
            .rev()
            .find(|(staged_path, _)| staged_path == path)
            .map(|(_, contents)| contents.clone())
            .map_or_else(|| read_text_artifact(path), Ok)
    }

    pub fn stage_text_artifact(&mut self, path: &Path, contents: impl Into<String>) {
        let contents = contents.into();
        if let Some((_, staged_contents)) = self
            .staged
            .iter_mut()
            .find(|(staged_path, _)| staged_path == path)
        {
            *staged_contents = contents;
            return;
        }

        if cached_text_artifact_matches(path, &contents) {
            return;
        }

        self.staged.push((path.to_path_buf(), contents));
    }

    pub fn commit(self) -> io::Result<()> {
        let mut parents = HashSet::new();
        for (path, contents) in self.staged {
            if write_text_artifact_with_parent_sync(&path, &contents, false)? {
                parents.insert(atomic_parent(&path).to_path_buf());
            }
        }
        sync_parent_directories(&parents)
    }

    pub fn commit_timed(self) -> io::Result<Vec<ArtifactFlushRecord>> {
        let mut flush_records = Vec::with_capacity(self.staged.len());
        let mut parents = HashSet::new();

        for (path, contents) in self.staged {
            let started = Instant::now();
            if write_text_artifact_with_parent_sync(&path, &contents, false)? {
                parents.insert(atomic_parent(&path).to_path_buf());
            }
            flush_records.push(ArtifactFlushRecord {
                path,
                elapsed: started.elapsed(),
            });
        }

        sync_parent_directories(&parents)?;
        Ok(flush_records)
    }
}

fn ensure_cached_parent_dir(path: &Path) -> io::Result<()> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };

    let parent_is_cached =
        ARTIFACT_CACHE.with(|cache| cache.borrow().ensured_dirs.contains(parent));
    if parent_is_cached {
        return Ok(());
    }

    fs::create_dir_all(parent)?;
    ARTIFACT_CACHE.with(|cache| {
        cache.borrow_mut().ensured_dirs.insert(parent.to_path_buf());
    });
    Ok(())
}

pub fn read_text_artifact(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

struct PendingAtomicWrite {
    path: PathBuf,
    committed: bool,
}

impl Drop for PendingAtomicWrite {
    fn drop(&mut self) {
        if !self.committed {
            let _ = fs::remove_file(&self.path);
        }
    }
}

fn atomic_parent(path: &Path) -> &Path {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

fn create_atomic_temp(path: &Path) -> io::Result<(File, PendingAtomicWrite)> {
    let parent = atomic_parent(path);
    fs::create_dir_all(parent)?;
    let file_name = path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "atomic write target must have a file name",
        )
    })?;

    for _ in 0..100 {
        let nonce = ATOMIC_WRITE_NONCE.fetch_add(1, Ordering::Relaxed);
        let mut temp_name = OsString::from(".");
        temp_name.push(file_name);
        temp_name.push(format!(
            ".hollow-grove-atomic-{}-{nonce}",
            std::process::id()
        ));
        let temp_path = parent.join(temp_name);
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)
        {
            Ok(file) => {
                return Ok((
                    file,
                    PendingAtomicWrite {
                        path: temp_path,
                        committed: false,
                    },
                ));
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    }

    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "could not allocate a unique atomic-write staging file",
    ))
}

fn write_bytes_atomically_with(
    path: &Path,
    contents: &[u8],
    before_rename: impl FnOnce(&Path) -> io::Result<()>,
) -> io::Result<()> {
    write_bytes_atomically_with_parent_sync(path, contents, before_rename, true)
}

fn write_bytes_atomically_with_parent_sync(
    path: &Path,
    contents: &[u8],
    before_rename: impl FnOnce(&Path) -> io::Result<()>,
    sync_parent: bool,
) -> io::Result<()> {
    let existing_permissions = match fs::metadata(path) {
        Ok(metadata) => Some(metadata.permissions()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => return Err(error),
    };
    let (mut file, mut pending) = create_atomic_temp(path)?;
    if let Some(permissions) = existing_permissions {
        file.set_permissions(permissions)?;
    }
    file.write_all(contents)?;
    file.sync_all()?;
    drop(file);

    before_rename(&pending.path)?;
    fs::rename(&pending.path, path)?;
    pending.committed = true;
    if sync_parent {
        File::open(atomic_parent(path))?.sync_all()?;
    }
    Ok(())
}

fn sync_parent_directories(parents: &HashSet<PathBuf>) -> io::Result<()> {
    for parent in parents {
        File::open(parent)?.sync_all()?;
    }
    Ok(())
}

/// Durably replaces one file without exposing partially written contents.
///
/// Bytes are flushed to a unique sibling file, renamed over the destination,
/// and followed by a parent-directory sync. If staging fails, the old file is
/// left in place and the sibling is removed.
pub fn write_bytes_atomically(path: &Path, contents: &[u8]) -> io::Result<()> {
    write_bytes_atomically_with(path, contents, |_| Ok(()))
}

pub fn write_text_artifact(path: &Path, contents: &str) -> io::Result<()> {
    write_text_artifact_with_parent_sync(path, contents, true).map(|_| ())
}

fn write_text_artifact_with_parent_sync(
    path: &Path,
    contents: &str,
    sync_parent: bool,
) -> io::Result<bool> {
    let target_bytes = contents.as_bytes();

    if cached_text_artifact_matches(path, contents) {
        return Ok(false);
    }

    ensure_cached_parent_dir(path)?;

    match fs::metadata(path) {
        Ok(metadata) if metadata.len() == target_bytes.len() as u64 => {
            let existing_bytes = fs::read(path)?;
            if existing_bytes == target_bytes {
                cache_artifact(path, existing_bytes)?;
                return Ok(false);
            }
        }
        Ok(_) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }

    write_bytes_atomically_with_parent_sync(path, target_bytes, |_| Ok(()), sync_parent)?;
    cache_artifact(path, target_bytes.to_vec())?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::{
        ArtifactSession, read_text_artifact, write_bytes_atomically, write_bytes_atomically_with,
        write_text_artifact,
    };
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    use std::thread;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nonce}"))
    }

    #[test]
    fn write_text_artifact_creates_parent_directories_and_writes_contents() {
        let root = unique_temp_dir("hollow-grove-artifact-io");
        let path = root.join("artifacts/example.txt");

        write_text_artifact(&path, "alpha").expect("artifact should write");

        assert_eq!(
            read_text_artifact(&path).expect("artifact should read"),
            "alpha"
        );

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn atomic_byte_write_replaces_complete_contents_and_leaves_no_staging_file() {
        let root = unique_temp_dir("hollow-grove-atomic-byte-write");
        let path = root.join("archives/state.bin");

        write_bytes_atomically(&path, b"first").expect("initial archive should write");
        write_bytes_atomically(&path, b"second").expect("replacement archive should write");

        assert_eq!(fs::read(&path).expect("archive should read"), b"second");
        assert_eq!(
            fs::read_dir(path.parent().unwrap())
                .expect("archive directory should read")
                .count(),
            1
        );

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn failed_atomic_write_preserves_last_good_file_and_cleans_staging_file() {
        let root = unique_temp_dir("hollow-grove-atomic-byte-failure");
        let path = root.join("archives/state.bin");
        write_bytes_atomically(&path, b"last-good").expect("initial archive should write");

        let error = write_bytes_atomically_with(&path, b"incomplete", |_| {
            Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "simulated interruption before rename",
            ))
        })
        .expect_err("simulated interruption should fail");

        assert_eq!(error.kind(), io::ErrorKind::Interrupted);
        assert_eq!(
            fs::read(&path).expect("last-good archive should read"),
            b"last-good"
        );
        assert_eq!(
            fs::read_dir(path.parent().unwrap())
                .expect("archive directory should read")
                .count(),
            1
        );

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn write_text_artifact_skips_rewriting_identical_contents() {
        let root = unique_temp_dir("hollow-grove-artifact-io-skip");
        let path = root.join("artifacts/example.txt");

        write_text_artifact(&path, "alpha").expect("initial artifact should write");
        let first_modified = fs::metadata(&path)
            .expect("metadata should exist")
            .modified()
            .expect("modified time should exist");

        thread::sleep(Duration::from_millis(5));
        write_text_artifact(&path, "alpha").expect("identical artifact should no-op");
        let second_modified = fs::metadata(&path)
            .expect("metadata should exist")
            .modified()
            .expect("modified time should exist");

        assert_eq!(first_modified, second_modified);

        thread::sleep(Duration::from_millis(5));
        write_text_artifact(&path, "beta").expect("updated artifact should rewrite");
        let third_modified = fs::metadata(&path)
            .expect("metadata should exist")
            .modified()
            .expect("modified time should exist");

        assert!(third_modified > second_modified);
        assert_eq!(
            read_text_artifact(&path).expect("artifact should read"),
            "beta"
        );

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn artifact_cache_detects_an_external_replacement() {
        let root = unique_temp_dir("hollow-grove-artifact-cache-external-write");
        let path = root.join("artifacts/example.txt");

        write_text_artifact(&path, "alpha").expect("initial artifact should write");
        thread::sleep(Duration::from_millis(5));
        fs::write(&path, "external").expect("external replacement should write");
        write_text_artifact(&path, "alpha").expect("canonical artifact should be restored");

        assert_eq!(
            read_text_artifact(&path).expect("artifact should read"),
            "alpha"
        );
        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn artifact_session_reads_staged_contents_before_flush() {
        let root = unique_temp_dir("hollow-grove-artifact-session");
        let path = root.join("artifacts/example.txt");
        let mut session = ArtifactSession::new();

        session.stage_text_artifact(&path, "alpha");

        assert_eq!(
            session
                .read_text_artifact(&path)
                .expect("staged artifact should read"),
            "alpha"
        );

        session.commit().expect("session should flush");
        assert_eq!(
            read_text_artifact(&path).expect("flushed artifact should read"),
            "alpha"
        );

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn artifact_session_commit_timed_reports_each_staged_artifact() {
        let root = unique_temp_dir("hollow-grove-artifact-session-timed");
        let first_path = root.join("artifacts/one.txt");
        let second_path = root.join("artifacts/two.txt");
        let mut session = ArtifactSession::new();

        session.stage_text_artifact(&first_path, "alpha");
        session.stage_text_artifact(&second_path, "beta");

        let flush_records = session.commit_timed().expect("session should flush");

        assert_eq!(flush_records.len(), 2);
        assert!(flush_records.iter().any(|record| record.path == first_path));
        assert!(
            flush_records
                .iter()
                .any(|record| record.path == second_path)
        );

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn artifact_session_skips_staging_cached_identical_contents() {
        let root = unique_temp_dir("hollow-grove-artifact-session-skip");
        let path = root.join("artifacts/example.txt");

        write_text_artifact(&path, "alpha").expect("initial artifact should write");

        let mut session = ArtifactSession::new();
        session.stage_text_artifact(&path, "alpha");

        let flush_records = session.commit_timed().expect("session should flush");
        assert!(flush_records.is_empty());

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }
}
