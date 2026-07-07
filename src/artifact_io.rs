use std::fs;
use std::io;
use std::path::Path;

pub fn read_text_artifact(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_text_artifact(path: &Path, contents: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, contents)
}
