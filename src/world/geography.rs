//! Constitutional route law above the frozen runtime layers.
//!
//! This module does not decide movement, mutate Constitutional Runtime V2, or
//! reinterpret House authority. It gives the world-facing layer one exact,
//! validated answer for why civilization uses each major route.
//!
//! `AuraWay`, `CurrentSea`, and `Riptide` retain `geography.route.*` identities
//! as frozen map/interface projections. They are deliberately distinct from
//! permanent `route.aura-way`, body `region.current-sea`, and forces
//! `force.riptide` / `force.undertow`, whose controlling constitutional law is
//! implemented in `world::way_back` and `world::current_sea_passage`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::hollow_grove_contract::House;

/// The ten constitutionally named major routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConstitutionalRouteId {
    Boardwalk,
    Riptide,
    CurrentSea,
    AuraRidge,
    Glausbahn,
    CurrentSeanad,
    AuraWay,
    MntAura,
    BasinMotorspeedway,
    StairwayToHeaven,
}

impl ConstitutionalRouteId {
    pub const ALL: [Self; 10] = [
        Self::Boardwalk,
        Self::Riptide,
        Self::CurrentSea,
        Self::AuraRidge,
        Self::Glausbahn,
        Self::CurrentSeanad,
        Self::AuraWay,
        Self::MntAura,
        Self::BasinMotorspeedway,
        Self::StairwayToHeaven,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::Boardwalk => "geography.route.boardwalk",
            Self::Riptide => "geography.route.riptide",
            Self::CurrentSea => "geography.route.current-sea",
            Self::AuraRidge => "geography.route.aura-ridge",
            Self::Glausbahn => "geography.route.glausbahn",
            Self::CurrentSeanad => "geography.route.current-seanad",
            Self::AuraWay => "geography.route.aura-way",
            Self::MntAura => "geography.route.mnt-aura",
            Self::BasinMotorspeedway => "geography.route.basin-motorspeedway",
            Self::StairwayToHeaven => "geography.route.stairway-to-heaven",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Boardwalk => "Boardwalk",
            Self::Riptide => "Riptide",
            Self::CurrentSea => "Current Sea",
            Self::AuraRidge => "Aura Ridge",
            Self::Glausbahn => "Glausbahn",
            Self::CurrentSeanad => "Current Seanad",
            Self::AuraWay => "Aura Way",
            Self::MntAura => "Mt. Aura",
            Self::BasinMotorspeedway => "Basin Motor Speedway",
            Self::StairwayToHeaven => "Stairway to Heaven",
        }
    }
}

/// One dominant constitutional verb per route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstitutionalRouteVerb {
    Return,
    Retrieve,
    Certify,
    Witness,
    Refine,
    Deliberate,
    Design,
    Aspire,
    Produce,
    Ascend,
}

impl ConstitutionalRouteVerb {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Return => "Return",
            Self::Retrieve => "Retrieve",
            Self::Certify => "Certify",
            Self::Witness => "Witness",
            Self::Refine => "Refine",
            Self::Deliberate => "Deliberate",
            Self::Design => "Design",
            Self::Aspire => "Aspire",
            Self::Produce => "Produce",
            Self::Ascend => "Ascend",
        }
    }
}

/// The five House boundaries covered by the major-route roster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HouseBoundary {
    FlyntGlaushouse,
    GlaushouseStonebend,
    GlaushouseSandmanor,
    StonebendSandmanor,
    StonebendFlynt,
}

impl HouseBoundary {
    pub const ALL: [Self; 5] = [
        Self::FlyntGlaushouse,
        Self::GlaushouseStonebend,
        Self::GlaushouseSandmanor,
        Self::StonebendSandmanor,
        Self::StonebendFlynt,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::FlyntGlaushouse => "geography.boundary.flynt-glaushouse",
            Self::GlaushouseStonebend => "geography.boundary.glaushouse-stonebend",
            Self::GlaushouseSandmanor => "geography.boundary.glaushouse-sandmanor",
            Self::StonebendSandmanor => "geography.boundary.stonebend-sandmanor",
            Self::StonebendFlynt => "geography.boundary.stonebend-flynt",
        }
    }

    #[must_use]
    pub const fn houses(self) -> [House; 2] {
        match self {
            Self::FlyntGlaushouse => [House::Flynt, House::Glaushouse],
            Self::GlaushouseStonebend => [House::Glaushouse, House::Stonebend],
            Self::GlaushouseSandmanor => [House::Glaushouse, House::Sandmanor],
            Self::StonebendSandmanor => [House::Stonebend, House::Sandmanor],
            Self::StonebendFlynt => [House::Stonebend, House::Flynt],
        }
    }

    #[must_use]
    pub const fn contains(self, house: House) -> bool {
        let [first, second] = self.houses();
        house_eq(first, house) || house_eq(second, house)
    }
}

const fn house_eq(left: House, right: House) -> bool {
    matches!(
        (left, right),
        (House::Stonebend, House::Stonebend)
            | (House::Sandmanor, House::Sandmanor)
            | (House::Glaushouse, House::Glaushouse)
            | (House::Flynt, House::Flynt)
    )
}

/// The dominant process direction, not a restriction on ordinary traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstitutionalFlowDirection {
    Directed { from: House, to: House },
    Reciprocal,
    SharedDeliberation,
}

/// Projection keys exposed by the already-frozen Current Synthesis route layer.
///
/// `Current Sea` intentionally has no value here. The frozen implementation's
/// internal `CurrentSea` token denotes Current Seanad, so mapping the new and
/// distinct Current Sea to that token would create constitutional ambiguity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FrozenRuntimeRouteKey {
    AuraRidge,
    AuraWay,
    BasinMotorspeedway,
    Boardwalk,
    Glausbahn,
    StairwayToHeaven,
    Riptide,
    CurrentSeanad,
    MntAura,
}

impl FrozenRuntimeRouteKey {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuraRidge => "aura-ridge",
            Self::AuraWay => "aura-way",
            Self::BasinMotorspeedway => "basin-motorspeedway",
            Self::Boardwalk => "boardwalk",
            Self::Glausbahn => "glausbahn",
            Self::StairwayToHeaven => "stairway-to-heaven",
            Self::Riptide => "riptide",
            Self::CurrentSeanad => "current-seanad",
            Self::MntAura => "mnt-aura",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalRouteDefinition {
    pub id: ConstitutionalRouteId,
    pub boundary: HouseBoundary,
    pub verb: ConstitutionalRouteVerb,
    pub purpose: &'static str,
    pub direction: ConstitutionalFlowDirection,
    pub process: &'static [&'static str],
    pub frozen_runtime_projection: Option<FrozenRuntimeRouteKey>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryFlowLaw {
    pub boundary: HouseBoundary,
    pub routes: [ConstitutionalRouteId; 2],
    pub inward_flow: &'static str,
    pub outward_flow: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutePlacement {
    pub route: ConstitutionalRouteId,
    pub boundary: HouseBoundary,
    pub verb: ConstitutionalRouteVerb,
    pub frozen_runtime_projection: Option<FrozenRuntimeRouteKey>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalGeographyAudit {
    pub route_count: usize,
    pub distinct_purpose_count: usize,
    pub distinct_verb_count: usize,
    pub duplicate_role_count: usize,
    pub every_route_has_one_purpose: bool,
    pub every_route_has_process_flow: bool,
    pub every_boundary_has_two_routes: bool,
    pub every_boundary_has_inward_and_outward_flow: bool,
    pub current_sea_is_distinct_from_current_seanad: bool,
    pub frozen_runtime_projections_are_unique: bool,
    pub placements: Vec<RoutePlacement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalGeography {
    routes: Vec<ConstitutionalRouteDefinition>,
    boundaries: Vec<BoundaryFlowLaw>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeographyValidationError {
    DuplicateRoute(ConstitutionalRouteId),
    MissingRoute(ConstitutionalRouteId),
    MissingPurpose(ConstitutionalRouteId),
    MissingProcessFlow(ConstitutionalRouteId),
    DuplicateVerb(ConstitutionalRouteVerb),
    DuplicatePurpose(&'static str),
    DirectionOutsideBoundary(ConstitutionalRouteId),
    DuplicateRuntimeProjection(FrozenRuntimeRouteKey),
    CurrentSeaConflatedWithRuntimeRoute,
    RouteDefinitionMismatch(ConstitutionalRouteId),
    DuplicateBoundary(HouseBoundary),
    MissingBoundary(HouseBoundary),
    BoundaryRouteCount {
        boundary: HouseBoundary,
        count: usize,
    },
    BoundaryFlowMissing(HouseBoundary),
    BoundaryDefinitionMismatch(HouseBoundary),
}

impl fmt::Display for GeographyValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateRoute(route) => write!(
                formatter,
                "duplicate constitutional route {}",
                route.display_name()
            ),
            Self::MissingRoute(route) => write!(
                formatter,
                "missing constitutional route {}",
                route.display_name()
            ),
            Self::MissingPurpose(route) => write!(
                formatter,
                "route {} has no constitutional purpose",
                route.display_name()
            ),
            Self::MissingProcessFlow(route) => write!(
                formatter,
                "route {} has no complete constitutional process",
                route.display_name()
            ),
            Self::DuplicateVerb(verb) => {
                write!(
                    formatter,
                    "constitutional route verb {} is duplicated",
                    verb.as_str()
                )
            }
            Self::DuplicatePurpose(purpose) => {
                write!(
                    formatter,
                    "constitutional route purpose is duplicated: {purpose}"
                )
            }
            Self::DirectionOutsideBoundary(route) => write!(
                formatter,
                "route {} points outside its House boundary",
                route.display_name()
            ),
            Self::DuplicateRuntimeProjection(key) => write!(
                formatter,
                "frozen runtime route projection {} is used twice",
                key.as_str()
            ),
            Self::CurrentSeaConflatedWithRuntimeRoute => formatter.write_str(
                "Current Sea must remain distinct from the frozen Current Seanad projection",
            ),
            Self::RouteDefinitionMismatch(route) => write!(
                formatter,
                "route {} differs from canonical route law",
                route.display_name()
            ),
            Self::DuplicateBoundary(boundary) => {
                write!(
                    formatter,
                    "duplicate House boundary {}",
                    boundary.stable_id()
                )
            }
            Self::MissingBoundary(boundary) => {
                write!(formatter, "missing House boundary {}", boundary.stable_id())
            }
            Self::BoundaryRouteCount { boundary, count } => write!(
                formatter,
                "House boundary {} has {count} routes instead of two",
                boundary.stable_id()
            ),
            Self::BoundaryFlowMissing(boundary) => write!(
                formatter,
                "House boundary {} lacks inward or outward constitutional flow",
                boundary.stable_id()
            ),
            Self::BoundaryDefinitionMismatch(boundary) => write!(
                formatter,
                "House boundary {} differs from canonical route law",
                boundary.stable_id()
            ),
        }
    }
}

impl std::error::Error for GeographyValidationError {}

impl ConstitutionalGeography {
    #[must_use]
    pub fn from_parts(
        routes: Vec<ConstitutionalRouteDefinition>,
        boundaries: Vec<BoundaryFlowLaw>,
    ) -> Self {
        Self { routes, boundaries }
    }

    #[must_use]
    pub fn routes(&self) -> &[ConstitutionalRouteDefinition] {
        &self.routes
    }

    #[must_use]
    pub fn boundaries(&self) -> &[BoundaryFlowLaw] {
        &self.boundaries
    }

    #[must_use]
    pub fn route(&self, id: ConstitutionalRouteId) -> Option<&ConstitutionalRouteDefinition> {
        self.routes.iter().find(|route| route.id == id)
    }

    #[must_use]
    pub fn routes_for_boundary(
        &self,
        boundary: HouseBoundary,
    ) -> Vec<&ConstitutionalRouteDefinition> {
        self.routes
            .iter()
            .filter(|route| route.boundary == boundary)
            .collect()
    }

    pub fn validate(&self) -> Result<(), GeographyValidationError> {
        let mut route_ids = BTreeSet::new();
        let mut verbs = BTreeSet::new();
        let mut purposes = BTreeSet::new();
        let mut runtime_keys = BTreeSet::new();

        for route in &self.routes {
            if !route_ids.insert(route.id) {
                return Err(GeographyValidationError::DuplicateRoute(route.id));
            }
            if route.purpose.trim().is_empty() {
                return Err(GeographyValidationError::MissingPurpose(route.id));
            }
            if route.process.len() < 2 || route.process.iter().any(|stage| stage.trim().is_empty())
            {
                return Err(GeographyValidationError::MissingProcessFlow(route.id));
            }
            if !verbs.insert(route.verb) {
                return Err(GeographyValidationError::DuplicateVerb(route.verb));
            }
            if !purposes.insert(route.purpose) {
                return Err(GeographyValidationError::DuplicatePurpose(route.purpose));
            }
            if let ConstitutionalFlowDirection::Directed { from, to } = route.direction
                && (!route.boundary.contains(from) || !route.boundary.contains(to))
            {
                return Err(GeographyValidationError::DirectionOutsideBoundary(route.id));
            }
            if route.id == ConstitutionalRouteId::CurrentSea
                && route.frozen_runtime_projection.is_some()
            {
                return Err(GeographyValidationError::CurrentSeaConflatedWithRuntimeRoute);
            }
            if let Some(key) = route.frozen_runtime_projection
                && !runtime_keys.insert(key)
            {
                return Err(GeographyValidationError::DuplicateRuntimeProjection(key));
            }
        }

        for required in ConstitutionalRouteId::ALL {
            let route = self
                .route(required)
                .ok_or(GeographyValidationError::MissingRoute(required))?;
            if route != &canonical_route_definition(required) {
                return Err(GeographyValidationError::RouteDefinitionMismatch(required));
            }
        }

        let mut boundary_ids = BTreeSet::new();
        for law in &self.boundaries {
            if !boundary_ids.insert(law.boundary) {
                return Err(GeographyValidationError::DuplicateBoundary(law.boundary));
            }
            if law.inward_flow.trim().is_empty() || law.outward_flow.trim().is_empty() {
                return Err(GeographyValidationError::BoundaryFlowMissing(law.boundary));
            }
            let count = self.routes_for_boundary(law.boundary).len();
            if count != 2 {
                return Err(GeographyValidationError::BoundaryRouteCount {
                    boundary: law.boundary,
                    count,
                });
            }
            if law != &canonical_boundary_flow(law.boundary) {
                return Err(GeographyValidationError::BoundaryDefinitionMismatch(
                    law.boundary,
                ));
            }
        }
        for required in HouseBoundary::ALL {
            if !boundary_ids.contains(&required) {
                return Err(GeographyValidationError::MissingBoundary(required));
            }
        }

        Ok(())
    }

    pub fn audit(&self) -> Result<ConstitutionalGeographyAudit, GeographyValidationError> {
        self.validate()?;
        let distinct_purpose_count = self
            .routes
            .iter()
            .map(|route| route.purpose)
            .collect::<BTreeSet<_>>()
            .len();
        let distinct_verb_count = self
            .routes
            .iter()
            .map(|route| route.verb)
            .collect::<BTreeSet<_>>()
            .len();
        let runtime_projection_count = self
            .routes
            .iter()
            .filter_map(|route| route.frozen_runtime_projection)
            .count();
        let distinct_runtime_projection_count = self
            .routes
            .iter()
            .filter_map(|route| route.frozen_runtime_projection)
            .collect::<BTreeSet<_>>()
            .len();
        Ok(ConstitutionalGeographyAudit {
            route_count: self.routes.len(),
            distinct_purpose_count,
            distinct_verb_count,
            duplicate_role_count: self.routes.len().saturating_sub(distinct_verb_count),
            every_route_has_one_purpose: self.routes.iter().all(|route| !route.purpose.is_empty()),
            every_route_has_process_flow: self.routes.iter().all(|route| {
                route.process.len() >= 2 && route.process.iter().all(|stage| !stage.is_empty())
            }),
            every_boundary_has_two_routes: HouseBoundary::ALL
                .iter()
                .all(|boundary| self.routes_for_boundary(*boundary).len() == 2),
            every_boundary_has_inward_and_outward_flow: self
                .boundaries
                .iter()
                .all(|law| !law.inward_flow.is_empty() && !law.outward_flow.is_empty()),
            current_sea_is_distinct_from_current_seanad: self
                .route(ConstitutionalRouteId::CurrentSea)
                .is_some_and(|route| route.frozen_runtime_projection.is_none())
                && self
                    .route(ConstitutionalRouteId::CurrentSeanad)
                    .is_some_and(|route| {
                        route.frozen_runtime_projection
                            == Some(FrozenRuntimeRouteKey::CurrentSeanad)
                    }),
            frozen_runtime_projections_are_unique: runtime_projection_count
                == distinct_runtime_projection_count,
            placements: self
                .routes
                .iter()
                .map(|route| RoutePlacement {
                    route: route.id,
                    boundary: route.boundary,
                    verb: route.verb,
                    frozen_runtime_projection: route.frozen_runtime_projection,
                })
                .collect(),
        })
    }
}

#[must_use]
pub fn canonical_route_definitions() -> Vec<ConstitutionalRouteDefinition> {
    ConstitutionalRouteId::ALL
        .into_iter()
        .map(canonical_route_definition)
        .collect()
}

#[must_use]
pub fn canonical_boundary_flows() -> Vec<BoundaryFlowLaw> {
    HouseBoundary::ALL
        .into_iter()
        .map(canonical_boundary_flow)
        .collect()
}

pub fn canonical_constitutional_geography()
-> Result<ConstitutionalGeography, GeographyValidationError> {
    let geography = ConstitutionalGeography::from_parts(
        canonical_route_definitions(),
        canonical_boundary_flows(),
    );
    geography.validate()?;
    Ok(geography)
}

#[must_use]
pub fn canonical_route_verb_map() -> BTreeMap<&'static str, &'static str> {
    canonical_route_definitions()
        .into_iter()
        .map(|route| (route.id.stable_id(), route.verb.as_str()))
        .collect()
}

fn canonical_route_definition(id: ConstitutionalRouteId) -> ConstitutionalRouteDefinition {
    use ConstitutionalFlowDirection::{Directed, Reciprocal, SharedDeliberation};
    use ConstitutionalRouteVerb::{
        Ascend, Aspire, Certify, Deliberate, Design, Produce, Refine, Retrieve, Return, Witness,
    };
    use FrozenRuntimeRouteKey as Runtime;
    use House::{Flynt, Glaushouse, Stonebend};

    match id {
        ConstitutionalRouteId::Boardwalk => ConstitutionalRouteDefinition {
            id,
            boundary: HouseBoundary::FlyntGlaushouse,
            verb: Return,
            purpose: "constitutional discharge, reintegration, and recognition after recovery",
            direction: Directed {
                from: Glaushouse,
                to: Flynt,
            },
            process: &["Recovery Ward", "Boardwalk", "Flynt", "Recognition"],
            frozen_runtime_projection: Some(Runtime::Boardwalk),
        },
        ConstitutionalRouteId::Riptide => ConstitutionalRouteDefinition {
            id,
            boundary: HouseBoundary::FlyntGlaushouse,
            verb: Retrieve,
            purpose: "involuntary emergency retrieval of damaged beings toward repair",
            direction: Directed {
                from: Flynt,
                to: Glaushouse,
            },
            process: &["Flynt Crisis", "Riptide", "Glaüshouse Intake", "Repair"],
            frozen_runtime_projection: Some(Runtime::Riptide),
        },
        ConstitutionalRouteId::CurrentSea => ConstitutionalRouteDefinition {
            id,
            boundary: HouseBoundary::GlaushouseStonebend,
            verb: Certify,
            purpose: "certification that restoration survives depth and endurance",
            direction: Directed {
                from: Glaushouse,
                to: Stonebend,
            },
            process: &["Repair", "Current Sea", "Stonebend", "Naming", "Title"],
            frozen_runtime_projection: None,
        },
        ConstitutionalRouteId::AuraRidge => ConstitutionalRouteDefinition {
            id,
            boundary: HouseBoundary::GlaushouseStonebend,
            verb: Witness,
            purpose: "public witness, presentation, exchange, and civic reintegration",
            direction: Directed {
                from: Glaushouse,
                to: Stonebend,
            },
            process: &[
                "Repair",
                "Recovery",
                "Aura Ridge",
                "Equal Gaze",
                "Central Junction",
            ],
            frozen_runtime_projection: Some(Runtime::AuraRidge),
        },
        ConstitutionalRouteId::Glausbahn => ConstitutionalRouteDefinition {
            id,
            boundary: HouseBoundary::GlaushouseSandmanor,
            verb: Refine,
            purpose: "high-speed iteration where design and repair improve one another",
            direction: Reciprocal,
            process: &["Design", "Prototype", "Repair", "Improved Design"],
            frozen_runtime_projection: Some(Runtime::Glausbahn),
        },
        ConstitutionalRouteId::CurrentSeanad => ConstitutionalRouteDefinition {
            id,
            boundary: HouseBoundary::GlaushouseSandmanor,
            verb: Deliberate,
            purpose: "constitutional review of difficult design, repair, and Synthesis questions",
            direction: SharedDeliberation,
            process: &[
                "Question",
                "Evidence",
                "Institutional Deliberation",
                "Judgment",
            ],
            frozen_runtime_projection: Some(Runtime::CurrentSeanad),
        },
        ConstitutionalRouteId::AuraWay => ConstitutionalRouteDefinition {
            id,
            boundary: HouseBoundary::StonebendSandmanor,
            verb: Design,
            purpose: "the established constitutional design process",
            direction: Reciprocal,
            process: &["Need", "Design", "Arrangement", "Usable Form"],
            frozen_runtime_projection: Some(Runtime::AuraWay),
        },
        ConstitutionalRouteId::MntAura => ConstitutionalRouteDefinition {
            id,
            boundary: HouseBoundary::StonebendSandmanor,
            verb: Aspire,
            purpose: "the established constitutional path of aspiration",
            direction: Reciprocal,
            process: &[
                "Present Form",
                "Aspiration",
                "Ascent Pressure",
                "Higher Aim",
            ],
            frozen_runtime_projection: Some(Runtime::MntAura),
        },
        ConstitutionalRouteId::BasinMotorspeedway => ConstitutionalRouteDefinition {
            id,
            boundary: HouseBoundary::StonebendFlynt,
            verb: Produce,
            purpose: "the established constitutional production process",
            direction: Reciprocal,
            process: &["Named Plan", "Production", "Field Trial", "Deployable Work"],
            frozen_runtime_projection: Some(Runtime::BasinMotorspeedway),
        },
        ConstitutionalRouteId::StairwayToHeaven => ConstitutionalRouteDefinition {
            id,
            boundary: HouseBoundary::StonebendFlynt,
            verb: Ascend,
            purpose: "the established constitutional path of ascension",
            direction: Directed {
                from: Flynt,
                to: Stonebend,
            },
            process: &["Recognized Capability", "Ascent", "Higher Burden", "Title"],
            frozen_runtime_projection: Some(Runtime::StairwayToHeaven),
        },
    }
}

fn canonical_boundary_flow(boundary: HouseBoundary) -> BoundaryFlowLaw {
    use ConstitutionalRouteId as Route;

    match boundary {
        HouseBoundary::FlyntGlaushouse => BoundaryFlowLaw {
            boundary,
            routes: [Route::Boardwalk, Route::Riptide],
            inward_flow: "Riptide retrieves Flynt crises into Glaüshouse intake and repair",
            outward_flow: "Boardwalk returns discharged beings to Flynt reintegration and recognition",
        },
        HouseBoundary::GlaushouseStonebend => BoundaryFlowLaw {
            boundary,
            routes: [Route::CurrentSea, Route::AuraRidge],
            inward_flow: "restored beings enter either depth certification or public witness",
            outward_flow: "the boundary releases certified title or witnessed civic reintegration",
        },
        HouseBoundary::GlaushouseSandmanor => BoundaryFlowLaw {
            boundary,
            routes: [Route::Glausbahn, Route::CurrentSeanad],
            inward_flow: "designs, repairs, and hard questions enter iteration or review",
            outward_flow: "improved designs, repair knowledge, and constitutional judgments leave",
        },
        HouseBoundary::StonebendSandmanor => BoundaryFlowLaw {
            boundary,
            routes: [Route::AuraWay, Route::MntAura],
            inward_flow: "named needs enter the established design or aspiration path",
            outward_flow: "designed forms and articulated aspirations return to civilization",
        },
        HouseBoundary::StonebendFlynt => BoundaryFlowLaw {
            boundary,
            routes: [Route::BasinMotorspeedway, Route::StairwayToHeaven],
            inward_flow: "named capability enters production or the established ascent path",
            outward_flow: "deployable work and accepted higher burden return to civilization",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_geography_validates() {
        canonical_constitutional_geography().unwrap();
    }

    #[test]
    fn missing_purpose_fails_closed() {
        let mut routes = canonical_route_definitions();
        routes[0].purpose = "";
        let geography = ConstitutionalGeography::from_parts(routes, canonical_boundary_flows());
        assert_eq!(
            geography.validate(),
            Err(GeographyValidationError::MissingPurpose(
                ConstitutionalRouteId::Boardwalk
            ))
        );
    }

    #[test]
    fn duplicate_verb_fails_closed() {
        let mut routes = canonical_route_definitions();
        routes[0].verb = ConstitutionalRouteVerb::Retrieve;
        let geography = ConstitutionalGeography::from_parts(routes, canonical_boundary_flows());
        assert_eq!(
            geography.validate(),
            Err(GeographyValidationError::DuplicateVerb(
                ConstitutionalRouteVerb::Retrieve
            ))
        );
    }

    #[test]
    fn current_sea_cannot_reuse_the_frozen_seanad_projection() {
        let mut routes = canonical_route_definitions();
        routes
            .iter_mut()
            .find(|route| route.id == ConstitutionalRouteId::CurrentSea)
            .unwrap()
            .frozen_runtime_projection = Some(FrozenRuntimeRouteKey::CurrentSeanad);
        let geography = ConstitutionalGeography::from_parts(routes, canonical_boundary_flows());
        assert_eq!(
            geography.validate(),
            Err(GeographyValidationError::CurrentSeaConflatedWithRuntimeRoute)
        );
    }
}
