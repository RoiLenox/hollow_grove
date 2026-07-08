use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

struct ArtifactCache {
    contents: HashMap<PathBuf, Vec<u8>>,
    ensured_dirs: HashSet<PathBuf>,
}

thread_local! {
    static ARTIFACT_CACHE: RefCell<ArtifactCache> = RefCell::new(ArtifactCache {
            contents: HashMap::new(),
            ensured_dirs: HashSet::new(),
    });
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

        self.staged.push((path.to_path_buf(), contents));
    }

    pub fn commit(self) -> io::Result<()> {
        for (path, contents) in self.staged {
            write_text_artifact(&path, &contents)?;
        }
        Ok(())
    }

    pub fn commit_timed(self) -> io::Result<Vec<ArtifactFlushRecord>> {
        let mut flush_records = Vec::with_capacity(self.staged.len());

        for (path, contents) in self.staged {
            let started = Instant::now();
            write_text_artifact(&path, &contents)?;
            flush_records.push(ArtifactFlushRecord {
                path,
                elapsed: started.elapsed(),
            });
        }

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

pub fn write_text_artifact(path: &Path, contents: &str) -> io::Result<()> {
    let target_bytes = contents.as_bytes();

    let cache_hit = ARTIFACT_CACHE.with(|cache| {
        cache
            .borrow()
            .contents
            .get(path)
            .is_some_and(|cached_bytes| cached_bytes.as_slice() == target_bytes)
    });
    if cache_hit {
        return Ok(());
    }

    ensure_cached_parent_dir(path)?;

    match fs::metadata(path) {
        Ok(metadata) if metadata.len() == target_bytes.len() as u64 => {
            let existing_bytes = fs::read(path)?;
            if existing_bytes == target_bytes {
                ARTIFACT_CACHE.with(|cache| {
                    cache
                        .borrow_mut()
                        .contents
                        .insert(path.to_path_buf(), existing_bytes);
                });
                return Ok(());
            }
        }
        Ok(_) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }

    fs::write(path, contents)?;
    ARTIFACT_CACHE.with(|cache| {
        cache
            .borrow_mut()
            .contents
            .insert(path.to_path_buf(), target_bytes.to_vec());
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ArtifactSession, read_text_artifact, write_text_artifact};
    use std::fs;
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
}
