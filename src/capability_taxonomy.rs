//! Practitioner capability vocabulary above the frozen kernel.
//!
//! `CurrentPowerRoot` and `AuraPowerRoot` are V1 compatibility mechanics. They
//! remain attached to stable capability records for replay, but do not define
//! the semantic Current and Aura axes in the practitioner ontology.

use std::collections::BTreeSet;

use crate::world::hueman_faculties::{FacultyAuthority, HuemanFaculty};
use crate::world::power_recipes::{AuraPowerRoot, CurrentPowerRoot, faculty_power_root};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CapabilityAxis {
    Current,
    Aura,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CapabilityHouse {
    Stonebend,
    Sandmanor,
    Glaushouse,
    Flynt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KinesisFamily {
    Barokinesis,
    Haptokinesis,
    Technokinesis,
    Ideokinesis,
    Biokinesis,
    Noomancy,
    /// Stable V1 spelling retained for old records.
    Precognition,
    Telekinesis,
    Nookinesis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CanonicalPowerRoot {
    Current(CurrentPowerRoot),
    Aura(AuraPowerRoot),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityDefinition {
    pub id: &'static str,
    pub house: CapabilityHouse,
    pub axis: CapabilityAxis,
    pub family: KinesisFamily,
    pub faculty: HuemanFaculty,
    pub canonical_root: CanonicalPowerRoot,
    pub frame: &'static str,
    pub range: &'static str,
    pub load: &'static str,
    pub distortion: &'static str,
    pub recovery: &'static str,
}

pub const STONEBEND_BAROKINESIS: CapabilityDefinition = CapabilityDefinition {
    id: "capability.stonebend.barokinesis",
    house: CapabilityHouse::Stonebend,
    axis: CapabilityAxis::Current,
    family: KinesisFamily::Barokinesis,
    faculty: HuemanFaculty::Presynce,
    canonical_root: CanonicalPowerRoot::Current(CurrentPowerRoot::Somatokinesis),
    frame: "Gremlin→Troglodyte",
    range: "body and physical contact",
    load: "material weight and structural pressure",
    distortion: "overload, rigidity, and Current exhaustion",
    recovery: "rest, repair, and re-circulation",
};

pub const STONEBEND_HAPTOKINESIS: CapabilityDefinition = CapabilityDefinition {
    id: "capability.stonebend.haptokinesis",
    house: CapabilityHouse::Stonebend,
    axis: CapabilityAxis::Aura,
    family: KinesisFamily::Haptokinesis,
    faculty: HuemanFaculty::Presynce,
    canonical_root: CanonicalPowerRoot::Aura(AuraPowerRoot::Psychometry),
    frame: "Gremlin→Troglodyte",
    range: "touch or close structural coupling",
    load: "tension, vibration, and emerging movement",
    distortion: "sensory saturation and boundary confusion",
    recovery: "grounding, separation, and witnessed recalibration",
};

pub const SANDMANOR_TECHNOKINESIS: CapabilityDefinition = CapabilityDefinition {
    id: "capability.sandmanor.technokinesis",
    house: CapabilityHouse::Sandmanor,
    axis: CapabilityAxis::Current,
    family: KinesisFamily::Technokinesis,
    faculty: HuemanFaculty::Prefig,
    canonical_root: CanonicalPowerRoot::Current(CurrentPowerRoot::Shapeshifting),
    frame: "Prefig-compatible Being",
    range: "tool, mechanism, or provisional constructed form",
    load: "material arrangement and demonstrable function",
    distortion: "premature permanence and hidden design assumptions",
    recovery: "versioned revision, proof, and preserved failure",
};

pub const SANDMANOR_IDEOKINESIS: CapabilityDefinition = CapabilityDefinition {
    id: "capability.sandmanor.ideokinesis",
    house: CapabilityHouse::Sandmanor,
    axis: CapabilityAxis::Aura,
    family: KinesisFamily::Ideokinesis,
    faculty: HuemanFaculty::Prefog,
    canonical_root: CanonicalPowerRoot::Aura(AuraPowerRoot::Dreamwalking),
    frame: "Prefog-compatible Being",
    range: "bounded possibility model or conceptual field",
    load: "candidate designs, relations, and unresolved alternatives",
    distortion: "abstraction detached from lived evidence",
    recovery: "criticism, comparison, and return to attributed revision",
};

pub const GLAUSHOUSE_BIOKINESIS: CapabilityDefinition = CapabilityDefinition {
    id: "capability.glaushouse.biokinesis",
    house: CapabilityHouse::Glaushouse,
    axis: CapabilityAxis::Current,
    family: KinesisFamily::Biokinesis,
    faculty: HuemanFaculty::Precog,
    canonical_root: CanonicalPowerRoot::Current(CurrentPowerRoot::Biokinesis),
    frame: "Precog-compatible living Being",
    range: "consenting living tissue under clinical scope",
    load: "adaptation, compatibility, and embodied recovery",
    distortion: "overreach, dependency, and identity strain",
    recovery: "stop authority, care, reassessment, and lawful return",
};

pub const GLAUSHOUSE_NOOMANCY: CapabilityDefinition = CapabilityDefinition {
    id: "capability.glaushouse.precognition",
    house: CapabilityHouse::Glaushouse,
    axis: CapabilityAxis::Aura,
    family: KinesisFamily::Noomancy,
    faculty: HuemanFaculty::Precog,
    canonical_root: CanonicalPowerRoot::Aura(AuraPowerRoot::Clairvoyance),
    frame: "Pixie→(Sprite→Spirit | Fairy→Nymph)→Siren→Muse",
    range: "evidence-grounded probable consequence",
    load: "attention, pattern, uncertainty, and living context",
    distortion: "future fixation and coerced interpretation",
    recovery: "disclosed uncertainty, consent, and present evidence",
};

/// Stable source-level alias for V1 callers. Current canon exposes Noomancy.
#[deprecated(note = "use GLAUSHOUSE_NOOMANCY; the stable V1 ID still decodes")]
pub const GLAUSHOUSE_PRECOGNITION: CapabilityDefinition = GLAUSHOUSE_NOOMANCY;

pub const FLYNT_TELEKINESIS: CapabilityDefinition = CapabilityDefinition {
    id: "capability.flynt.telekinesis",
    house: CapabilityHouse::Flynt,
    axis: CapabilityAxis::Current,
    family: KinesisFamily::Telekinesis,
    faculty: HuemanFaculty::Resynce,
    canonical_root: CanonicalPowerRoot::Current(CurrentPowerRoot::Telekinesis),
    frame: "Gargoyle",
    range: "external line of Aura coupling",
    load: "remote mass and acceleration",
    distortion: "recoil, field break, and Current depletion",
    recovery: "safe release, rest, and pressure normalization",
};

pub const FLYNT_NOOKINESIS: CapabilityDefinition = CapabilityDefinition {
    id: "capability.flynt.nookinesis",
    house: CapabilityHouse::Flynt,
    axis: CapabilityAxis::Aura,
    family: KinesisFamily::Nookinesis,
    faculty: HuemanFaculty::Resynce,
    canonical_root: CanonicalPowerRoot::Aura(AuraPowerRoot::Telepathy),
    frame: "Resynce-compatible Being",
    range: "relational Aura field",
    load: "attention, intention, and group coordination",
    distortion: "resistance, backlash, and relational debt",
    recovery: "consent restoration, disclosure, and return",
};

/// Locked, active House-side expressions. Stonebend's exact axis mappings and
/// additional Sandmanor expression aliases remain open in the practitioner
/// ontology and are not fabricated here.
pub const ACTIVE_CAPABILITIES: &[CapabilityDefinition] = &[
    GLAUSHOUSE_BIOKINESIS,
    GLAUSHOUSE_NOOMANCY,
    FLYNT_TELEKINESIS,
    FLYNT_NOOKINESIS,
];

/// Stable V1 physical/root adapters retained for archive and Recipe replay.
pub const LEGACY_V1_CAPABILITIES: &[CapabilityDefinition] = &[
    STONEBEND_BAROKINESIS,
    STONEBEND_HAPTOKINESIS,
    SANDMANOR_TECHNOKINESIS,
    SANDMANOR_IDEOKINESIS,
];

/// Backward-compatible public name for the active catalog.
pub const CANONICAL_CAPABILITIES: &[CapabilityDefinition] = ACTIVE_CAPABILITIES;

pub const PILOT_CAPABILITIES: &[CapabilityDefinition] = &[
    STONEBEND_BAROKINESIS,
    STONEBEND_HAPTOKINESIS,
    FLYNT_TELEKINESIS,
];

pub fn validate_capabilities(capabilities: &[CapabilityDefinition]) -> Result<(), &'static str> {
    let mut ids = BTreeSet::new();
    let mut pairs = BTreeSet::new();
    for capability in capabilities {
        if capability.id.is_empty()
            || !capability.id.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-')
            })
            || !ids.insert(capability.id)
        {
            return Err("capability IDs must be unique stable identifiers");
        }
        if !pairs.insert((capability.house, capability.axis)) {
            return Err("each House may define only one capability per axis");
        }
        let root = faculty_power_root(capability.faculty);
        let expected_authority = match capability.house {
            CapabilityHouse::Stonebend => FacultyAuthority::Stonebend,
            CapabilityHouse::Sandmanor => match capability.faculty {
                HuemanFaculty::Prefog => FacultyAuthority::SandmanorMinorian,
                HuemanFaculty::Prefig => FacultyAuthority::SandmanorMinoan,
                _ => return Err("Sandmanor capability uses a foreign faculty"),
            },
            CapabilityHouse::Glaushouse => FacultyAuthority::Glaushouse,
            CapabilityHouse::Flynt => FacultyAuthority::Flynt,
        };
        if root.authority != expected_authority {
            return Err("capability House differs from canonical faculty authority");
        }
        let expected_root = match capability.axis {
            CapabilityAxis::Current => CanonicalPowerRoot::Current(root.current),
            CapabilityAxis::Aura => CanonicalPowerRoot::Aura(root.aura),
        };
        if capability.canonical_root != expected_root {
            return Err("capability differs from its canonical power root");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pilot_keeps_current_and_aura_house_boundaries() {
        validate_capabilities(CANONICAL_CAPABILITIES).unwrap();
        assert_eq!(CANONICAL_CAPABILITIES.len(), 4);
        assert_eq!(FLYNT_TELEKINESIS.house, CapabilityHouse::Flynt);
        assert_eq!(FLYNT_NOOKINESIS.family, KinesisFamily::Nookinesis);
        assert_eq!(GLAUSHOUSE_NOOMANCY.family, KinesisFamily::Noomancy);
        assert_eq!(LEGACY_V1_CAPABILITIES.len(), 4);
        assert_eq!(STONEBEND_BAROKINESIS.frame, "Gremlin→Troglodyte");
        assert_eq!(
            FLYNT_TELEKINESIS.canonical_root,
            CanonicalPowerRoot::Current(CurrentPowerRoot::Telekinesis)
        );
        assert_eq!(
            GLAUSHOUSE_NOOMANCY.canonical_root,
            CanonicalPowerRoot::Aura(AuraPowerRoot::Clairvoyance)
        );
    }

    #[test]
    fn validation_rejects_parallel_or_foreign_power_roots() {
        let mut wrong_root = FLYNT_TELEKINESIS;
        wrong_root.canonical_root = CanonicalPowerRoot::Current(CurrentPowerRoot::Somatokinesis);
        assert_eq!(
            validate_capabilities(&[wrong_root]),
            Err("capability differs from its canonical power root")
        );

        let mut foreign_faculty = FLYNT_TELEKINESIS;
        foreign_faculty.faculty = HuemanFaculty::Presynce;
        foreign_faculty.canonical_root =
            CanonicalPowerRoot::Current(CurrentPowerRoot::Somatokinesis);
        assert_eq!(
            validate_capabilities(&[foreign_faculty]),
            Err("capability House differs from canonical faculty authority")
        );
    }
}
