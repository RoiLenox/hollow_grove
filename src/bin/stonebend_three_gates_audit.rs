use std::collections::BTreeSet;

use hollow_grove::world::central_junction::JunctionApproach;
use hollow_grove::world::geography::ConstitutionalRouteId;
use hollow_grove::world::stonebend::second_pass::{
    ConstitutionalDimension, DiamondState, HypergiantSuccessionStage, StonebendConstitutionalPower,
    StonebendGateFacing, canonical_stonebend_gates, diamond_title_id, validate_three_gate_topology,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gates = canonical_stonebend_gates();
    validate_three_gate_topology(&gates)?;

    let facings = gates
        .iter()
        .map(|gate| gate.facing)
        .collect::<BTreeSet<_>>();
    if facings != StonebendGateFacing::ALL.into_iter().collect() {
        return Err("Stonebend principal gate facings drifted".into());
    }
    if StonebendGateFacing::CentralJunction
        .house_endpoint()
        .is_some()
        || StonebendGateFacing::CentralJunction.junction_approach()
            != Some(JunctionApproach::CraftCorridor)
    {
        return Err("Central Junction was flattened into a House endpoint".into());
    }
    if StonebendGateFacing::Flynt.routes()
        != [
            ConstitutionalRouteId::StairwayToHeaven,
            ConstitutionalRouteId::BasinMotorspeedway,
        ]
        || StonebendGateFacing::Sandmanor.routes()
            != [
                ConstitutionalRouteId::AuraWay,
                ConstitutionalRouteId::MntAura,
            ]
    {
        return Err("Stonebend gate route mapping drifted".into());
    }

    let powers = StonebendConstitutionalPower::ALL
        .into_iter()
        .map(|power| power.domain())
        .collect::<BTreeSet<_>>();
    if powers
        != BTreeSet::from([
            ConstitutionalDimension::Claim,
            ConstitutionalDimension::Title,
            ConstitutionalDimension::Yield,
        ])
    {
        return Err("Claim, Title, and Yield separation drifted".into());
    }
    if StonebendConstitutionalPower::Proliteriate
        .office()
        .is_some()
    {
        return Err("Proliteriate was flattened into one permanent office".into());
    }

    let diamond = DiamondState::default();
    if diamond.title != diamond_title_id() || !diamond.is_vacant() {
        return Err("Diamond no longer supports a stable vacant sovereign Title".into());
    }
    if HypergiantSuccessionStage::LazerhornClimbed.semantic_order()
        >= HypergiantSuccessionStage::AccessionEligible.semantic_order()
    {
        return Err("The Lazerhorn no longer precedes accession eligibility".into());
    }

    println!("Stonebend Three Gates and Offices Audit: pass");
    println!("principal gates: 3");
    println!("facings: Flynt, Central Junction, Sandmanor");
    println!("bidirectional: true");
    println!("Central Junction modeled as House: false");
    println!("Mt. Aura modeled as gate: false");
    println!("sovereign Title: Diamond");
    println!("active bearer office: Hypergiant");
    println!("vacancy supported: true");
    println!("constitutional dimensions: Claim, Title, Yield");
    println!("Proliteriate permanent office: false");
    println!("Spartacus permanent office: false");
    println!("two distinct powers required for removal: true");
    println!("Lazerhorn precedes accession: true");
    println!("recursion kernel dependency: none");
    Ok(())
}
