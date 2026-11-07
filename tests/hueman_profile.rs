use hollow_grove::constitutional::{CausalPosition, ParticipantId, RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::*;
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::world::geography::ConstitutionalRouteId;
use hollow_grove::world::session::WorldSession;
use std::path::Path;

fn profile() -> HuemanProfile {
    serde_json::from_value(serde_json::json!({"schema_version":1,"avatar_id":"avatar.0123456789abcdef0123456789abcdef",
        "appearance":{"skin":"deep_neutral","face":"bright","hair":{"style":"long","color":"auburn"}}})).unwrap()
}
fn app() -> GameApplicationService {
    GameApplicationService::with_world_session(
        RuleSetId::new(V2_RULE_SET).unwrap(),
        WorldSession::load_or_canonical_at(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap(),
    )
}
fn execute(app: &mut GameApplicationService, command: GameplayCommand) {
    let at = app.revision().get() + 1;
    app.execute(
        GameplayEventMetadata {
            id: GameplayEventId::new(format!("event.hueman02.{at}")).unwrap(),
            causal_position: CausalPosition::new(at),
        },
        command,
    )
    .unwrap();
}
#[test]
fn introduction_requires_the_aura_way_witness_not_an_unrelated_interaction() {
    let mut app = app();
    execute(
        &mut app,
        GameplayCommand::CreateHueman { profile: profile() },
    );
    execute(
        &mut app,
        GameplayCommand::EnterMap {
            map: WorldMapId::for_route(ConstitutionalRouteId::Glausbahn),
        },
    );
    for _ in 0..10 {
        execute(
            &mut app,
            GameplayCommand::MoveHueman {
                direction: CardinalDirection::North,
            },
        );
    }
    execute(&mut app, GameplayCommand::InteractHueman);
    let resumed = GameApplicationService::from_archive(&app.encode_archive().unwrap()).unwrap();
    assert!(
        !InitiationView::from_runtime(resumed.runtime())
            .unwrap()
            .objectives[2]
            .complete
    );
    execute(
        &mut app,
        GameplayCommand::EnterMap {
            map: WorldMapId::for_route(ConstitutionalRouteId::AuraWay),
        },
    );
    for _ in 0..10 {
        execute(
            &mut app,
            GameplayCommand::MoveHueman {
                direction: CardinalDirection::North,
            },
        );
    }
    execute(&mut app, GameplayCommand::InteractHueman);
    let resumed = GameApplicationService::from_archive(&app.encode_archive().unwrap()).unwrap();
    assert!(
        InitiationView::from_runtime(resumed.runtime())
            .unwrap()
            .objectives[2]
            .complete
    );
}

#[test]
fn cosmetic_catalog_preserves_the_runtime_preset_vocabulary() {
    let catalog: serde_json::Value = serde_json::from_str(include_str!(
        "../hueman_godot/protocol/hueman_cosmetics_v1.json"
    ))
    .unwrap();
    assert_eq!(catalog["schema_version"], 1);
    assert_eq!(catalog["scope"], "hueman-cosmetic-presets");
    assert_eq!(catalog["skins"].as_array().unwrap().len(), 6);
    assert_eq!(catalog["faces"].as_array().unwrap().len(), 4);
    assert_eq!(catalog["hair_styles"].as_array().unwrap().len(), 5);
    assert_eq!(catalog["hair_colors"].as_array().unwrap().len(), 4);
    for skin in catalog["skins"].as_array().unwrap() {
        for face in catalog["faces"].as_array().unwrap() {
            for hair in catalog["hair_styles"].as_array().unwrap() {
                for color in catalog["hair_colors"].as_array().unwrap() {
                    HuemanAppearance {
                        skin: skin["id"].as_str().unwrap().into(),
                        face: face["id"].as_str().unwrap().into(),
                        hair: HuemanHair {
                            style: hair["id"].as_str().unwrap().into(),
                            color: color["id"].as_str().unwrap().into(),
                        },
                    }
                    .validate()
                    .unwrap();
                }
            }
        }
    }
    for color in catalog["skins"]
        .as_array()
        .unwrap()
        .iter()
        .chain(catalog["hair_colors"].as_array().unwrap())
        .map(|v| v["hex"].as_str().unwrap())
        .chain(
            catalog["materials"]
                .as_object()
                .unwrap()
                .values()
                .map(|v| v.as_str().unwrap()),
        )
    {
        assert_eq!(color.len(), 6);
        assert!(color.bytes().all(|b| b.is_ascii_hexdigit()));
    }
}

#[test]
fn identity_appearance_location_and_initiation_replay_together() {
    let mut app = app();
    execute(
        &mut app,
        GameplayCommand::CreateHueman { profile: profile() },
    );
    let id = app.hueman().unwrap().clone();
    assert_eq!(id.id().as_str(), profile().continuity_id());
    assert_eq!(
        app.runtime().hueman_map(),
        WorldMapId::for_route(ConstitutionalRouteId::AuraWay)
    );
    let start = GameView::from_runtime(app.runtime(), vec![]);
    assert_eq!(
        start.baseline_stats,
        Some(hollow_grove::hollow_deck::HUEMAN_STATS)
    );
    assert_eq!(start.initiation.unwrap().title, "The Initiation");
    assert_eq!(
        app.runtime().winged_gargoyle_pilot().form(),
        WingedGargoylePilotForm::Hueman
    );
    for _ in 0..10 {
        execute(
            &mut app,
            GameplayCommand::MoveHueman {
                direction: CardinalDirection::North,
            },
        );
    }
    assert_eq!(app.runtime().hueman_position().unwrap().y, 5);
    execute(&mut app, GameplayCommand::InteractHueman);
    execute(
        &mut app,
        GameplayCommand::AdvanceSurveyFrame {
            action: SurveyFrameAction::ReceiveNeed,
        },
    );
    let partial = app.encode_archive().unwrap();
    app = GameApplicationService::from_archive(&partial).unwrap();
    assert_eq!(app.runtime().hueman_profile(), Some(&profile()));
    assert_eq!(app.hueman(), Some(&id));
    let mission = GameView::from_runtime(app.runtime(), vec![])
        .initiation
        .unwrap();
    assert!(!mission.complete);
    assert!(mission.objective.contains("Recover"));
    execute(
        &mut app,
        GameplayCommand::AdvanceSurveyFrame {
            action: SurveyFrameAction::RecoverParts,
        },
    );
    assert!(
        GameView::from_runtime(app.runtime(), vec![])
            .initiation
            .unwrap()
            .complete
    );
    let current = app.encode_archive().unwrap();
    let resumed = GameApplicationService::from_archive(&current).unwrap();
    assert_eq!(resumed.encode_archive().unwrap(), current);
    assert_eq!(
        resumed.runtime().survey_frame(),
        app.runtime().survey_frame()
    );
    assert_eq!(
        resumed.runtime().hueman_position(),
        app.runtime().hueman_position()
    );
}
#[test]
fn invalid_or_duplicate_creation_changes_nothing() {
    let mut app = app();
    let before = app.encode_archive().unwrap();
    for field in ["skin", "face", "hair"] {
        let mut raw = serde_json::to_value(profile()).unwrap();
        raw["appearance"][field] = serde_json::json!("invalid");
        if let Ok(p) = serde_json::from_value::<HuemanProfile>(raw) {
            assert!(
                app.execute(
                    GameplayEventMetadata {
                        id: GameplayEventId::new("event.bad-profile").unwrap(),
                        causal_position: CausalPosition::new(1)
                    },
                    GameplayCommand::CreateHueman { profile: p }
                )
                .is_err()
            );
        }
        assert_eq!(app.encode_archive().unwrap(), before);
    }
    let mut raw = serde_json::to_value(profile()).unwrap();
    raw["appearance"]["body"] = serde_json::json!(100);
    assert!(serde_json::from_value::<HuemanProfile>(raw).is_err());
    execute(
        &mut app,
        GameplayCommand::CreateHueman { profile: profile() },
    );
    let created = app.encode_archive().unwrap();
    assert!(
        app.execute(
            GameplayEventMetadata {
                id: GameplayEventId::new("event.duplicate").unwrap(),
                causal_position: CausalPosition::new(2)
            },
            GameplayCommand::CreateHueman { profile: profile() }
        )
        .is_err()
    );
    assert_eq!(app.encode_archive().unwrap(), created);
}
#[test]
fn synthesis_and_return_preserve_profile_identity_and_progress() {
    let mut app = app();
    execute(
        &mut app,
        GameplayCommand::CreateHueman { profile: profile() },
    );
    let id = app.hueman().unwrap().clone();
    let mut forms = Vec::new();
    while let Some(action) = app.runtime().winged_gargoyle_pilot().next_action() {
        execute(
            &mut app,
            GameplayCommand::AdvanceWingedGargoylePilot { action },
        );
        forms.push(app.runtime().winged_gargoyle_pilot().form());
        let archive = app.encode_archive().unwrap();
        app = GameApplicationService::from_archive(&archive).unwrap();
        assert_eq!(app.hueman(), Some(&id));
        assert_eq!(app.runtime().hueman_profile(), Some(&profile()));
    }
    assert!(forms.contains(&WingedGargoylePilotForm::Gargoyle));
    assert!(forms.contains(&WingedGargoylePilotForm::WingedGargoyle));
    assert_eq!(
        app.runtime().winged_gargoyle_pilot().form(),
        WingedGargoylePilotForm::Hueman
    );
    assert!(
        app.runtime()
            .winged_gargoyle_pilot()
            .capability_id()
            .is_some()
    );
}
#[test]
fn v6_identity_migrates_once_without_creating_cosmetics_or_a_tutorial() {
    let mut app = app();
    let profile = profile();
    execute(
        &mut app,
        GameplayCommand::EstablishHuemanIdentity {
            continuity: BeingContinuityId::new(profile.continuity_id()).unwrap(),
            participant: ParticipantId::new(profile.participant_id()).unwrap(),
            institutional: InstitutionalBeingId::new(profile.institutional_id()).unwrap(),
        },
    );
    let mut old: serde_json::Value = serde_json::from_str(&app.encode_archive().unwrap()).unwrap();
    // V6 has this exact payload/checksum layout and contains only its supported events.
    old["schema_version"] = serde_json::json!(6);
    let old = old.to_string();
    let migrated = migrate_gameplay_archive(&old).unwrap();
    let decoded = decode_gameplay_archive_with_metadata(&migrated).unwrap();
    assert_eq!(
        decoded
            .migration_history
            .iter()
            .filter(|m| m.from_schema_version == 6 && m.to_schema_version == 7)
            .count(),
        1
    );
    assert_eq!(migrate_gameplay_archive(&migrated).unwrap(), migrated);
    let restored = GameApplicationService::from_archive(&migrated).unwrap();
    assert_eq!(restored.events(), app.events());
    assert_eq!(restored.hueman(), app.hueman());
    assert_eq!(
        restored.runtime().hueman_position(),
        app.runtime().hueman_position()
    );
    let view = GameView::from_runtime(restored.runtime(), vec![]);
    assert!(
        view.hueman_profile.is_none() && view.initiation.is_none() && view.baseline_stats.is_none()
    );
}

#[test]
fn legacy_archives_gain_no_profile_or_mission_and_v7_events_cannot_be_backdated() {
    let old = include_str!("fixtures/compatibility/gameplay_v5_sympiote.json");
    let legacy = GameApplicationService::from_archive(old).unwrap();
    assert!(legacy.runtime().hueman_profile().is_none());
    assert!(
        GameView::from_runtime(legacy.runtime(), vec![])
            .initiation
            .is_none()
    );
    let current = legacy.encode_archive().unwrap();
    let decoded = decode_gameplay_archive_with_metadata(&current).unwrap();
    assert_eq!(decoded.source_schema_version, 7);
    assert!(
        decoded
            .migration_history
            .iter()
            .any(|m| m.from_schema_version == 6 && m.to_schema_version == 7)
    );
    assert_eq!(
        legacy.events(),
        GameApplicationService::from_archive(&current)
            .unwrap()
            .events()
    );
    let mut app = app();
    execute(
        &mut app,
        GameplayCommand::CreateHueman { profile: profile() },
    );
    let mut raw: serde_json::Value = serde_json::from_str(&app.encode_archive().unwrap()).unwrap();
    // Schema is outside the checksum payload. Backdating must still be refused.
    raw["schema_version"] = serde_json::json!(6);
    assert!(
        decode_gameplay_archive_with_metadata(&raw.to_string())
            .unwrap_err()
            .to_string()
            .contains("before V7")
    );
}
#[test]
fn service_restart_loads_the_complete_character_and_preserves_backup() {
    let path = std::env::temp_dir().join(format!(
        "hueman02-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&path).unwrap();
    let new_service = || {
        GameProtocolService::new_with_save_root(
            "session.hueman02-test",
            RuleSetId::new(V2_RULE_SET).unwrap(),
            &path,
        )
        .unwrap()
    };
    let mut service = new_service();
    let send = |service: &mut GameProtocolService, id: &str, intent| {
        let response = service.handle(ProtocolRequestEnvelope {
            protocol_version: 1,
            session_id: "session.hueman02-test".into(),
            request_id: id.into(),
            expected_revision: service.revision(),
            intent,
        });
        assert_eq!(
            response.status,
            ProtocolResponseStatus::Completed,
            "{:?}",
            response.rejection
        );
        response.view.unwrap()
    };
    send(
        &mut service,
        "request.create",
        GameplayIntent::CreateHuemanIntent { profile: profile() },
    );
    send(
        &mut service,
        "request.move",
        GameplayIntent::MoveIntent {
            direction: CardinalDirection::North,
        },
    );
    let saved = send(
        &mut service,
        "request.save",
        GameplayIntent::SaveIntent {
            slot_id: "slot-a".into(),
        },
    );
    assert!(saved.character_save_available);
    drop(service);
    let mut service = new_service();
    let loaded = send(
        &mut service,
        "request.load",
        GameplayIntent::LoadIntent {
            slot_id: "slot-a".into(),
        },
    );
    assert_eq!(loaded.hueman, saved.hueman);
    assert_eq!(loaded.hueman_profile, saved.hueman_profile);
    assert_eq!(loaded.overworld, saved.overworld);
    assert_eq!(loaded.initiation, saved.initiation);
    send(
        &mut service,
        "request.save-again",
        GameplayIntent::SaveIntent {
            slot_id: "slot-a".into(),
        },
    );
    assert!(
        std::fs::read_dir(&path).unwrap().any(|e| e
            .unwrap()
            .file_name()
            .to_string_lossy()
            .contains("backup"))
    );
    drop(service);
    std::fs::remove_dir_all(path).unwrap();
}
