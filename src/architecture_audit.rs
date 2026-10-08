//! Read-only architecture crosswalk. Source links are declarations, not execution evidence.
use std::collections::BTreeSet;
use std::fmt::Write;
use std::path::{Component, Path};

use serde::Serialize;

use crate::constitutional::HouseFunction;
use crate::gameplay::{
    HollowGroveGameRuntime, LoreInspection, LoreInspectionError, SympioteFoundationPhase,
};
use crate::hollow_grove_contract::House;
use crate::kernel_invariants::KernelInvariantReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchitectureLayer {
    Recursion,
    Semantic,
    Constitutional,
    House,
    SharedInterface,
    Continuity,
    Gameplay,
    OperationalLore,
    Presentation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArchitectureLink {
    pub id: &'static str,
    pub layer: ArchitectureLayer,
    pub authority_source: &'static str,
    pub implementation: &'static str,
    pub declared_proof_source: &'static str,
    pub persistence: Option<&'static str>,
    pub projection: Option<&'static str>,
    pub principal_house: Option<House>,
    #[serde(serialize_with = "serialize_house_function")]
    pub principal_function: Option<HouseFunction>,
}

fn serialize_house_function<S: serde::Serializer>(
    function: &Option<HouseFunction>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    function
        .map(|function| format!("{function:?}"))
        .serialize(serializer)
}

fn link(
    id: &'static str,
    layer: ArchitectureLayer,
    authority_source: &'static str,
    implementation: &'static str,
    proof: &'static str,
    persistence: Option<&'static str>,
    projection: Option<&'static str>,
) -> ArchitectureLink {
    ArchitectureLink {
        id,
        layer,
        authority_source,
        implementation,
        declared_proof_source: proof,
        persistence,
        projection,
        principal_house: None,
        principal_function: None,
    }
}

/// Bounded core crosswalk; this does not claim to enumerate every civic rule.
#[must_use]
pub fn canonical_architecture_links() -> Vec<ArchitectureLink> {
    use ArchitectureLayer::{
        Constitutional, Continuity, Gameplay, House as HouseLayer, OperationalLore, Presentation,
        Recursion, Semantic, SharedInterface,
    };
    let mut links = vec![
        link(
            "architecture.compromise",
            Constitutional,
            "HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md",
            "src/hollow_grove_contract.rs",
            "tests/hollow_grove_alignment.rs",
            None,
            Some("HOLLOW_GROVE_CONSTITUTIONAL_ARCHITECTURE_V1.md"),
        ),
        link(
            "architecture.recursion",
            Recursion,
            "HOLLOW_GROVE_CORE_v1.0.0.md",
            "hollow-grove-recursion-kernel/src/lib.rs",
            "tests/kernel_purity.rs",
            None,
            Some("src/kernel_pass_output.rs"),
        ),
        link(
            "architecture.semantic",
            Semantic,
            "HOLLOW_GROVE_SEMANTIC_FOUNDATION_V1.md",
            "hollow-grove-semantic-kernel/src/lib.rs",
            "tests/semantic_geometry.rs",
            Some("src/semantic/state.rs"),
            Some("src/semantic/mod.rs"),
        ),
        link(
            "architecture.constitutional",
            Constitutional,
            "HOLLOW_GROVE_V2_CONSTITUTIONAL_SPECIFICATION.md",
            "src/constitutional/mod.rs",
            "tests/constitutional_runtime.rs",
            Some("src/constitutional/persistence.rs"),
            Some("src/constitutional/visual_identity.rs"),
        ),
        link(
            "architecture.stonebend",
            HouseLayer,
            "STONEBEND_CONSTITUTION_V2.md",
            "src/world/stonebend.rs",
            "tests/stonebend_constitutional_architecture.rs",
            Some("src/world/persistence.rs"),
            Some("src/gameplay/view.rs"),
        ),
        link(
            "architecture.sandmanor",
            HouseLayer,
            "SANDMANOR_CONSTITUTION_V2.md",
            "src/world/sandmanor.rs",
            "tests/sandmanor_constitutional_architecture.rs",
            Some("src/world/persistence.rs"),
            Some("src/gameplay/view.rs"),
        ),
        link(
            "architecture.glaushouse",
            HouseLayer,
            "GLAUSHOUSE_CONSTITUTION_V2.md",
            "src/world/glaushouse.rs",
            "tests/glaushouse_constitutional_architecture.rs",
            Some("src/world/persistence.rs"),
            Some("src/gameplay/view.rs"),
        ),
        link(
            "architecture.flynt",
            HouseLayer,
            "FLYNT_CONSTITUTION_V2.md",
            "officials-and-outlaws/src/lib.rs",
            "tests/flynt_constitutional_architecture.rs",
            Some("src/world/persistence.rs"),
            Some("src/world/flynt.rs"),
        ),
        link(
            "architecture.geography",
            SharedInterface,
            "HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md",
            "src/world/geography.rs",
            "tests/constitutional_geography.rs",
            Some("src/world/constitutional_interfaces.rs"),
            Some("src/gameplay/world.rs"),
        ),
        link(
            "architecture.spatial",
            Semantic,
            "SPATIAL_INHERITANCE_V1.md",
            "src/world/spatial_inheritance.rs",
            "tests/spatial_inheritance.rs",
            Some("src/world/spatial_topology_v2.rs"),
            Some("hueman_godot/scripts/runtime_client.gd"),
        ),
        link(
            "architecture.shared-interfaces",
            SharedInterface,
            "HOLLOW_GROVE_CONSTITUTIONAL_ARCHITECTURE_V1.md",
            "src/world/constitutional_interfaces.rs",
            "tests/institutional_boundary.rs",
            Some("src/world/constitutional_interfaces.rs"),
            Some("src/world/session.rs"),
        ),
        link(
            "architecture.central-junction",
            SharedInterface,
            "CENTRAL_JUNCTION_FOUR_POLE_ECONOMY_V1.md",
            "src/world/central_junction.rs",
            "tests/central_junction_constitutional_architecture.rs",
            Some("src/world/constitutional_interfaces.rs"),
            Some("src/gameplay/view.rs"),
        ),
        link(
            "architecture.service-tournament",
            SharedInterface,
            "SERVICE_TOURNAMENT_CENTRAL_JUNCTION_CANON_V1.md",
            "src/world/service_tournament.rs",
            "tests/service_tournament_archive_v1.rs",
            Some("src/world/service_tournament_archive.rs"),
            Some("src/world/service_tournament_fixture.rs"),
        ),
        link(
            "architecture.rampart",
            Continuity,
            "THE_RAMPART_V1.md",
            "src/runtime_rampart.rs",
            "tests/runtime_rampart_v1.rs",
            Some("src/runtime_rampart.rs"),
            Some("src/gameplay/view.rs"),
        ),
        link(
            "architecture.guides",
            Continuity,
            "docs/SURFACE_HUEMAN_GUIDE_PROTOCOL_V1.md",
            "src/guide_state_machine.rs",
            "tests/guide_state_machine_v2.rs",
            Some("src/guide_state_machine.rs"),
            Some("src/gameplay/view.rs"),
        ),
        link(
            "architecture.gameplay",
            Gameplay,
            "HUEMAN_v0.1.0.md",
            "src/gameplay/runtime.rs",
            "tests/gameplay_runtime.rs",
            Some("src/gameplay/archive.rs"),
            Some("src/gameplay/view.rs"),
        ),
        link(
            "architecture.operational-lore",
            OperationalLore,
            "HOLLOW_GROVE_OPERATIONAL_LORE_KERNEL_V1_DRAFT.md",
            "src/world/lived_lore.rs",
            "tests/functional_lore_integration.rs",
            Some("src/world/lived_lore.rs"),
            Some("src/gameplay/lore_binding.rs"),
        ),
        link(
            "architecture.sympiote-foundation",
            Gameplay,
            "HOLLOW_GROVE_SYMPIOTE_FOUNDATION_V1.md",
            "src/world/sympiote_foundation.rs",
            "tests/sympiote_foundation.rs",
            Some("src/gameplay/archive.rs"),
            Some("src/gameplay/view.rs"),
        ),
        link(
            "architecture.recipes",
            Gameplay,
            "HOLLOW_GROVE_RECIPE_CONSTITUTION_V1.md",
            "src/recipe_discovery.rs",
            "tests/gameplay_synthesis_pilot.rs",
            Some("src/gameplay/archive.rs"),
            Some("src/gameplay/view.rs"),
        ),
        link(
            "architecture.survey-frame",
            Gameplay,
            crate::world::survey_frame::SURVEY_FRAME_SOURCE,
            "src/world/survey_frame.rs",
            "tests/survey_frame.rs",
            Some("src/gameplay/archive.rs"),
            Some("src/gameplay/view.rs"),
        ),
        link(
            "architecture.sympiote-graft",
            HouseLayer,
            "HOLLOW_GROVE_POWER_RECIPE_CONSTITUTION_V1.md",
            "src/world/sympiote.rs",
            "tests/power_recipe_constitution.rs",
            Some("src/world/sympiote.rs"),
            None,
        ),
        link(
            "architecture.presentation",
            Presentation,
            "docs/HOLLOW_GROVE_HUEMAN_SEAM_V1.md",
            "hueman_godot/scripts/runtime_client.gd",
            "hueman_godot/tests/runtime_client_smoke.gd",
            None,
            Some("hueman_godot/scripts/retro_overworld.gd"),
        ),
        link(
            "architecture.backend-house-handoffs",
            OperationalLore,
            crate::world::lore_handoffs::SOURCE,
            crate::world::lore_handoffs::IMPLEMENTATION,
            crate::world::lore_handoffs::PROOF_SOURCE,
            Some(crate::world::lore_handoffs::IMPLEMENTATION),
            Some(crate::world::lore_handoffs::IMPLEMENTATION),
        ),
    ];
    for (id, house, function) in [
        (
            "architecture.stonebend",
            House::Stonebend,
            HouseFunction::Name,
        ),
        (
            "architecture.sandmanor",
            House::Sandmanor,
            HouseFunction::Prove,
        ),
        (
            "architecture.glaushouse",
            House::Glaushouse,
            HouseFunction::Clear,
        ),
        ("architecture.flynt", House::Flynt, HouseFunction::Recognize),
    ] {
        let item = links
            .iter_mut()
            .find(|item| item.id == id)
            .expect("canonical House link");
        item.principal_house = Some(house);
        item.principal_function = Some(function);
    }
    links
}

fn safe_source_path(path: &str) -> bool {
    !path.is_empty()
        && Path::new(path)
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
}

/// Validates link structure and file presence; it does not execute the declared tests.
#[must_use]
pub fn audit_architecture_links(root: &Path, links: &[ArchitectureLink]) -> KernelInvariantReport {
    let mut ids = BTreeSet::new();
    let mut authorities = BTreeSet::new();
    let mut violations = Vec::new();
    for item in links {
        if item.id.is_empty() || !ids.insert(item.id) {
            violations.push(format!(
                "duplicate or empty architecture component: {}",
                item.id
            ));
        }
        if !authorities.insert(item.authority_source) {
            violations.push(format!(
                "duplicate primary contract ownership: {}",
                item.authority_source
            ));
        }
        for path in [
            Some(item.authority_source),
            Some(item.implementation),
            Some(item.declared_proof_source),
            item.persistence,
            item.projection,
        ]
        .into_iter()
        .flatten()
        {
            if !safe_source_path(path) || !root.join(path).is_file() {
                violations.push(format!(
                    "{} has missing or unsafe source link: {path}",
                    item.id
                ));
            }
        }
        match (item.principal_house, item.principal_function) {
            (Some(house), Some(function))
                if function.constitutional_house() == house
                    && item.layer == ArchitectureLayer::House => {}
            (None, None) => {}
            _ => violations.push(format!("{} substitutes a House function or owner", item.id)),
        }
    }
    for (house, function) in [
        (House::Stonebend, HouseFunction::Name),
        (House::Sandmanor, HouseFunction::Prove),
        (House::Glaushouse, HouseFunction::Clear),
        (House::Flynt, HouseFunction::Recognize),
    ] {
        if links
            .iter()
            .filter(|item| {
                item.principal_house == Some(house) && item.principal_function == Some(function)
            })
            .count()
            != 1
        {
            violations.push(format!(
                "exactly one principal {house:?}/{function:?} mapping required"
            ));
        }
    }
    KernelInvariantReport {
        schema_version: crate::kernel_invariants::KERNEL_INVARIANT_SCHEMA_VERSION,
        checked_invariants: vec![
            "architecture-component-identities-unique",
            "architecture-primary-contract-owner-unique",
            "architecture-source-links-resolve",
            "architecture-four-house-functions-match-existing-law",
        ],
        violations,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoreArchitectureCoverage {
    pub lore_id: &'static str,
    pub house: House,
    pub authority_source: &'static str,
    pub definition_owner: &'static str,
    pub gameplay_case: Option<&'static str>,
    pub backend_handoff: Option<&'static str>,
    pub reducer_owner: Option<&'static str>,
    pub persistence_owner: Option<&'static str>,
    pub projection_owner: Option<&'static str>,
    pub execution_complete_in_supplied_save: Option<bool>,
    pub execution_complete_in_supplied_handoff: Option<bool>,
    pub gap: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArchitectureCoverageReport {
    pub schema_version: u16,
    pub scope: &'static str,
    pub source_links_passed: bool,
    pub checked_invariants: Vec<&'static str>,
    pub violations: Vec<String>,
    pub components: Vec<ArchitectureLink>,
    pub canonical_route_contracts: Vec<LoreArchitectureCoverage>,
    pub additional_foundation_contract: LoreArchitectureCoverage,
    pub additional_survey_frame_contract: LoreArchitectureCoverage,
    pub verified_binding_count: usize,
    pub unbound_canonical_route_count: usize,
    pub backend_handoff_binding_count: usize,
    pub unbound_gameplay_route_count: usize,
    pub replayed_handoff_route: Option<&'static str>,
    pub replayed_save_revision: Option<u64>,
}

impl ArchitectureCoverageReport {
    pub fn inspect(
        root: &Path,
        runtime: Option<&HollowGroveGameRuntime>,
    ) -> Result<Self, LoreInspectionError> {
        let components = canonical_architecture_links();
        let mut audit = audit_architecture_links(root, &components);
        let inspection = LoreInspection::from_runtime(runtime)?;
        let mut contracts = Vec::new();
        for definition in inspection.definitions {
            let backend = crate::world::lore_handoffs::ROUTES
                .into_iter()
                .find(|route| route.lore_id() == definition.lore_id);
            let reducer = if definition.lore_id == crate::gameplay::BOARDWALK_RETURN_LORE_ID
                && definition.gameplay_binding == Some(crate::gameplay::BOARDWALK_RETURN_CASE_ID)
            {
                Some("src/gameplay/boardwalk.rs")
            } else if definition.lore_id == crate::gameplay::CURRENT_SEA_CONTINUITY_LORE_ID
                && definition.gameplay_binding
                    == Some(crate::gameplay::CURRENT_SEA_CONTINUITY_CASE_ID)
            {
                Some("src/gameplay/stonebend.rs")
            } else if backend.is_some() {
                Some(crate::world::lore_handoffs::IMPLEMENTATION)
            } else {
                None
            };
            if definition.gameplay_binding.is_some() && reducer.is_none() {
                audit.violations.push(format!(
                    "{} has no matching audited binding owner",
                    definition.lore_id
                ));
            }
            contracts.push(LoreArchitectureCoverage {
                lore_id: definition.lore_id,
                house: definition.house,
                authority_source: definition.authority_source,
                definition_owner: "src/world/lived_lore.rs",
                gameplay_case: definition.gameplay_binding,
                backend_handoff: backend.map(|r| r.lore_id()),
                reducer_owner: reducer,
                persistence_owner: reducer.map(|_| {
                    if backend.is_some() {
                        crate::world::lore_handoffs::IMPLEMENTATION
                    } else {
                        "src/gameplay/archive.rs"
                    }
                }),
                projection_owner: reducer.map(|_| {
                    if backend.is_some() {
                        crate::world::lore_handoffs::IMPLEMENTATION
                    } else {
                        "src/gameplay/lore_binding.rs"
                    }
                }),
                execution_complete_in_supplied_save: definition
                    .execution
                    .as_ref()
                    .map(|execution| execution.complete)
                    .or_else(|| {
                        definition
                            .stonebend_execution
                            .as_ref()
                            .map(|execution| execution.complete)
                    }),
                execution_complete_in_supplied_handoff: None,
                gap: reducer
                    .is_none()
                    .then_some("no backend or gameplay execution binding"),
            });
        }
        let foundation = inspection.sympiote_foundation_contract;
        if foundation.gameplay_binding
            != Some(crate::world::sympiote_foundation::SYMPIOTE_FOUNDATION_CASE_ID)
        {
            audit
                .violations
                .push("foundation binding does not match its reducer-owned case".into());
        }
        let additional = LoreArchitectureCoverage {
            lore_id: foundation.lore_id,
            house: foundation.house,
            authority_source: foundation.authority_source,
            definition_owner: "src/world/sympiote_foundation.rs",
            gameplay_case: foundation.gameplay_binding,
            backend_handoff: None,
            reducer_owner: Some("src/world/sympiote_foundation.rs"),
            persistence_owner: Some("src/gameplay/archive.rs"),
            projection_owner: Some("src/gameplay/view.rs"),
            execution_complete_in_supplied_save: runtime.map(|runtime| {
                let state = runtime.sympiote_foundation();
                state.form_established()
                    && state.phase() == SympioteFoundationPhase::Discharged
                    && state.comfortable_use()
                    && !state.needs_rest()
            }),
            execution_complete_in_supplied_handoff: None,
            gap: None,
        };
        let unbound = contracts
            .iter()
            .filter(|item| item.reducer_owner.is_none())
            .count();
        let frame_contract = inspection.survey_frame_contract;
        if frame_contract.gameplay_binding != Some(crate::world::survey_frame::SURVEY_FRAME_CASE_ID)
        {
            audit
                .violations
                .push("survey-frame binding does not match its reducer-owned case".into());
        }
        let object_case = LoreArchitectureCoverage {
            lore_id: frame_contract.lore_id,
            house: frame_contract.house,
            authority_source: frame_contract.authority_source,
            definition_owner: "src/world/survey_frame.rs",
            gameplay_case: frame_contract.gameplay_binding,
            backend_handoff: None,
            reducer_owner: Some("src/world/survey_frame.rs"),
            persistence_owner: Some("src/gameplay/archive.rs"),
            projection_owner: Some("src/gameplay/view.rs"),
            execution_complete_in_supplied_save: runtime.map(|r| r.survey_frame().complete()),
            execution_complete_in_supplied_handoff: None,
            gap: None,
        };
        let backend_count = contracts
            .iter()
            .filter(|c| c.backend_handoff.is_some())
            .count();
        let gameplay_unbound = contracts
            .iter()
            .filter(|c| c.gameplay_case.is_none())
            .count();
        let bound = contracts.len() - unbound + usize::from(additional.reducer_owner.is_some()) + 1;
        Ok(Self {
            schema_version: 3,
            scope: "read-only bounded core crosswalk and lore-binding coverage; source presence and declared test links do not attest execution or complete whole-world implementation",
            source_links_passed: audit.passed(),
            checked_invariants: audit.checked_invariants,
            violations: audit.violations,
            components,
            canonical_route_contracts: contracts,
            additional_foundation_contract: additional,
            additional_survey_frame_contract: object_case,
            verified_binding_count: bound,
            unbound_canonical_route_count: unbound,
            backend_handoff_binding_count: backend_count,
            unbound_gameplay_route_count: gameplay_unbound,
            replayed_handoff_route: None,
            replayed_save_revision: runtime.map(|runtime| runtime.revision().get()),
        })
    }

    /// An independently replayed handoff attests only its own route and scope.
    pub fn with_handoff(mut self, service: &crate::world::lore_handoffs::HandoffService) -> Self {
        let route = service.route().lore_id();
        if let Some(contract) = self
            .canonical_route_contracts
            .iter_mut()
            .find(|c| c.backend_handoff == Some(route))
        {
            contract.execution_complete_in_supplied_handoff = Some(service.complete());
            self.replayed_handoff_route = Some(route);
        }
        self
    }

    #[must_use]
    pub fn witness_markdown(&self) -> String {
        let mut output = format!(
            "# Hollow Grove Architecture Coverage\n\n{}\n\nSource-link audit: {}\nVerified bindings: {}; unbound canonical route contracts: {}\nSupplied-save revision: {:?}\n\n",
            self.scope,
            if self.source_links_passed {
                "passed"
            } else {
                "failed"
            },
            self.verified_binding_count,
            self.unbound_canonical_route_count,
            self.replayed_save_revision
        );
        let _ = writeln!(
            output,
            "Headless backend handoffs: {}; deferred gameplay route bindings: {}; supplied handoff: {:?}\n",
            self.backend_handoff_binding_count,
            self.unbound_gameplay_route_count,
            self.replayed_handoff_route
        );
        output.push_str("| Component | Layer | Contract | Implementation | Declared proof source |\n|---|---|---|---|---|\n");
        for item in &self.components {
            let _ = writeln!(
                output,
                "| {} | {:?} | `{}` | `{}` | `{}` |",
                item.id,
                item.layer,
                item.authority_source,
                item.implementation,
                item.declared_proof_source
            );
        }
        output.push_str(
            "\nLore bindings (completion requires a supplied validated game runtime or replayed backend handoff):\n\n",
        );
        for item in self
            .canonical_route_contracts
            .iter()
            .chain(std::iter::once(&self.additional_foundation_contract))
            .chain(std::iter::once(&self.additional_survey_frame_contract))
        {
            let _ = writeln!(
                output,
                "- {} → {}; game completion: {:?}; backend handoff completion: {:?}; {}",
                item.lore_id,
                item.gameplay_case
                    .or(item.backend_handoff)
                    .unwrap_or("unbound"),
                item.execution_complete_in_supplied_save,
                item.execution_complete_in_supplied_handoff,
                item.gap
                    .unwrap_or("reducer, persistence and projection links recorded")
            );
        }
        for violation in &self.violations {
            let _ = writeln!(output, "- Violation: {violation}");
        }
        output
    }
}
