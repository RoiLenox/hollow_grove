use std::fs;
use std::path::Path;

use flynt_constitution::{
    FORM_MANTICORP, INSTITUTION_GALLOWS, RECIPE_MANTICORP, SITE_GALLOWRY, canonical_constitution,
    canonical_hierarchy_rows,
};

const CONSTITUTION_DOCUMENT: &str = include_str!("../FLYNT_CONSTITUTION_V2.md");
const HUEMAN_PROJECTION: &str = include_str!("../artifacts/hueman_flynt_constitution.md");

#[test]
fn canonical_document_names_every_authority_node_and_stable_id() {
    let constitution = canonical_constitution().unwrap();
    for node in constitution.nodes() {
        assert!(
            CONSTITUTION_DOCUMENT.contains(node.id.as_str()),
            "Flynt constitution omits stable ID {}",
            node.id
        );
        assert!(
            CONSTITUTION_DOCUMENT.contains(&node.name),
            "Flynt constitution omits authority name {}",
            node.name
        );
    }
    assert!(CONSTITUTION_DOCUMENT.contains(SITE_GALLOWRY));
    assert!(CONSTITUTION_DOCUMENT.contains(INSTITUTION_GALLOWS));
    assert!(CONSTITUTION_DOCUMENT.contains(FORM_MANTICORP));
    assert!(CONSTITUTION_DOCUMENT.contains(RECIPE_MANTICORP));
}

#[test]
fn public_projection_contains_both_exact_main_hierarchies() {
    assert!(HUEMAN_PROJECTION.contains("Tross -> Manticorp Institution"));
    assert!(HUEMAN_PROJECTION.contains("The public chain is Tross -> Manticorp Institution"));
    assert!(
        HUEMAN_PROJECTION.contains("The underground chain is Mystery Man/Mr. X -> The Gallows")
    );
    assert!(HUEMAN_PROJECTION.contains("There is exactly one constitutional Chimera"));
    assert!(HUEMAN_PROJECTION.contains("The Gallowry is the hidden headquarters"));
    assert!(HUEMAN_PROJECTION.contains("Tross = Mystery Man = Mr. X"));
}

#[test]
fn every_executable_hierarchy_row_is_documented() {
    let constitution = canonical_constitution().unwrap();
    for (node, superior) in canonical_hierarchy_rows() {
        let node_name = &constitution.node_by_key(node).unwrap().name;
        let superior_name = &constitution.node_by_key(superior).unwrap().name;
        assert!(CONSTITUTION_DOCUMENT.contains(node_name));
        assert!(CONSTITUTION_DOCUMENT.contains(superior_name));
    }
}

#[test]
fn repository_text_has_no_obsolete_flynt_authority_vocabulary() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let prohibited = [
        concat!("Mystery", "guard"),
        concat!("Manticorp", "s"),
        concat!("Manti", "core"),
        concat!("Chimera", "Refinement"),
        concat!("Executive", "Mastery"),
        concat!("Lawful", "Accession"),
        concat!("Hueman Tross", " Helpers"),
        concat!("White", " Dwarfs"),
        concat!("institution.flynt.", "gallowry"),
        concat!("role.flynt.", "gallowry"),
    ];
    scan_text_tree(root, root, &prohibited);
    let retired_artifact = format!("artifacts/hueman_{}_{}.md", "tross", "helpers");
    assert!(!root.join(retired_artifact).exists());
}

fn scan_text_tree(root: &Path, path: &Path, prohibited: &[&str]) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let relative = path.strip_prefix(root).unwrap();
        if path.is_dir() {
            let should_skip = path.components().any(|component| {
                matches!(
                    component.as_os_str().to_str(),
                    Some(".git" | "target" | ".godot")
                )
            });
            if should_skip {
                continue;
            }
            scan_text_tree(root, &path, prohibited);
            continue;
        }
        let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
            continue;
        };
        if !matches!(extension, "rs" | "md" | "json" | "toml" | "gd" | "txt") {
            continue;
        }
        let Ok(contents) = fs::read_to_string(&path) else {
            continue;
        };
        for term in prohibited {
            assert!(
                !contents.contains(term),
                "{} contains obsolete Flynt authority vocabulary {term}",
                relative.display()
            );
        }
    }
}
