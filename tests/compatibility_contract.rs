use std::fs;
use std::path::{Path, PathBuf};

use hollow_grove::gameplay::{
    GAMEPLAY_ARCHIVE_SCHEMA_VERSION, GameplayArchiveError, decode_gameplay_archive_with_metadata,
    migrate_gameplay_archive,
};
use hollow_grove::runtime_federation::{
    RUNTIME_FEDERATION_ARCHIVE_VERSION, decode_runtime_federation_archive,
    encode_runtime_federation_archive,
};
use hollow_grove::world::spatial_inheritance::{
    HOLLOW_GROVE_SPATIAL_ARCHIVE_VERSION, decode_hollow_grove_spatial_archive,
    encode_hollow_grove_spatial_archive,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CompatibilityManifest {
    manifest_version: u16,
    policy: String,
    artifacts: Vec<CompatibilityEntry>,
}

#[derive(Debug, Deserialize)]
struct CompatibilityEntry {
    id: String,
    path: String,
    format: String,
    version: String,
    support: String,
    byte_length: usize,
    digest: String,
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn fixture(path: impl AsRef<Path>) -> Vec<u8> {
    fs::read(repository_root().join(path)).expect("compatibility fixture should read")
}

fn fixture_text(path: impl AsRef<Path>) -> String {
    String::from_utf8(fixture(path)).expect("compatibility fixture should be UTF-8")
}

fn fnv1a64(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}

#[test]
fn manifest_freezes_every_declared_fixture_by_size_and_digest() {
    let manifest: CompatibilityManifest =
        serde_json::from_slice(&fixture("COMPATIBILITY_MANIFEST.json"))
            .expect("compatibility manifest should parse");
    assert_eq!(manifest.manifest_version, 1);
    assert!(
        manifest
            .policy
            .contains("unknown future versions fail closed")
    );
    assert_eq!(manifest.artifacts.len(), 11);

    for entry in &manifest.artifacts {
        let bytes = fixture(&entry.path);
        assert_eq!(
            bytes.len(),
            entry.byte_length,
            "{} byte length changed",
            entry.id
        );
        assert_eq!(
            fnv1a64(&bytes),
            entry.digest,
            "{} frozen bytes changed",
            entry.id
        );
        assert!(!entry.format.is_empty());
        assert!(!entry.version.is_empty());
        assert!(!entry.support.is_empty());
    }
}

#[test]
fn gameplay_v1_v2_and_v3_fixtures_decode_and_migrate_to_the_current_schema() {
    for (path, expected_source) in [
        ("tests/fixtures/compatibility/gameplay_v1.json", 1),
        ("tests/fixtures/compatibility/gameplay_v2.json", 2),
        ("tests/fixtures/compatibility/gameplay_v3.json", 3),
    ] {
        let archive = fixture_text(path);
        let decoded = decode_gameplay_archive_with_metadata(&archive)
            .expect("gameplay fixture should decode");
        assert_eq!(decoded.source_schema_version, expected_source);

        let migrated = migrate_gameplay_archive(&archive).expect("gameplay fixture should migrate");
        let migrated_decoded = decode_gameplay_archive_with_metadata(&migrated)
            .expect("migrated gameplay fixture should decode");
        assert_eq!(
            migrated_decoded.source_schema_version,
            GAMEPLAY_ARCHIVE_SCHEMA_VERSION
        );
        assert_eq!(
            migrate_gameplay_archive(&migrated).expect("migration should be idempotent"),
            migrated
        );
    }
}

#[test]
fn runtime_federation_v1_fixture_is_an_immutable_round_trip() {
    let archive = fixture("tests/fixtures/compatibility/runtime_federation_v1.hgrf.json");
    let (payload, runtime) =
        decode_runtime_federation_archive(&archive).expect("HGRF fixture should decode");
    assert_eq!(
        runtime.manifest().archive_version,
        RUNTIME_FEDERATION_ARCHIVE_VERSION
    );
    assert_eq!(
        encode_runtime_federation_archive(&payload).expect("HGRF fixture should re-encode"),
        archive
    );
}

#[test]
fn spatial_v1_fixture_is_an_immutable_round_trip() {
    let archive = fixture("tests/fixtures/compatibility/spatial_inheritance_v1.json");
    let map = decode_hollow_grove_spatial_archive(&archive).expect("spatial fixture should decode");
    assert_eq!(
        encode_hollow_grove_spatial_archive(&map).expect("spatial fixture should re-encode"),
        archive
    );
}

#[test]
fn unknown_future_archive_versions_fail_closed() {
    let gameplay = fixture_text("tests/fixtures/compatibility/gameplay_v3.json");
    let mut gameplay_value: serde_json::Value =
        serde_json::from_str(&gameplay).expect("gameplay fixture should parse");
    gameplay_value["schema_version"] = serde_json::json!(GAMEPLAY_ARCHIVE_SCHEMA_VERSION + 1);
    assert!(matches!(
        decode_gameplay_archive_with_metadata(
            &serde_json::to_string(&gameplay_value).expect("future gameplay should serialize")
        ),
        Err(GameplayArchiveError::UnsupportedSchema(version))
            if version == GAMEPLAY_ARCHIVE_SCHEMA_VERSION + 1
    ));

    let hgrf = fixture("tests/fixtures/compatibility/runtime_federation_v1.hgrf.json");
    let mut hgrf_value: serde_json::Value =
        serde_json::from_slice(&hgrf).expect("HGRF fixture should parse");
    hgrf_value["archive_version"] = serde_json::json!(RUNTIME_FEDERATION_ARCHIVE_VERSION + 1);
    assert!(
        decode_runtime_federation_archive(
            &serde_json::to_vec(&hgrf_value).expect("future HGRF should serialize")
        )
        .is_err()
    );

    let spatial = fixture("tests/fixtures/compatibility/spatial_inheritance_v1.json");
    let mut spatial_value: serde_json::Value =
        serde_json::from_slice(&spatial).expect("spatial fixture should parse");
    spatial_value["schema_version"] = serde_json::json!(HOLLOW_GROVE_SPATIAL_ARCHIVE_VERSION + 1);
    assert!(
        decode_hollow_grove_spatial_archive(
            &serde_json::to_vec(&spatial_value).expect("future spatial archive should serialize")
        )
        .is_err()
    );
}
