//! Locate authored inspection inputs without embedding a developer build path.

use std::path::{Path, PathBuf};

fn is_source_root(path: &Path) -> bool {
    path.join("Cargo.toml").is_file()
        && path
            .join("HOLLOW_GROVE_EMBODIED_WORLD_FOUNDATION_V1.md")
            .is_file()
}

fn locate(executable: &Path, working_directory: &Path) -> PathBuf {
    for ancestor in executable.parent().into_iter().flat_map(Path::ancestors) {
        if is_source_root(ancestor) {
            return ancestor.to_path_buf();
        }
        let installed = ancestor.join("share/hollow-grove/source");
        if is_source_root(&installed) {
            return installed;
        }
    }
    for ancestor in working_directory.ancestors() {
        if is_source_root(ancestor) {
            return ancestor.to_path_buf();
        }
    }
    PathBuf::from("/usr/share/hollow-grove/source")
}

pub fn default_source_root() -> PathBuf {
    locate(
        &std::env::current_exe().unwrap_or_default(),
        &std::env::current_dir().unwrap_or_default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn relocated_install_uses_its_inventory_before_a_working_checkout() {
        let temp = std::env::temp_dir().join(format!("hg-source-root-{}", std::process::id()));
        let installed = temp.join("relocated/usr/share/hollow-grove/source");
        let checkout = temp.join("checkout");
        for root in [&installed, &checkout] {
            fs::create_dir_all(root).unwrap();
            fs::write(root.join("Cargo.toml"), "").unwrap();
            fs::write(
                root.join("HOLLOW_GROVE_EMBODIED_WORLD_FOUNDATION_V1.md"),
                "",
            )
            .unwrap();
        }
        assert_eq!(
            locate(
                &temp.join("relocated/usr/lib/hollow-grove/cli/hollow-grove"),
                &checkout
            ),
            installed
        );
        assert_eq!(
            locate(&checkout.join("target/debug/hollow-grove"), &temp),
            checkout
        );
        assert_eq!(
            locate(&temp.join("external-target/hollow-grove"), &checkout),
            checkout
        );
        fs::remove_dir_all(&temp).unwrap();
    }

    #[test]
    fn missing_source_uses_the_installed_inventory_path() {
        assert_eq!(
            locate(Path::new("/unrelated/bin/hollow-grove"), Path::new("/")),
            PathBuf::from("/usr/share/hollow-grove/source")
        );
    }
}
