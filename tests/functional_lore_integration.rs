use std::collections::BTreeSet;
use std::path::Path;

use hollow_grove::constitutional::{CausalPosition, HouseFunction};
use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::geography::ConstitutionalRouteId;
use hollow_grove::world::lived_lore::{
    FunctionalLoreCatalog, FunctionalLoreError, canonical_functional_lore_definitions,
    validate_definitions,
};
use hollow_grove::world::session::WorldSession;

fn live_world() -> WorldSession {
    WorldSession::load_or_canonical_at(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap()
}

#[test]
fn all_four_houses_have_three_complete_lived_loops_and_every_route_is_used() {
    let definitions = canonical_functional_lore_definitions();
    validate_definitions(&definitions).unwrap();
    assert_eq!(definitions.len(), 12);
    for house in [
        House::Flynt,
        House::Stonebend,
        House::Sandmanor,
        House::Glaushouse,
    ] {
        let records = definitions
            .iter()
            .filter(|definition| definition.house == house)
            .collect::<Vec<_>>();
        assert_eq!(records.len(), 3, "{house:?}");
        for record in records {
            assert!(!record.entities.is_empty());
            assert!(!record.evidence.is_empty());
            assert!(!record.uncertainty.is_empty());
            assert!(record.player_choices.len() >= 2);
            assert_eq!(
                record.player_choices.len(),
                record.lawful_state_changes.len()
            );
            assert!(!record.persistence_and_replay.is_empty());
            assert!(!record.presentation.is_empty());
            assert!(!record.failure_and_refusal.is_empty());
        }
    }
    let routes = definitions
        .iter()
        .map(|definition| definition.route)
        .collect::<BTreeSet<_>>();
    assert_eq!(routes, ConstitutionalRouteId::ALL.into_iter().collect());
}

#[test]
fn lived_lore_uses_live_non_fixture_authority_without_house_substitution() {
    let catalog = FunctionalLoreCatalog::instantiate(&live_world(), CausalPosition::new(40))
        .expect("live functional lore");
    for record in catalog.records() {
        assert_eq!(
            record.authority.function,
            record.definition.authority_function
        );
        assert_eq!(record.authority.authority.house, record.definition.house);
        assert!(
            record
                .authority
                .authority
                .grants(record.definition.authority_class)
        );
        assert!(
            !record
                .authority
                .authority
                .actor
                .as_str()
                .contains("fixture")
        );
        let expected = match record.definition.house {
            House::Stonebend => HouseFunction::Name,
            House::Sandmanor => HouseFunction::Prove,
            House::Glaushouse => HouseFunction::Clear,
            House::Flynt => HouseFunction::Recognize,
        };
        assert_eq!(record.definition.authority_function, expected);
    }
}

#[test]
fn functional_lore_archive_replays_exactly_and_rejects_drift() {
    let world = live_world();
    let catalog = FunctionalLoreCatalog::instantiate(&world, CausalPosition::new(41)).unwrap();
    let encoded = catalog.encode().unwrap();
    assert!(encoded.contains("\"checksum\": \"fnv1a64:"));
    assert!(encoded.contains("lore.stonebend.mnt-aura-illegal-hollowing"));
    assert!(encoded.contains("lore.sandmanor.glausbahn-recovery-design"));
    assert!(encoded.contains("lore.glaushouse.riptide-emergency-intake"));
    assert!(encoded.contains("lore.flynt.boardwalk-return-recognition"));

    let replayed = FunctionalLoreCatalog::replay(&encoded, &world).unwrap();
    assert_eq!(replayed, catalog);

    let damaged = encoded.replacen(
        "public return lane",
        "unconstitutionally closed return lane",
        1,
    );
    assert_eq!(
        FunctionalLoreCatalog::replay(&damaged, &world),
        Err(FunctionalLoreError::ChecksumMismatch)
    );
}

#[test]
fn missing_current_office_holders_fail_closed_and_kernel_imports_stay_absent() {
    let error =
        FunctionalLoreCatalog::instantiate(&WorldSession::canonical(), CausalPosition::new(1))
            .unwrap_err();
    assert!(matches!(error, FunctionalLoreError::Authority { .. }));

    let source = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/world/lived_lore.rs"),
    )
    .unwrap();
    for forbidden in [
        "crate::kernel",
        "crate::recursion",
        "CurrentSynthesisEngine",
        "execute_recursion",
    ] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn functional_lore_document_tracks_the_executable_contract_and_every_loop() {
    let document = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("HOLLOW_GROVE_FUNCTIONAL_LORE_INTEGRATION_V1.md"),
    )
    .unwrap();
    assert!(document.contains("HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md"));
    for field in [
        "stable identity",
        "authority class",
        "route, location, and jurisdiction",
        "involved entities",
        "dominant House verb",
        "trigger",
        "evidence and explicit uncertainty",
        "player-visible choices",
        "lawful state change",
        "persistence and replay",
        "presentation",
        "failure and refusal",
    ] {
        assert!(document.contains(field), "{field}");
    }
    for definition in canonical_functional_lore_definitions() {
        assert!(document.contains(definition.id), "{}", definition.id);
    }
}
