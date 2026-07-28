//! Executable lived-lore integration above the frozen constitutional kernel.
//!
//! The Compromise requires lore to identify its authority, evidence, player
//! choice, lawful transition, persistence, presentation, and refusal path.
//! This module supplies that contract for every House. It reads institutional
//! authority and constitutional geography; it never changes recursion,
//! manufactures an office holder, or lets one House perform another's act.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::{
    CausalPosition, HouseDecision, HouseFunction, scenario_house_decision,
};
use crate::hollow_grove_contract::House;

use super::geography::{ConstitutionalRouteId, canonical_constitutional_geography};
use super::session::WorldSession;

pub const FUNCTIONAL_LORE_ARCHIVE_FORMAT: &str = "hollow-grove-functional-lore";
pub const FUNCTIONAL_LORE_ARCHIVE_SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionalLoreDefinition {
    /// 1. Stable identity.
    pub id: &'static str,
    pub house: House,
    /// 2. Authority class.
    pub authority_function: HouseFunction,
    pub authority_class: &'static str,
    /// 3. Location and jurisdiction.
    pub route: ConstitutionalRouteId,
    pub location: &'static str,
    pub jurisdiction: &'static str,
    /// 4. Involved entities.
    pub entities: &'static [&'static str],
    /// 5. Dominant House verb.
    pub dominant_verb: &'static str,
    /// 6. Trigger.
    pub trigger: &'static str,
    /// 7. Evidence and uncertainty.
    pub evidence: &'static [&'static str],
    pub uncertainty: &'static [&'static str],
    /// 8. Player-visible choices.
    pub player_choices: &'static [&'static str],
    /// 9. Lawful state changes, positionally paired with player choices.
    pub lawful_state_changes: &'static [&'static str],
    /// 10. Persistence and replay.
    pub persistence_and_replay: &'static str,
    /// 11. Presentation.
    pub presentation: &'static str,
    /// 12. Failure and refusal.
    pub failure_and_refusal: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionalLoreRecord {
    pub definition: FunctionalLoreDefinition,
    pub authority: HouseDecision,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionalLoreCatalog {
    observed_at: CausalPosition,
    records: Vec<FunctionalLoreRecord>,
}

impl FunctionalLoreCatalog {
    pub fn instantiate(
        world: &WorldSession,
        observed_at: CausalPosition,
    ) -> Result<Self, FunctionalLoreError> {
        let definitions = canonical_functional_lore_definitions();
        validate_definitions(&definitions)?;
        let records = definitions
            .into_iter()
            .map(|definition| {
                let authority = scenario_house_decision(
                    &world.institutional().catalog,
                    definition.id,
                    definition.authority_function,
                    observed_at.get(),
                )
                .map_err(|error| FunctionalLoreError::Authority {
                    id: definition.id,
                    detail: error.to_string(),
                })?;
                if authority.authority.house != definition.house {
                    return Err(FunctionalLoreError::HouseSubstitution(definition.id));
                }
                Ok(FunctionalLoreRecord {
                    definition,
                    authority,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            observed_at,
            records,
        })
    }

    #[must_use]
    pub const fn observed_at(&self) -> CausalPosition {
        self.observed_at
    }

    #[must_use]
    pub fn records(&self) -> &[FunctionalLoreRecord] {
        &self.records
    }

    #[must_use]
    pub fn records_for_house(&self, house: House) -> Vec<&FunctionalLoreRecord> {
        self.records
            .iter()
            .filter(|record| record.definition.house == house)
            .collect()
    }

    pub fn encode(&self) -> Result<String, FunctionalLoreError> {
        let payload = FunctionalLoreArchivePayload {
            observed_at: self.observed_at.get(),
            records: self
                .records
                .iter()
                .map(WireFunctionalLoreRecord::from)
                .collect(),
        };
        let checksum = checksum_for(&payload)?;
        serde_json::to_string_pretty(&FunctionalLoreArchiveEnvelope {
            format: FUNCTIONAL_LORE_ARCHIVE_FORMAT.into(),
            schema_version: FUNCTIONAL_LORE_ARCHIVE_SCHEMA_VERSION,
            checksum,
            payload,
        })
        .map_err(|error| FunctionalLoreError::Serialization(error.to_string()))
    }

    pub fn replay(encoded: &str, world: &WorldSession) -> Result<Self, FunctionalLoreError> {
        let envelope: FunctionalLoreArchiveEnvelope = serde_json::from_str(encoded)
            .map_err(|error| FunctionalLoreError::Serialization(error.to_string()))?;
        if envelope.format != FUNCTIONAL_LORE_ARCHIVE_FORMAT {
            return Err(FunctionalLoreError::UnsupportedFormat(envelope.format));
        }
        if envelope.schema_version != FUNCTIONAL_LORE_ARCHIVE_SCHEMA_VERSION {
            return Err(FunctionalLoreError::UnsupportedSchema(
                envelope.schema_version,
            ));
        }
        let actual_checksum = checksum_for(&envelope.payload)?;
        if envelope.checksum != actual_checksum {
            return Err(FunctionalLoreError::ChecksumMismatch);
        }
        let replayed = Self::instantiate(world, CausalPosition::new(envelope.payload.observed_at))?;
        let actual = replayed
            .records
            .iter()
            .map(WireFunctionalLoreRecord::from)
            .collect::<Vec<_>>();
        if actual != envelope.payload.records {
            return Err(FunctionalLoreError::ReplayDivergence);
        }
        Ok(replayed)
    }

    #[must_use]
    pub fn witness_markdown(&self) -> String {
        let mut output = String::from(
            "# Hollow Grove Functional Lore Witness\n\n\
             Every entry carries all twelve fields required by the Hollow Grove Compromise. \
             House authority is snapshotted from the live institutional session.\n",
        );
        for record in &self.records {
            let definition = &record.definition;
            output.push_str(&format!(
                "\n## {}\n\n\
                 - House act: {} / {}\n\
                 - Authority actor: `{}` via `{}`\n\
                 - Route: `{}`\n\
                 - Location: {}\n\
                 - Trigger: {}\n\
                 - Choice count: {}\n\
                 - Failure/refusal count: {}\n\
                 - Persistence: {}\n",
                definition.id,
                definition.dominant_verb,
                definition.authority_class,
                record.authority.authority.actor.as_str(),
                record.authority.authority.office.as_str(),
                definition.route.stable_id(),
                definition.location,
                definition.trigger,
                definition.player_choices.len(),
                definition.failure_and_refusal.len(),
                definition.persistence_and_replay,
            ));
        }
        output
    }
}

pub fn validate_definitions(
    definitions: &[FunctionalLoreDefinition],
) -> Result<(), FunctionalLoreError> {
    let geography = canonical_constitutional_geography()
        .map_err(|error| FunctionalLoreError::Geography(error.to_string()))?;
    let mut identities = BTreeSet::new();
    let mut routes = BTreeSet::new();
    let mut house_counts = [0_usize; 4];
    for definition in definitions {
        if !stable_id(definition.id) || !identities.insert(definition.id) {
            return Err(FunctionalLoreError::InvalidIdentity(definition.id));
        }
        let expected_house = definition.authority_function.constitutional_house();
        if definition.house != expected_house {
            return Err(FunctionalLoreError::HouseSubstitution(definition.id));
        }
        if definition.authority_class != definition.authority_function.required_authority() {
            return Err(FunctionalLoreError::AuthorityClassMismatch(definition.id));
        }
        if definition.dominant_verb != house_verb(definition.house) {
            return Err(FunctionalLoreError::DominantVerbMismatch(definition.id));
        }
        let route = geography
            .route(definition.route)
            .ok_or(FunctionalLoreError::RouteMissing(definition.id))?;
        if !route.boundary.contains(definition.house) {
            return Err(FunctionalLoreError::JurisdictionMismatch(definition.id));
        }
        routes.insert(definition.route);
        house_counts[house_index(definition.house)] += 1;
        let required_text = [
            definition.location,
            definition.jurisdiction,
            definition.trigger,
            definition.persistence_and_replay,
            definition.presentation,
        ];
        if required_text.iter().any(|value| value.trim().is_empty())
            || definition.entities.is_empty()
            || definition.evidence.is_empty()
            || definition.uncertainty.is_empty()
            || definition.player_choices.len() < 2
            || definition.player_choices.len() != definition.lawful_state_changes.len()
            || definition.failure_and_refusal.is_empty()
        {
            return Err(FunctionalLoreError::IncompleteContract(definition.id));
        }
    }
    if house_counts.iter().any(|count| *count < 3) {
        return Err(FunctionalLoreError::IncompleteHouseCoverage);
    }
    if !ConstitutionalRouteId::ALL
        .iter()
        .all(|route| routes.contains(route))
    {
        return Err(FunctionalLoreError::IncompleteRouteCoverage);
    }
    Ok(())
}

#[must_use]
pub fn canonical_functional_lore_definitions() -> Vec<FunctionalLoreDefinition> {
    use ConstitutionalRouteId::{
        AuraRidge, AuraWay, BasinMotorspeedway, Boardwalk, CurrentSea, CurrentSeanad, Glausbahn,
        MntAura, Riptide, StairwayToHeaven,
    };
    use House::{Flynt, Glaushouse, Sandmanor, Stonebend};
    vec![
        definition(
            "lore.flynt.boardwalk-return-recognition",
            Flynt,
            HouseFunction::Recognize,
            Boardwalk,
            "Boardwalk return vestibule",
            "Flynt–Glaüshouse discharge and civic-return boundary",
            &[
                "Returning resident",
                "Tross recognition record",
                "Boardwalk witnesses",
            ],
            "recovery is complete enough for voluntary public return",
            &[
                "discharge record",
                "identity continuity",
                "declared affiliation boundary",
            ],
            &[
                "future role remains open",
                "recognition does not prove competence",
            ],
            &[
                "seek finite affiliation",
                "accept scoped work",
                "return independently",
            ],
            &[
                "recognized affiliation review opens",
                "only the disclosed work scope opens",
                "civic return is recognized without affiliation",
            ],
            "choice event and Flynt recognition decision replay together",
            "public return lane, open exits, and visible non-ownership language",
            &[
                "resident may refuse every affiliation",
                "recognition cannot become Title or custody",
            ],
        ),
        definition(
            "lore.flynt.stairway-functional-recognition",
            Flynt,
            HouseFunction::Recognize,
            StairwayToHeaven,
            "lower Stairway to Heaven and Gallowry approach",
            "Stonebend–Flynt ascent from performed function toward named standing",
            &[
                "Gallows crew",
                "independent worker",
                "Stonebend registry observer",
            ],
            "a worker demonstrates a stable public function without claiming office",
            &[
                "witnessed performance",
                "disclosed crew scope",
                "prior recognition history",
            ],
            &[
                "popular acclaim may exceed proven scope",
                "Stonebend Title remains separate",
            ],
            &[
                "request recognition",
                "remain an unrecognized contractor",
                "challenge crew attribution",
            ],
            &[
                "lawful function enters Flynt recognition history",
                "work remains valid without institutional recognition",
                "attribution enters review without retaliatory loss of work",
            ],
            "performance witnesses, decision, and challenge remain ordered",
            "the ascent changes banners only after recognition; no crown appears",
            &[
                "crew cannot manufacture office",
                "refusal cannot erase completed work",
            ],
        ),
        definition(
            "lore.flynt.basin-dual-expression",
            Flynt,
            HouseFunction::Recognize,
            BasinMotorspeedway,
            "Basin Motor Speedway trade apron",
            "Stonebend production route meeting Flynt urban and rural expression",
            &[
                "Manticorp delegate",
                "Gallows delegate",
                "independent carrier",
            ],
            "urban and rural institutions claim different lawful uses for one delivery",
            &[
                "cargo provenance",
                "delivery performance",
                "separate institutional mandates",
            ],
            &[
                "dual expression may look like duplicate sovereignty",
                "contraband status may remain disputed",
            ],
            &[
                "recognize urban function",
                "recognize rural function",
                "recognize neither pending review",
            ],
            &[
                "Manticorp function is recognized within scope",
                "Gallows function is recognized within scope",
                "cargo is held without invented recognition",
            ],
            "each recognition keeps its institution, scope, and evidence",
            "parallel urban and rural signs remain distinct beneath one Tross mark",
            &[
                "one expression cannot erase the other",
                "Tross sovereignty cannot be duplicated",
            ],
        ),
        definition(
            "lore.stonebend.current-sea-continuity",
            Stonebend,
            HouseFunction::Name,
            CurrentSea,
            "Current Sea Many-Witness Concourse",
            "Glaüshouse restoration entering Stonebend identity jurisdiction",
            &[
                "restored being",
                "Gerald registrar",
                "Mercury Mirror record",
            ],
            "a restored subject enters dense public life while bearing materially changed form",
            &[
                "pre-treatment Name",
                "restoration record",
                "public-circulation continuity witness",
            ],
            &[
                "continuity may be partial",
                "clinical success does not settle identity",
            ],
            &[
                "affirm existing Name",
                "issue provisional transformed-form Name",
                "refer identity conflict",
            ],
            &[
                "existing continuity is sealed",
                "a provisional Name preserves both forms",
                "no final Name acts until high review",
            ],
            "Name history, former form, evidence, and Seal sequence replay",
            "old and present silhouettes remain linked rather than overwritten",
            &[
                "subject may challenge substitution",
                "provisional status cannot grant Title",
            ],
        ),
        definition(
            "lore.stonebend.aura-ridge-public-name",
            Stonebend,
            HouseFunction::Name,
            AuraRidge,
            "Central Junction public-name desk",
            "public witness route from recovery toward civic continuity",
            &[
                "returning citizen",
                "public witnesses",
                "Proliteriate hearing clerk",
            ],
            "a citizen asks the exchange to use a changed civic Name",
            &[
                "personal petition",
                "former Name history",
                "conflict search",
            ],
            &[
                "public familiarity may lag",
                "shared language may cause ambiguity",
            ],
            &[
                "open provisional Name",
                "retain current Name",
                "challenge a conflicting claim",
            ],
            &[
                "provisional public reference begins",
                "current record remains active",
                "conflict enters hearing without either claimant disappearing",
            ],
            "petition, notice, challenge window, and rename history persist",
            "both former and provisional Names are visible in the registry panel",
            &[
                "renaming cannot erase debt or history",
                "witness does not create ownership",
            ],
        ),
        definition(
            "lore.stonebend.mnt-aura-illegal-hollowing",
            Stonebend,
            HouseFunction::Name,
            MntAura,
            "Mt. Aura extraction refuge",
            "Stonebend–Sandmanor aspiration route under Hollowing and provenance law",
            &[
                "injured structure-being",
                "Freemason examiner",
                "protected Hollow sample",
            ],
            "an interior breach reveals extracted Hollow with missing custody",
            &[
                "subject identity",
                "breach survey",
                "material provenance gap",
            ],
            &[
                "operator intent is unknown",
                "restoration feasibility is unsettled",
            ],
            &[
                "seal emergency custody",
                "halt and investigate",
                "refuse destructive inspection",
            ],
            &[
                "subject and sample receive distinct protected Names",
                "possible Illegal Hollowing enters review",
                "the subject remains intact while non-destructive evidence is sought",
            ],
            "survey, custody, former condition, and every Seal remain append-only",
            "cutaway view distinguishes subject, Hollow, and Hollowed without gore-as-proof",
            &[
                "emergency cannot excuse research",
                "refusal cannot be relabeled incapacity",
            ],
        ),
        definition(
            "lore.sandmanor.aura-way-public-design",
            Sandmanor,
            HouseFunction::Prove,
            AuraWay,
            "Aura Way civic prototype court",
            "Stonebend–Sandmanor design route",
            &[
                "Minorian interior team",
                "Minoan exterior team",
                "ordinary route users",
            ],
            "a public shelter prototype claims to improve both interior use and exterior access",
            &[
                "versioned design",
                "measured demonstration",
                "user testimony",
            ],
            &[
                "seasonal performance is incomplete",
                "maintenance cost remains estimated",
            ],
            &[
                "approve proof within scope",
                "require revision",
                "reject the claim while preserving failure",
            ],
            &[
                "claim becomes proven only for tested conditions",
                "design returns to a new attributed version",
                "failure record stays visible and no proof is granted",
            ],
            "method, raw measures, criticism, failure, and revisions replay by version",
            "players walk both inside and around the same prototype before judging",
            &[
                "critics may refuse endorsement",
                "negative results cannot be deleted",
            ],
        ),
        definition(
            "lore.sandmanor.glausbahn-recovery-design",
            Sandmanor,
            HouseFunction::Prove,
            Glausbahn,
            "Glausbahn rolling rehabilitation laboratory",
            "Glaüshouse–Sandmanor reciprocal refinement route",
            &[
                "recovery patient",
                "Gnome method keeper",
                "Glaüshouse therapist",
            ],
            "a mobility design works in the laboratory but fails during moving-route recovery",
            &[
                "prototype telemetry",
                "patient report",
                "failed field demonstration",
            ],
            &[
                "failure cause is mixed",
                "clinical tolerance varies by subject",
            ],
            &["revise the design", "narrow the claim", "stop testing"],
            &[
                "a successor version opens without inherited proof",
                "proof scope contracts to laboratory conditions",
                "testing closes while recovery duties continue",
            ],
            "failed run and clinical stop precede every successor proof",
            "the route visibly slows and exposes the failed mechanism for inspection",
            &[
                "patient refusal immediately stops participation",
                "Sandmanor proof cannot override clinical clearance",
            ],
        ),
        definition(
            "lore.sandmanor.aura-beach-current-sea-role-proof",
            Sandmanor,
            HouseFunction::Prove,
            AuraWay,
            "Aura Beach and Current Sea exterior trial",
            "Sandmanor regional design jurisdiction linked through Aura Way review",
            &["Elf candidate", "Centaur mentor", "coastal access users"],
            "an Elf-to-Centaur regional role claims safer coastal rescue performance",
            &[
                "recipe lineage",
                "coastal demonstration",
                "independent reproduction",
            ],
            &[
                "weather range remains narrow",
                "role proof is not Synthesis clearance",
            ],
            &[
                "prove regional role",
                "require wider reproduction",
                "preserve Elf practice without transition",
            ],
            &[
                "role proof becomes eligible for separate House acts",
                "claim remains provisional",
                "existing practice keeps equal standing",
            ],
            "recipe, reproduction, failure, and scope limits remain attributable",
            "shoreline navigation and Current Sea pressure are shown as separate tests",
            &[
                "transition is never automatic",
                "proof grants neither consent nor office",
            ],
        ),
        definition(
            "lore.glaushouse.riptide-emergency-intake",
            Glaushouse,
            HouseFunction::Clear,
            Riptide,
            "Riptide emergency intake",
            "Flynt–Glaüshouse retrieval boundary",
            &["retrieved patient", "Nightingale", "emergency operator"],
            "Riptide delivers a damaged being whose wishes and exact injury are not yet known",
            &[
                "retrieval record",
                "rapid diagnosis",
                "known directive search",
            ],
            &[
                "capacity is temporarily uncertain",
                "full procedure need is unproven",
            ],
            &[
                "accept stabilization",
                "refuse non-emergency intervention",
                "invoke Nightingale stop",
            ],
            &[
                "narrow emergency clearance opens",
                "care continues without elective procedure",
                "procedure halts and mandatory review opens",
            ],
            "diagnosis, consent scope, stop, intervention, and recovery remain ordered",
            "emergency urgency is visible beside a hard boundary around elective actions",
            &[
                "silence never becomes consent",
                "retrieval custody never becomes ownership",
            ],
        ),
        definition(
            "lore.glaushouse.current-seanad-high-risk-clearance",
            Glaushouse,
            HouseFunction::Clear,
            CurrentSeanad,
            "Current Seanad high-risk chamber",
            "Glaüshouse–Sandmanor shared deliberation route",
            &[
                "procedure subject",
                "Prima Donna authority",
                "Sandmanor proof body",
            ],
            "a proven design is proposed for irreversible living-subject Synthesis",
            &[
                "separate proof judgment",
                "specific consent",
                "recovery and rescue plan",
            ],
            &[
                "identity result remains uncertain",
                "reversal may be partial or impossible",
            ],
            &[
                "grant conditional clearance",
                "seek a reversible alternative",
                "refuse Synthesis",
            ],
            &[
                "clearance opens only under listed conditions",
                "proposal returns to design without procedure authority",
                "no Synthesis begins and care continues",
            ],
            "proof, consent, clearance, deviations, and recovery are separately replayable",
            "the interface never merges Proof, Consent, and Clearance into one meter",
            &[
                "proof cannot compel participation",
                "refusal is not incapacity",
            ],
        ),
        definition(
            "lore.glaushouse.aura-ridge-recovery-discharge",
            Glaushouse,
            HouseFunction::Clear,
            AuraRidge,
            "Aura Ridge recovery transfer",
            "Glaüshouse–Stonebend public reintegration route carrying adapted patients toward ordinary work",
            &[
                "recovering worker",
                "Persephone service",
                "worksite receiver",
            ],
            "a recovering worker can travel but the receiving worksite requests premature full duty",
            &["recovery assessment", "travel tolerance", "worksite demand"],
            &[
                "endurance at full load remains unknown",
                "economic pressure may distort consent",
            ],
            &[
                "clear limited transfer",
                "continue recovery",
                "refuse the receiving conditions",
            ],
            &[
                "travel opens with explicit restrictions",
                "clinical custody and support continue",
                "transfer does not occur and no standing is lost",
            ],
            "restrictions, reassessment date, discharge status, and refusal persist",
            "route access and work clearance appear as visibly separate gates",
            &[
                "premature discharge may be stopped",
                "livelihood dependence cannot manufacture consent",
            ],
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
const fn definition(
    id: &'static str,
    house: House,
    authority_function: HouseFunction,
    route: ConstitutionalRouteId,
    location: &'static str,
    jurisdiction: &'static str,
    entities: &'static [&'static str],
    trigger: &'static str,
    evidence: &'static [&'static str],
    uncertainty: &'static [&'static str],
    player_choices: &'static [&'static str],
    lawful_state_changes: &'static [&'static str],
    persistence_and_replay: &'static str,
    presentation: &'static str,
    failure_and_refusal: &'static [&'static str],
) -> FunctionalLoreDefinition {
    FunctionalLoreDefinition {
        id,
        house,
        authority_function,
        authority_class: authority_function.required_authority(),
        route,
        location,
        jurisdiction,
        entities,
        dominant_verb: house_verb(house),
        trigger,
        evidence,
        uncertainty,
        player_choices,
        lawful_state_changes,
        persistence_and_replay,
        presentation,
        failure_and_refusal,
    }
}

const fn house_verb(house: House) -> &'static str {
    match house {
        House::Stonebend => "Name",
        House::Sandmanor => "Prove",
        House::Glaushouse => "Clear",
        House::Flynt => "Recognize",
    }
}

const fn house_index(house: House) -> usize {
    match house {
        House::Stonebend => 0,
        House::Sandmanor => 1,
        House::Glaushouse => 2,
        House::Flynt => 3,
    }
}

fn stable_id(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-')
        })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FunctionalLoreArchiveEnvelope {
    format: String,
    schema_version: u16,
    checksum: String,
    payload: FunctionalLoreArchivePayload,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FunctionalLoreArchivePayload {
    observed_at: u64,
    records: Vec<WireFunctionalLoreRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireFunctionalLoreRecord {
    id: String,
    house: String,
    authority_function: String,
    authority_class: String,
    authority_decision_id: String,
    authority_actor: String,
    authority_office: String,
    route: String,
    location: String,
    jurisdiction: String,
    entities: Vec<String>,
    dominant_verb: String,
    trigger: String,
    evidence: Vec<String>,
    uncertainty: Vec<String>,
    player_choices: Vec<String>,
    lawful_state_changes: Vec<String>,
    persistence_and_replay: String,
    presentation: String,
    failure_and_refusal: Vec<String>,
}

impl From<&FunctionalLoreRecord> for WireFunctionalLoreRecord {
    fn from(record: &FunctionalLoreRecord) -> Self {
        let definition = &record.definition;
        Self {
            id: definition.id.into(),
            house: house_id(definition.house).into(),
            authority_function: definition.dominant_verb.into(),
            authority_class: definition.authority_class.into(),
            authority_decision_id: record.authority.id.as_str().into(),
            authority_actor: record.authority.authority.actor.as_str().into(),
            authority_office: record.authority.authority.office.as_str().into(),
            route: definition.route.stable_id().into(),
            location: definition.location.into(),
            jurisdiction: definition.jurisdiction.into(),
            entities: strings(definition.entities),
            dominant_verb: definition.dominant_verb.into(),
            trigger: definition.trigger.into(),
            evidence: strings(definition.evidence),
            uncertainty: strings(definition.uncertainty),
            player_choices: strings(definition.player_choices),
            lawful_state_changes: strings(definition.lawful_state_changes),
            persistence_and_replay: definition.persistence_and_replay.into(),
            presentation: definition.presentation.into(),
            failure_and_refusal: strings(definition.failure_and_refusal),
        }
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).into()).collect()
}

const fn house_id(house: House) -> &'static str {
    match house {
        House::Stonebend => "house.stonebend",
        House::Sandmanor => "house.sandmanor",
        House::Glaushouse => "house.glaushouse",
        House::Flynt => "house.flynt",
    }
}

fn checksum_for(payload: &impl Serialize) -> Result<String, FunctionalLoreError> {
    let bytes = serde_json::to_vec(payload)
        .map_err(|error| FunctionalLoreError::Serialization(error.to_string()))?;
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64:{hash:016x}"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionalLoreError {
    InvalidIdentity(&'static str),
    HouseSubstitution(&'static str),
    AuthorityClassMismatch(&'static str),
    DominantVerbMismatch(&'static str),
    RouteMissing(&'static str),
    JurisdictionMismatch(&'static str),
    IncompleteContract(&'static str),
    IncompleteHouseCoverage,
    IncompleteRouteCoverage,
    Geography(String),
    Authority { id: &'static str, detail: String },
    Serialization(String),
    UnsupportedFormat(String),
    UnsupportedSchema(u16),
    ChecksumMismatch,
    ReplayDivergence,
}

impl fmt::Display for FunctionalLoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "functional lore rejected: {self:?}")
    }
}

impl std::error::Error for FunctionalLoreError {}
