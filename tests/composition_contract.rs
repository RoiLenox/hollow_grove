use hollow_grove::{
    CompositionNodeId, CompositionRecordId, FrameId,
    lineage_contract::validate_sandmanor_transition,
    world::composition_witnesses::canonical_composition_witness_catalog,
};

#[test]
fn four_witnesses_share_one_neutral_recursive_catalog() {
    let catalog = canonical_composition_witness_catalog().expect("canonical witness catalog");

    let landed_point = CompositionNodeId::new("node.point.hueman.landed-2").unwrap();
    let stabilized_point = CompositionNodeId::new("node.point.hueman.level-2").unwrap();
    assert_eq!(catalog.records_producing_result(&landed_point).len(), 1);
    assert_eq!(catalog.records_using_source(&landed_point).len(), 1);
    assert_eq!(catalog.records_producing_result(&stabilized_point).len(), 1);

    let being = CompositionNodeId::new("node.being.hueman.sandmanor-witness").unwrap();
    assert_eq!(catalog.records_producing_result(&being).len(), 2);
    assert_eq!(
        catalog.direct_containers(&being)[0].id.as_str(),
        "node.house.sandmanor"
    );
    assert!(validate_sandmanor_transition(FrameId::Gnome, FrameId::Pegasus).is_err());

    let house = CompositionNodeId::new("node.house.sandmanor").unwrap();
    assert_eq!(catalog.records_using_source(&house).len(), 1);
    assert!(catalog.direct_members(&house).len() >= 5);

    let artifact = CompositionNodeId::new("node.artifact.hueman.boundary").unwrap();
    assert_eq!(catalog.records_producing_result(&artifact).len(), 1);
    assert_eq!(catalog.records_using_source(&artifact).len(), 1);
    assert_eq!(
        catalog
            .record(&CompositionRecordId::new("record.artifact.hueman-boundary").unwrap())
            .unwrap()
            .sources
            .len(),
        2
    );
}
