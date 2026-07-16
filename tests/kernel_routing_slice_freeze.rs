use hollow_grove::{
    BeamRoute, ExteriorShape, KernelInput, LandingRoute, Mode, PlebMetaInput, SeamRoute, Symptom,
    build_snapshot_output, build_tree_output, run_kernel_cycle, run_kernel_cycle_with_input,
};

const STRAIGHT_CANONICAL_WITNESS: &str = "Point\n\
↓\n\
Triway\n\
↓\n\
Fourway\n\
↓\n\
HollowGrove\n\
↓\n\
CurrentSeam [PlebExterior]\n\
↓\n\
AuraBeam [BlepReturn]\n\
↓\n\
Point² (Landed Point) [BlepArrival]";

const CURVED_CANONICAL_WITNESS: &str = "Point\n\
↓\n\
Triway\n\
↓\n\
Fourway\n\
↓\n\
HollowGrove\n\
↓\n\
CurrentSeam [MetaExterior]\n\
↓\n\
AuraBeam [AtemReturn]\n\
↓\n\
Point² (Landed Point) [AtemArrival]";

const STRAIGHT_TREE_WITNESS: &str = "KernelPass\n\
├─ start: Point\n\
├─ triway\n\
│  ├─ ways: [One, Two, Three]\n\
├─ fourway: Fourway\n\
├─ hollow_grove\n\
│  ├─ bond: One\n\
│  └─ atmosphere: [Two, Three]\n\
├─ current_seam: CurrentSeam [PlebExterior]\n\
├─ aura_beam: AuraBeam [BlepReturn]\n\
└─ point_squared: Point² (Landed Point) [BlepArrival]";

const CURVED_TREE_WITNESS: &str = "KernelPass\n\
├─ start: Point\n\
├─ triway\n\
│  ├─ ways: [One, Two, Three]\n\
├─ fourway: Fourway\n\
├─ hollow_grove\n\
│  ├─ bond: One\n\
│  └─ atmosphere: [Two, Three]\n\
├─ current_seam: CurrentSeam [MetaExterior]\n\
├─ aura_beam: AuraBeam [AtemReturn]\n\
└─ point_squared: Point² (Landed Point) [AtemArrival]";

const STRAIGHT_SNAPSHOT: &str = "{\n\
                     \x20\x20\"start\": \"Point\",\n\
                     \x20\x20\"triway\": {\n\
                     \x20\x20\x20\x20\"ways\": [\"One\", \"Two\", \"Three\"]\n\
                     \x20\x20},\n\
                     \x20\x20\"fourway\": \"Fourway\",\n\
                     \x20\x20\"hollow_grove\": {\n\
                     \x20\x20\x20\x20\"bond\": \"One\",\n\
                     \x20\x20\x20\x20\"atmosphere\": [\"Two\", \"Three\"]\n\
                     \x20\x20},\n\
                     \x20\x20\"grove_seam\": \"CurrentSeam\",\n\
                     \x20\x20\"grove_seam_route\": \"PlebExterior\",\n\
                     \x20\x20\"hollow_beam\": \"AuraBeam\",\n\
                     \x20\x20\"hollow_beam_route\": \"BlepReturn\",\n\
                     \x20\x20\"landed\": \"Landed Point\",\n\
                     \x20\x20\"landing_route\": \"BlepArrival\",\n\
                     \x20\x20\"landed_point\": \"Point²\",\n\
                     \x20\x20\"canonical_witness\": \"Point\\n↓\\nTriway\\n↓\\nFourway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam [PlebExterior]\\n↓\\nAuraBeam [BlepReturn]\\n↓\\nPoint² (Landed Point) [BlepArrival]\"\n\
}";

const CURVED_SNAPSHOT: &str = "{\n\
                   \x20\x20\"start\": \"Point\",\n\
                   \x20\x20\"triway\": {\n\
                   \x20\x20\x20\x20\"ways\": [\"One\", \"Two\", \"Three\"]\n\
                   \x20\x20},\n\
                   \x20\x20\"fourway\": \"Fourway\",\n\
                   \x20\x20\"hollow_grove\": {\n\
                   \x20\x20\x20\x20\"bond\": \"One\",\n\
                   \x20\x20\x20\x20\"atmosphere\": [\"Two\", \"Three\"]\n\
                   \x20\x20},\n\
                   \x20\x20\"grove_seam\": \"CurrentSeam\",\n\
                   \x20\x20\"grove_seam_route\": \"MetaExterior\",\n\
                   \x20\x20\"hollow_beam\": \"AuraBeam\",\n\
                   \x20\x20\"hollow_beam_route\": \"AtemReturn\",\n\
                   \x20\x20\"landed\": \"Landed Point\",\n\
                   \x20\x20\"landing_route\": \"AtemArrival\",\n\
                   \x20\x20\"landed_point\": \"Point²\",\n\
                   \x20\x20\"canonical_witness\": \"Point\\n↓\\nTriway\\n↓\\nFourway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam [MetaExterior]\\n↓\\nAuraBeam [AtemReturn]\\n↓\\nPoint² (Landed Point) [AtemArrival]\"\n\
}";

fn curved_input() -> KernelInput {
    KernelInput {
        routing: PlebMetaInput {
            exterior_shape: ExteriorShape::Curved,
            pleb_mode: Mode::Pathos,
            meta_mode: Mode::Logos,
        },
    }
}

#[test]
fn straight_single_pass_boundary_is_frozen() {
    let kernel_pass = run_kernel_cycle(Symptom::origin());

    assert_eq!(kernel_pass.to_string(), STRAIGHT_CANONICAL_WITNESS);
    assert_eq!(build_snapshot_output(&kernel_pass), STRAIGHT_SNAPSHOT);
    assert_eq!(build_tree_output(&kernel_pass), STRAIGHT_TREE_WITNESS);

    assert_eq!(kernel_pass.grove_seam().route(), SeamRoute::PlebExterior);
    assert_eq!(
        kernel_pass.hollow_beam().seam_route(),
        SeamRoute::PlebExterior
    );
    assert_eq!(kernel_pass.hollow_beam().route(), BeamRoute::BlepReturn);
    assert_eq!(kernel_pass.landed().route(), LandingRoute::BlepArrival);
    assert_eq!(format!("{:?}", kernel_pass.landed().point()), "Point");
    assert_eq!(format!("{:?}", kernel_pass.end_point()), "Point");
}

#[test]
fn curved_single_pass_boundary_is_frozen() {
    let kernel_pass = run_kernel_cycle_with_input(Symptom::origin(), curved_input());

    assert_eq!(kernel_pass.to_string(), CURVED_CANONICAL_WITNESS);
    assert_eq!(build_snapshot_output(&kernel_pass), CURVED_SNAPSHOT);
    assert_eq!(build_tree_output(&kernel_pass), CURVED_TREE_WITNESS);

    assert_eq!(kernel_pass.grove_seam().route(), SeamRoute::MetaExterior);
    assert_eq!(
        kernel_pass.hollow_beam().seam_route(),
        SeamRoute::MetaExterior
    );
    assert_eq!(kernel_pass.hollow_beam().route(), BeamRoute::AtemReturn);
    assert_eq!(kernel_pass.landed().route(), LandingRoute::AtemArrival);
    assert_eq!(format!("{:?}", kernel_pass.landed().point()), "Point");
    assert_eq!(format!("{:?}", kernel_pass.end_point()), "Point");
}

#[test]
fn frozen_boundary_outputs_are_byte_stable() {
    let straight = run_kernel_cycle(Symptom::origin());
    let curved = run_kernel_cycle_with_input(Symptom::origin(), curved_input());

    assert_eq!(straight.to_string(), straight.to_string());
    assert_eq!(curved.to_string(), curved.to_string());
    assert_eq!(
        build_snapshot_output(&straight),
        build_snapshot_output(&straight)
    );
    assert_eq!(
        build_snapshot_output(&curved),
        build_snapshot_output(&curved)
    );
    assert_eq!(build_tree_output(&straight), build_tree_output(&straight));
    assert_eq!(build_tree_output(&curved), build_tree_output(&curved));
}
