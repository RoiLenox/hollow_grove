use std::collections::BTreeSet;
use std::process::ExitCode;

use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::central_junction::CentralJunctionFunction;
use hollow_grove::world::service_tournament::{
    HouseColorFamily, PairedServiceIdentity, ScoringCategory, canonical_service_tournament,
    canonical_war_of_a_thousand_hues,
};
use hollow_grove::world::service_tournament_archive::{
    CANONICAL_TOURNAMENT_YEAR_ID, FlagshipArtifactKind, SERVICE_TOURNAMENT_ARCHIVE_VERSION,
    canonical_color_registry, decode_service_tournament_archive, encode_service_tournament_archive,
    final_result, flagship_artifact,
};
use hollow_grove::world::service_tournament_fixture::{
    canonical_service_tournament_archive_fixture, canonical_tournament_year_fixture,
};

fn main() -> ExitCode {
    match audit() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("FAIL service-tournament-audit: {error}");
            ExitCode::FAILURE
        }
    }
}

fn require(condition: bool, label: &str) -> Result<(), String> {
    if condition {
        println!("PASS {label}");
        Ok(())
    } else {
        Err(label.into())
    }
}

fn audit() -> Result<(), String> {
    let fixture = canonical_service_tournament_archive_fixture();
    let year_record = canonical_tournament_year_fixture();
    let bytes = encode_service_tournament_archive(&fixture).map_err(|error| error.to_string())?;
    let decoded = decode_service_tournament_archive(&bytes).map_err(|error| error.to_string())?;
    let replayed = decode_service_tournament_archive(&bytes).map_err(|error| error.to_string())?;
    let year = decoded
        .years
        .values()
        .next()
        .ok_or_else(|| "canonical year missing after replay".to_owned())?;
    let tournament = canonical_service_tournament();
    let war = canonical_war_of_a_thousand_hues();

    require(
        year.id.as_str() == CANONICAL_TOURNAMENT_YEAR_ID
            && decoded.archive_version == SERVICE_TOURNAMENT_ARCHIVE_VERSION,
        &format!(
            "Tournament identity={} archive-version={}",
            year.id, decoded.archive_version
        ),
    )?;
    require(
        tournament.function == CentralJunctionFunction::ServiceTournament
            && year_record.central_junction_function_id == tournament.function.stable_id(),
        "Central Junction Function placement",
    )?;
    require(
        year.tournament_runtime.competitors().len() == 4,
        "exactly four House representatives",
    )?;
    for (identity, expected) in [
        (PairedServiceIdentity::FlyntAtfArmy, "Flynt = ATF & Army"),
        (
            PairedServiceIdentity::StonebendDeaAirForce,
            "Stonebend = DEA & Air Force",
        ),
        (
            PairedServiceIdentity::SandmanorCiaNavy,
            "Sandmanor = CIA & Navy",
        ),
        (
            PairedServiceIdentity::GlaushouseFbiMarines,
            "Glaüshouse = FBI & Marines",
        ),
    ] {
        require(
            year.tournament_runtime
                .competitors()
                .values()
                .any(|competitor| competitor.service_identity == identity),
            expected,
        )?;
    }

    let colors = canonical_color_registry();
    for (house, color, label) in [
        (House::Stonebend, HouseColorFamily::Blue, "Stonebend = blue"),
        (House::Sandmanor, HouseColorFamily::Red, "Sandmanor = red"),
        (
            House::Glaushouse,
            HouseColorFamily::Green,
            "Glaüshouse = green",
        ),
        (House::Flynt, HouseColorFamily::Black, "Flynt = black"),
    ] {
        require(colors.get(&house) == Some(&color), label)?;
    }
    require(
        war.nonlethal
            && year
                .tournament_runtime
                .scenarios()
                .values()
                .all(|scenario| scenario.nonlethal),
        "War of a Thousand Hues nonlethality",
    )?;
    require(
        year.tournament_runtime.scenarios().len() >= 5,
        &format!(
            "scenario count={}",
            year.tournament_runtime.scenarios().len()
        ),
    )?;
    require(
        year.real_emergencies.values().any(|emergency| {
            emergency.initially_interpreted_as_simulation
                && emergency.determined_real
                && emergency.simulation_suspended
        }),
        "real-emergency distinction",
    )?;
    require(
        year.tournament_runtime
            .service_marks()
            .values()
            .any(|mark| {
                mark.ordered_paint_layers.len() == 4
                    && mark.provenance.source_marks.len() == 4
                    && mark.houses.len() == 4
            }),
        "layered mark integrity",
    )?;
    require(
        year.tournament_runtime
            .service_marks()
            .values()
            .all(|mark| {
                !mark.provenance.evidence.is_empty()
                    && !mark.provenance.source_action_events.is_empty()
                    && !mark.participants.is_empty()
                    && !mark.constitutional_significance.is_empty()
            }),
        "Service Mark provenance",
    )?;
    let result = final_result(year).ok_or_else(|| "final result missing".to_owned())?;
    let categories = ScoringCategory::ALL.into_iter().collect::<BTreeSet<_>>();
    require(
        result.scorecards.values().all(|scorecard| {
            scorecard.scores.keys().copied().collect::<BTreeSet<_>>() == categories
        }),
        &format!("scoring categories={}", categories.len()),
    )?;
    require(
        year.scoring_events
            .values()
            .any(|event| event.constitutional_restraint_decision),
        "constitutional restraint",
    )?;

    let edge = flagship_artifact(year, FlagshipArtifactKind::EdgeOfTomorrow)
        .ok_or_else(|| "Edge missing".to_owned())?;
    let glass = flagship_artifact(year, FlagshipArtifactKind::GlassOfAThousandHues)
        .ok_or_else(|| "Glass missing".to_owned())?;
    require(
        edge.id.as_str() == FlagshipArtifactKind::EdgeOfTomorrow.stable_id()
            && edge.completed_synthesis
            && edge.custody.is_some(),
        &format!(
            "Edge identity={} custody={}",
            edge.id,
            edge.custody
                .as_ref()
                .map_or("none", |custody| custody.custodian_house.as_str())
        ),
    )?;
    require(
        glass.id.as_str() == FlagshipArtifactKind::GlassOfAThousandHues.stable_id()
            && glass.completed_synthesis
            && glass.custody.is_some(),
        &format!(
            "Glass identity={} custody={}",
            glass.id,
            glass
                .custody
                .as_ref()
                .map_or("none", |custody| custody.custodian_house.as_str())
        ),
    )?;
    require(
        edge.custody.as_ref().map(|value| &value.custodian_id)
            != glass.custody.as_ref().map(|value| &value.custodian_id),
        "Edge and Glass custody may differ",
    )?;
    require(
        !result.transfers_permanent_sovereignty
            && !year.war_result.transfers_permanent_sovereignty
            && year
                .prize_awards
                .values()
                .all(|award| !award.grants_sovereignty),
        "sovereignty prohibition",
    )?;
    require(
        decoded.years == replayed.years,
        "deterministic replay result",
    )?;
    require(
        encode_service_tournament_archive(&decoded.payload).map_err(|error| error.to_string())?
            == bytes,
        "archive checksum validity and canonical encoding",
    )?;
    println!("PASS archive checksum={}", decoded.checksum);
    Ok(())
}
