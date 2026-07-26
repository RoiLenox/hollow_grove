use hollow_grove::gameplay::{LivingCaseId, LivingWorldState};
use hollow_grove::world::extraction::canonical_extraction_sites;

fn main() {
    let state = LivingWorldState::canonical().expect("canonical living-world state");
    let sites = canonical_extraction_sites().expect("canonical extraction sites");
    println!(
        "living revision={} day={} shift={:?} weather={:?}",
        state.revision, state.clock.day, state.clock.shift, state.weather
    );
    println!(
        "surfaces=3 extraction-sites={} cases={} scheduled-people={} custody-lots={}",
        sites.len(),
        state.cases.len(),
        state.people.len(),
        state.custody.len()
    );
    for site in sites {
        let conditions = state
            .extraction
            .get(&site.id)
            .expect("site has canonical state");
        println!(
            "{} / route={} / method={:?} / resource={} / status={:?} / integrity={} / hazard={}",
            site.id.display_name(),
            site.id.route().display_name(),
            site.method,
            site.resource.display_name(),
            conditions.status,
            conditions.structural_integrity,
            conditions.hazard_pressure
        );
    }
    for case_id in LivingCaseId::ALL {
        println!(
            "{} / jurisdiction={} / evidence={} / decision-maker={}",
            case_id.stable_id(),
            case_id.jurisdiction(),
            case_id.required_evidence().len(),
            case_id.decision_maker()
        );
    }
}
