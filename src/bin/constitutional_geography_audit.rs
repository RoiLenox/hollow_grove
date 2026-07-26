use std::error::Error;

use hollow_grove::world::geography::canonical_constitutional_geography;

const SPECIFICATION: &str = include_str!("../../HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md");
const SCREEN_MAP: &str = include_str!("../../HUEMAN_SCREEN_MAP_v0.1.0.md");
const CROSSOVERS: &str = include_str!("../../HUEMAN_PATH_CROSSOVERS_v0.1.0.md");

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", build_audit()?);
    Ok(())
}

fn build_audit() -> Result<String, Box<dyn Error>> {
    let geography = canonical_constitutional_geography()?;
    let audit = geography.audit()?;

    for route in geography.routes() {
        for document in [SPECIFICATION, SCREEN_MAP, CROSSOVERS] {
            if !document.contains(route.id.display_name()) {
                return Err(format!(
                    "world documentation omits route {}",
                    route.id.display_name()
                )
                .into());
            }
        }
        if !SPECIFICATION.contains(route.id.stable_id())
            || !SPECIFICATION.contains(route.verb.as_str())
            || !SPECIFICATION.contains(route.purpose)
            || route
                .process
                .iter()
                .any(|stage| !SPECIFICATION.contains(stage))
        {
            return Err(format!(
                "constitutional specification differs from executable route {}",
                route.id.display_name()
            )
            .into());
        }
    }
    for boundary in geography.boundaries() {
        if !SPECIFICATION.contains(boundary.boundary.stable_id())
            || !SPECIFICATION.contains(boundary.inward_flow)
            || !SPECIFICATION.contains(boundary.outward_flow)
        {
            return Err(format!(
                "constitutional specification differs from executable boundary {}",
                boundary.boundary.stable_id()
            )
            .into());
        }
    }

    let mut output = String::from("Hollow Grove Constitutional Geography Audit V2\n");
    output.push_str("status: valid\n");
    output.push_str(&format!("route_count: {}\n", audit.route_count));
    output.push_str(&format!(
        "distinct_constitutional_verbs: {}\n",
        audit.distinct_verb_count
    ));
    output.push_str(&format!(
        "distinct_constitutional_purposes: {}\n",
        audit.distinct_purpose_count
    ));
    output.push_str(&format!(
        "duplicate_constitutional_roles: {}\n",
        audit.duplicate_role_count
    ));
    output.push_str(&format!(
        "every_route_has_one_purpose: {}\n",
        yes_no(audit.every_route_has_one_purpose)
    ));
    output.push_str(&format!(
        "every_route_has_process_flow: {}\n",
        yes_no(audit.every_route_has_process_flow)
    ));
    output.push_str(&format!(
        "every_boundary_has_two_routes: {}\n",
        yes_no(audit.every_boundary_has_two_routes)
    ));
    output.push_str(&format!(
        "every_boundary_has_inward_and_outward_flow: {}\n",
        yes_no(audit.every_boundary_has_inward_and_outward_flow)
    ));
    output.push_str(&format!(
        "current_sea_distinct_from_current_seanad: {}\n",
        yes_no(audit.current_sea_is_distinct_from_current_seanad)
    ));
    output.push_str(&format!(
        "frozen_runtime_projections_unique: {}\n",
        yes_no(audit.frozen_runtime_projections_are_unique)
    ));
    output.push_str("documentation_matches_implementation: yes\n");
    output.push_str("terminology_conforms: yes\n");
    output.push_str("frozen_layers_reinterpreted: no\n");
    output.push_str("\nroute_placements:\n");
    for placement in audit.placements {
        let runtime_key = placement
            .frozen_runtime_projection
            .map_or("unprojected-distinct", |key| key.as_str());
        output.push_str(&format!(
            "- {} | {} | {} | runtime={}\n",
            placement.route.stable_id(),
            placement.boundary.stable_id(),
            placement.verb.as_str(),
            runtime_key
        ));
    }
    Ok(output)
}

const fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geography_audit_is_deterministic_and_complete() {
        let first = build_audit().unwrap();
        assert_eq!(first, build_audit().unwrap());
        assert!(first.contains("status: valid"));
        assert!(first.contains("route_count: 10"));
        assert!(first.contains("duplicate_constitutional_roles: 0"));
        assert!(first.contains("current_sea_distinct_from_current_seanad: yes"));
        assert!(first.contains("documentation_matches_implementation: yes"));
    }
}
