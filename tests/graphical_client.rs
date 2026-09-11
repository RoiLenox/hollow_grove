use std::collections::BTreeSet;

use hollow_grove::House;
use hollow_grove::client::assets::{
    ATLAS_HEIGHT, ATLAS_WIDTH, SPRITE_FRAME_SIZE, SpriteKey, build_prototype_atlas, resolve_region,
};
use hollow_grove::client::input::direction_for_key;
use hollow_grove::client::model::{
    ActorBodyIdentity, ActorState, ActorVisualIdentity, ClientWorldState, LOGICAL_HEIGHT,
    LOGICAL_WIDTH, MoveOutcome, MovementState, TestWorld, WalkFrame,
};
use hollow_grove::client::presentation::{
    ActorPresentationState, Camera2d, ScenePresentation, logical_viewport,
};
use hollow_grove::gameplay::{CardinalDirection, TilePosition};
use hollow_grove::hollow_deck::{HUEMAN_STATS, StatBlock};
use winit::keyboard::KeyCode;

#[test]
fn hueman_identity_resolves_semantically_without_a_filename() {
    let actor = ActorState::hueman(TestWorld::SPAWN);
    let resolved = ActorPresentationState::resolve(&actor).unwrap();
    assert_eq!(resolved.sprite_key, SpriteKey::HuemanOverworldPrototype);
    assert_eq!(resolved.visual_width, 16);
    assert_eq!(resolved.visual_height, 16);
    assert_eq!(HUEMAN_STATS, StatBlock::new(50, 50, 50, 50));

    let layered = ActorVisualIdentity {
        body: ActorBodyIdentity::Nature {
            nature_id: "gargoyle".into(),
        },
        house: Some(House::Flynt),
        culture_id: None,
        current_form_id: Some("wings".into()),
        aura_set_id: Some("gargoyle".into()),
        institution_id: Some("mystery-men".into()),
        status_id: Some("mystery-man".into()),
        sect_or_order_id: Some("hunter".into()),
        organization_id: Some("the-gallows".into()),
        alias_id: Some("johnny-apple".into()),
    };
    assert!(matches!(layered.body, ActorBodyIdentity::Nature { .. }));
    assert_eq!(layered.organization_id.as_deref(), Some("the-gallows"));
    assert_eq!(layered.alias_id.as_deref(), Some("johnny-apple"));
}

#[test]
fn deterministic_movement_updates_game_state_and_honors_collision() {
    let mut state = ClientWorldState::new();
    let start = state.hueman.position;
    assert_eq!(
        state.move_hueman(CardinalDirection::North),
        MoveOutcome::Accepted
    );
    assert_eq!(
        state.hueman.position,
        TilePosition {
            x: start.x,
            y: start.y - 1,
            facing: CardinalDirection::North,
        }
    );
    assert_eq!(state.hueman.movement, MovementState::Walking);

    state.hueman = ActorState::hueman(TilePosition {
        x: 18,
        y: 15,
        facing: CardinalDirection::South,
    });
    assert_eq!(
        state.move_hueman(CardinalDirection::North),
        MoveOutcome::Blocked
    );
    assert_eq!(
        state.hueman.position,
        TilePosition {
            x: 18,
            y: 15,
            facing: CardinalDirection::North,
        }
    );
    assert_eq!(state.hueman.movement, MovementState::Idle);
}

#[test]
fn animation_uses_idle_walk_a_walk_b_and_central_step_completion() {
    let mut state = ClientWorldState::new();
    assert_eq!(state.hueman.movement, MovementState::Idle);
    assert_eq!(state.hueman.walk_frame, WalkFrame::A);
    state.move_hueman(CardinalDirection::North);
    assert_eq!(state.hueman.walk_frame, WalkFrame::A);
    state.finish_hueman_step();
    assert_eq!(state.hueman.movement, MovementState::Idle);
    state.move_hueman(CardinalDirection::North);
    assert_eq!(state.hueman.walk_frame, WalkFrame::B);

    let idle = resolve_region(
        SpriteKey::HuemanOverworldPrototype,
        CardinalDirection::North,
        MovementState::Idle,
        WalkFrame::A,
    );
    let walk_a = resolve_region(
        SpriteKey::HuemanOverworldPrototype,
        CardinalDirection::North,
        MovementState::Walking,
        WalkFrame::A,
    );
    let walk_b = resolve_region(
        SpriteKey::HuemanOverworldPrototype,
        CardinalDirection::North,
        MovementState::Walking,
        WalkFrame::B,
    );
    assert_ne!(idle, walk_a);
    assert_ne!(walk_a, walk_b);
}

#[test]
fn camera_and_logical_viewport_preserve_world_and_aspect_separation() {
    let state = ClientWorldState::new();
    let camera = Camera2d::following(
        &state.hueman,
        state.world.width_pixels(),
        state.world.height_pixels(),
    );
    assert!(camera.origin_x > 0);
    assert!(camera.origin_y > 0);
    let (screen_x, screen_y) = camera.world_to_screen(
        i32::from(state.hueman.position.x) * 16,
        i32::from(state.hueman.position.y) * 16,
    );
    assert!((0..LOGICAL_WIDTH as i32).contains(&screen_x));
    assert!((0..LOGICAL_HEIGHT as i32).contains(&screen_y));

    let exact = logical_viewport(1280, 720);
    assert_eq!(exact.scale, 2.0);
    assert!(exact.integer_scaled);
    assert_eq!(
        (exact.x, exact.y, exact.width, exact.height),
        (0.0, 0.0, 1280.0, 720.0)
    );

    let letterboxed = logical_viewport(1000, 700);
    assert_eq!(letterboxed.scale, 1.0);
    assert_eq!(letterboxed.width, 640.0);
    assert_eq!(letterboxed.height, 360.0);
    assert_eq!(letterboxed.x, 180.0);
    assert_eq!(letterboxed.y, 170.0);
}

#[test]
fn prototype_atlas_is_rgba8_transparent_limited_and_scene_ready() {
    let atlas = build_prototype_atlas();
    assert_eq!((atlas.width, atlas.height), (ATLAS_WIDTH, ATLAS_HEIGHT));
    assert_eq!(atlas.rgba8.len(), (ATLAS_WIDTH * ATLAS_HEIGHT * 4) as usize);

    let mut colors: BTreeSet<[u8; 4]> = BTreeSet::new();
    for y in 0..SPRITE_FRAME_SIZE {
        for x in 0..SPRITE_FRAME_SIZE {
            let index = ((y * ATLAS_WIDTH + x) * 4) as usize;
            colors.insert(atlas.rgba8[index..index + 4].try_into().unwrap());
        }
    }
    assert!(colors.contains(&[0, 0, 0, 0]));
    assert!(colors.iter().filter(|color| color[3] != 0).count() <= 5);

    let scene = ScenePresentation::resolve(&ClientWorldState::new()).unwrap();
    assert!(scene.sprites.len() > 2);
    assert_eq!(
        scene.sprites.last().unwrap().key,
        SpriteKey::HuemanOverworldPrototype
    );
    assert_eq!(scene.sprites.last().unwrap().order, 1);
}

#[test]
fn keyboard_mapping_supports_arrows_and_wasd_without_a_window() {
    assert_eq!(
        direction_for_key(KeyCode::ArrowUp),
        Some(CardinalDirection::North)
    );
    assert_eq!(
        direction_for_key(KeyCode::KeyD),
        Some(CardinalDirection::East)
    );
    assert_eq!(
        direction_for_key(KeyCode::KeyS),
        Some(CardinalDirection::South)
    );
    assert_eq!(
        direction_for_key(KeyCode::KeyA),
        Some(CardinalDirection::West)
    );
    assert_eq!(direction_for_key(KeyCode::Space), None);
}
