use std::collections::BTreeSet;

use hollow_grove::world::central_junction::{
    self, CentralJunctionFunction, CentralJunctionInstitution, EconomicPole, EventOutcome,
    HouseSectorHall, JunctionApproach, MarketLifecycleState, SummitConcept, ValueInstrument,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let district = central_junction::canonical_central_junction();
    if !district.district_not_single_building
        || district.formal_name != "CENTRAL JUNCTION"
        || district.institutions != CentralJunctionInstitution::ALL.into_iter().collect()
        || district.approaches != JunctionApproach::ALL.into_iter().collect()
        || district.sector_halls != HouseSectorHall::ALL.into_iter().collect()
        || district.public_functions != CentralJunctionFunction::ALL.into_iter().collect()
        || !CentralJunctionFunction::ServiceTournament.is_largest_public_function()
    {
        return Err("Central Junction district roster drifted".into());
    }
    if EconomicPole::ALL.into_iter().collect::<BTreeSet<_>>().len() != 4 {
        return Err("Four-Pole economic matrix drifted".into());
    }
    if central_junction::STANDARD_CURRENCY_PUBLIC_NAME.is_some()
        || !ValueInstrument::StandardCurrency.is_ordinary_currency()
        || ValueInstrument::TokeToken.is_ordinary_currency()
        || ValueInstrument::Gremlincoin.is_spendable_money()
        || ValueInstrument::SectorIndex.is_ordinary_currency()
    {
        return Err("ordinary-currency separation drifted".into());
    }
    let indexes = central_junction::canonical_market_indexes();
    if indexes.len() != 4
        || indexes.iter().any(|index| {
            index.owner.is_some()
                || index.currency
                || index.methodology_authority != CentralJunctionInstitution::JunctionBoard
                || index.calculation_authority != CentralJunctionInstitution::SouthRidgeExchange
                || index.publication_authority != CentralJunctionInstitution::JunctionWire
        })
    {
        return Err("official-index independence drifted".into());
    }
    let boards = central_junction::canonical_public_index_boards();
    if boards.len() != 4
        || boards
            .iter()
            .map(|board| board.hall)
            .collect::<BTreeSet<_>>()
            != HouseSectorHall::ALL.into_iter().collect()
        || boards
            .iter()
            .any(|board| board.connected_to != CentralJunctionInstitution::JunctionWire)
    {
        return Err("public Sector Hall index boards drifted".into());
    }
    for concept in [
        SummitConcept::CurrentHaze,
        SummitConcept::EqualGaze,
        SummitConcept::AuraBeam,
    ] {
        if concept.is_market_institution() || concept.is_financial_ticker() {
            return Err("Summit concept was converted into market bureaucracy".into());
        }
    }
    let proof = central_junction::blackroot_workshop_event_proof()?;
    if proof.decision.state != MarketLifecycleState::Recognized
        || proof.decision.outcome != Some(EventOutcome::FactionA)
        || proof.settlement.state != MarketLifecycleState::Settled
        || proof.settlement.market_price_determined_outcome
        || proof.publication.published_by != CentralJunctionInstitution::JunctionWire
    {
        return Err("Blackroot workshop settlement proof drifted".into());
    }

    println!("Central Junction Constitutional Audit: pass");
    println!("source: {}", central_junction::CENTRAL_JUNCTION_SOURCE);
    println!("district: CENTRAL JUNCTION / The Junction");
    println!("ordinary currency: one unnamed standard currency");
    println!("economic poles: Design, Engineering, Craft, Repair");
    println!(
        "market institutions: South Ridge Exchange, Junction Board, Clearing House, Junction Wire"
    );
    println!("largest public Function: The Service Tournament");
    println!("House-owned official indexes: none");
    println!("public Sector Hall index boards: 4 / Junction Wire connected");
    println!("Current Haze market authority: false");
    println!("Equal Gaze market office: false");
    println!("Aura Beam financial ticker: false");
    println!("Blackroot recognized outcome: FactionA");
    println!("market price determined outcome: false");
    println!("recursion kernel dependency: none");
    Ok(())
}
