use std::error::Error;

use flynt_constitution::{FORM_MANTICORP, RECIPE_MANTICORP, canonical_constitution};
use hollow_grove::world::flynt::canonical_flynt_institutions;

const CONSTITUTION_DOCUMENT: &str = include_str!("../../FLYNT_CONSTITUTION_V2.md");
const HUEMAN_PROJECTION: &str = include_str!("../../artifacts/hueman_flynt_constitution.md");

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", build_audit()?);
    Ok(())
}

fn build_audit() -> Result<String, Box<dyn Error>> {
    let constitution = canonical_constitution()?;
    let audit = constitution.audit()?;
    let world = canonical_flynt_institutions();
    world
        .validate()
        .map_err(|error| format!("world projection rejected: {error:?}"))?;

    for node in constitution.nodes() {
        if !CONSTITUTION_DOCUMENT.contains(node.id.as_str())
            || !CONSTITUTION_DOCUMENT.contains(&node.name)
            || !HUEMAN_PROJECTION.contains(&node.name)
        {
            return Err(format!("documentation omits canonical node {}", node.id).into());
        }
    }
    for stable in [
        FORM_MANTICORP,
        RECIPE_MANTICORP,
        "Tross = Mystery Man = Mr. X",
    ] {
        if !CONSTITUTION_DOCUMENT.contains(stable) || !HUEMAN_PROJECTION.contains(stable) {
            return Err(
                format!("documentation omits Manticorp identity invariant {stable}").into(),
            );
        }
    }

    let mut output = String::from("Flynt Constitutional Audit V2\n");
    output.push_str("status: valid\n");
    output.push_str(&format!(
        "sovereign_executive: {}\n",
        audit.sovereign_executive
    ));
    output.push_str(&format!(
        "constitutional_chimera_count: {}\n",
        audit.constitutional_chimera_count
    ));
    output.push_str(&format!(
        "chimera_recipe_count: {}\n",
        audit.chimera_recipe_count
    ));
    output.push_str(&format!(
        "manticorp_recipe_count: {}\n",
        audit.manticorp_recipe_count
    ));
    output.push_str(&format!(
        "duplicate_authority_count: {}\n",
        audit.duplicate_authority_count
    ));
    output.push_str(&format!(
        "all_non_root_nodes_have_one_superior: {}\n",
        yes_no(audit.all_non_root_nodes_have_one_superior)
    ));
    output.push_str(&format!(
        "hierarchy_is_acyclic: {}\n",
        yes_no(audit.hierarchy_is_acyclic)
    ));
    output.push_str(&format!(
        "all_authority_reaches_tross: {}\n",
        yes_no(audit.all_authority_reaches_tross)
    ));
    output.push_str(&format!(
        "gallowry_is_distinct_from_gallows: {}\n",
        yes_no(audit.gallowry_is_distinct_from_gallows)
    ));
    output.push_str(&format!(
        "founding_union_is_complete: {}\n",
        yes_no(audit.founding_union_is_complete)
    ));
    output.push_str("documentation_matches_implementation: yes\n");
    output.push_str("world_projection_valid: yes\n");
    output.push_str("\ninstitution_placements:\n");
    for placement in &audit.institution_placements {
        output.push_str(&format!(
            "- {} -> {}\n",
            placement.id,
            placement
                .superior
                .as_ref()
                .map_or("none", |superior| superior.as_str())
        ));
    }
    output.push_str("office_placements:\n");
    for placement in &audit.office_placements {
        output.push_str(&format!(
            "- {} -> {}\n",
            placement.id,
            placement
                .superior
                .as_ref()
                .map_or("none", |superior| superior.as_str())
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
    fn audit_is_deterministic_and_complete() {
        let first = build_audit().unwrap();
        assert_eq!(first, build_audit().unwrap());
        assert!(first.contains("status: valid"));
        assert!(first.contains("constitutional_chimera_count: 1"));
        assert!(first.contains("manticorp_recipe_count: 1"));
        assert!(first.contains("duplicate_authority_count: 0"));
        assert!(first.contains("documentation_matches_implementation: yes"));
        assert!(first.contains("world_projection_valid: yes"));
    }
}
