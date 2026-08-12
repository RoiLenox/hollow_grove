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
pub const FUNCTIONAL_LORE_ARCHIVE_SCHEMA_VERSION: u16 = 2;

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
    /// Operational role envelope carried by schema V2.
    pub affected_actor: &'static str,
    pub decision_owner: &'static str,
    pub executor: &'static str,
    pub witness: &'static str,
    pub ordered_handoffs: &'static [&'static str],
    pub completion_condition: &'static str,
    pub restitution_owner: &'static str,
    pub recipient: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OperationalRoleEnvelope {
    affected_actor: &'static str,
    decision_owner: &'static str,
    executor: &'static str,
    witness: &'static str,
    ordered_handoffs: &'static [&'static str],
    completion_condition: &'static str,
    restitution_owner: &'static str,
    recipient: &'static str,
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
        let header: FunctionalLoreArchiveHeader = serde_json::from_str(encoded)
            .map_err(|error| FunctionalLoreError::Serialization(error.to_string()))?;
        let (observed_at, records) = match header.schema_version {
            1 => {
                let envelope: FunctionalLoreArchiveEnvelopeV1 = serde_json::from_str(encoded)
                    .map_err(|error| FunctionalLoreError::Serialization(error.to_string()))?;
                validate_archive_header(&envelope.format, envelope.schema_version)?;
                if envelope.checksum != checksum_for(&envelope.payload)? {
                    return Err(FunctionalLoreError::ChecksumMismatch);
                }
                let mut records = envelope
                    .payload
                    .records
                    .into_iter()
                    .map(WireFunctionalLoreRecord::from)
                    .collect::<Vec<_>>();
                normalize_v1_role_envelope(&mut records)?;
                (envelope.payload.observed_at, records)
            }
            FUNCTIONAL_LORE_ARCHIVE_SCHEMA_VERSION => {
                let envelope: FunctionalLoreArchiveEnvelope = serde_json::from_str(encoded)
                    .map_err(|error| FunctionalLoreError::Serialization(error.to_string()))?;
                validate_archive_header(&envelope.format, envelope.schema_version)?;
                if envelope.checksum != checksum_for(&envelope.payload)? {
                    return Err(FunctionalLoreError::ChecksumMismatch);
                }
                (envelope.payload.observed_at, envelope.payload.records)
            }
            schema_version => return Err(FunctionalLoreError::UnsupportedSchema(schema_version)),
        };
        let replayed = Self::instantiate(world, CausalPosition::new(observed_at))?;
        let actual = replayed
            .records
            .iter()
            .map(WireFunctionalLoreRecord::from)
            .collect::<Vec<_>>();
        if actual != records {
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
                 - Roles: affected `{}`, decision `{}`, executor `{}`, witness `{}`, recipient `{}`\n\
                 - Handoffs: {}\n\
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
                definition.affected_actor,
                definition.decision_owner,
                definition.executor,
                definition.witness,
                definition.recipient,
                definition.ordered_handoffs.join(", "),
                definition.persistence_and_replay,
            ));
        }
        output
    }
}

fn validate_archive_header(format: &str, schema_version: u16) -> Result<(), FunctionalLoreError> {
    if format != FUNCTIONAL_LORE_ARCHIVE_FORMAT {
        return Err(FunctionalLoreError::UnsupportedFormat(format.into()));
    }
    if !matches!(schema_version, 1 | FUNCTIONAL_LORE_ARCHIVE_SCHEMA_VERSION) {
        return Err(FunctionalLoreError::UnsupportedSchema(schema_version));
    }
    Ok(())
}

fn normalize_v1_role_envelope(
    records: &mut [WireFunctionalLoreRecord],
) -> Result<(), FunctionalLoreError> {
    let definitions = canonical_functional_lore_definitions();
    for record in records {
        let definition = definitions
            .iter()
            .find(|definition| definition.id == record.id)
            .ok_or(FunctionalLoreError::ReplayDivergence)?;
        record.affected_actor = definition.affected_actor.into();
        record.decision_owner = definition.decision_owner.into();
        record.executor = definition.executor.into();
        record.witness = definition.witness.into();
        record.ordered_handoffs = strings(definition.ordered_handoffs);
        record.completion_condition = definition.completion_condition.into();
        record.restitution_owner = definition.restitution_owner.into();
        record.recipient = definition.recipient.into();
    }
    Ok(())
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
            definition.affected_actor,
            definition.decision_owner,
            definition.executor,
            definition.witness,
            definition.completion_condition,
            definition.restitution_owner,
            definition.recipient,
        ];
        let handoff_chain = definition.ordered_handoffs.join(" | ");
        if required_text.iter().any(|value| value.trim().is_empty())
            || definition.entities.is_empty()
            || definition.evidence.is_empty()
            || definition.uncertainty.is_empty()
            || definition.player_choices.len() < 2
            || definition.player_choices.len() != definition.lawful_state_changes.len()
            || definition.ordered_handoffs.len() < 2
            || definition
                .ordered_handoffs
                .iter()
                .any(|handoff| handoff.trim().is_empty() || !handoff.contains('→'))
            || [
                definition.decision_owner,
                definition.executor,
                definition.witness,
                definition.recipient,
            ]
            .iter()
            .any(|role| !handoff_chain.contains(role))
            || definition.decision_owner != house_decision_office(definition.house)
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
            roles(
                "Returning resident",
                "office.flynt.tross",
                "Boardwalk return steward",
                "Boardwalk witnesses",
                &[
                    "office.flynt.tross→Boardwalk return steward: scoped recognition decision",
                    "Boardwalk return steward→Boardwalk witnesses: public recognition record",
                    "Boardwalk witnesses→Returning resident: voluntary return receipt",
                ],
                "the resident receives scoped recognition with every exit still open",
                "office.flynt.tross",
                "Returning resident",
            ),
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
            roles(
                "independent worker",
                "office.flynt.tross",
                "Gallows crew",
                "Stonebend registry observer",
                &[
                    "office.flynt.tross→Gallows crew: bounded function decision",
                    "Gallows crew→Stonebend registry observer: attributed performance record",
                    "Stonebend registry observer→independent worker: scoped standing receipt",
                ],
                "the worker receives an attributable scope without office or coerced affiliation",
                "office.flynt.tross",
                "independent worker",
            ),
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
            roles(
                "independent carrier",
                "office.flynt.tross",
                "independent carrier",
                "Manticorp and Gallows delegates",
                &[
                    "office.flynt.tross→independent carrier: bounded delivery disposition",
                    "independent carrier→Manticorp and Gallows delegates: performed delivery evidence",
                    "Manticorp and Gallows delegates→independent carrier: witnessed custody receipt",
                ],
                "cargo custody and each institutional scope are recorded without duplicate sovereignty",
                "office.flynt.tross",
                "independent carrier",
            ),
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
            roles(
                "restored being",
                "office.stonebend.hypergiant",
                "Gerald registrar",
                "Current Sea public witnesses",
                &[
                    "office.stonebend.hypergiant→Gerald registrar: bounded Name disposition",
                    "Gerald registrar→Current Sea public witnesses: continuity record",
                    "Current Sea public witnesses→restored being: witnessed Name receipt",
                ],
                "the restored being receives a challengeable Name record preserving both forms",
                "office.stonebend.hypergiant",
                "restored being",
            ),
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
            roles(
                "returning citizen",
                "office.stonebend.hypergiant",
                "Proliteriate hearing clerk",
                "public witnesses",
                &[
                    "office.stonebend.hypergiant→Proliteriate hearing clerk: provisional Name disposition",
                    "Proliteriate hearing clerk→public witnesses: notice and challenge record",
                    "public witnesses→returning citizen: provisional public reference receipt",
                ],
                "the citizen receives a reversible public reference with former history intact",
                "office.stonebend.hypergiant",
                "returning citizen",
            ),
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
            roles(
                "injured structure-being",
                "office.stonebend.hypergiant",
                "Freemason examiner",
                "Mt. Aura custody witness",
                &[
                    "office.stonebend.hypergiant→Freemason examiner: emergency custody scope",
                    "Freemason examiner→Mt. Aura custody witness: non-destructive survey record",
                    "Mt. Aura custody witness→injured structure-being: protected-subject receipt",
                ],
                "the subject and sample receive distinct custody records without destructive inspection",
                "office.stonebend.hypergiant",
                "injured structure-being",
            ),
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
            roles(
                "ordinary route users",
                "office.sandmanor.sandman",
                "Minoan exterior team",
                "Minorian interior team",
                &[
                    "office.sandmanor.sandman→Minoan exterior team: bounded proof disposition",
                    "Minoan exterior team→Minorian interior team: measured field result",
                    "Minorian interior team→ordinary route users: tested-scope receipt",
                ],
                "route users receive a versioned shelter result limited to tested conditions",
                "office.sandmanor.sandman",
                "ordinary route users",
            ),
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
            roles(
                "recovery patient",
                "office.sandmanor.sandman",
                "Glaüshouse therapist",
                "Gnome method keeper",
                &[
                    "office.sandmanor.sandman→Glaüshouse therapist: bounded revision question",
                    "Glaüshouse therapist→Gnome method keeper: patient-tolerance evidence",
                    "Gnome method keeper→recovery patient: revised-method receipt",
                ],
                "the patient receives a stopped, narrowed, or revised method with recovery intact",
                "office.sandmanor.sandman",
                "recovery patient",
            ),
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
            roles(
                "coastal access users",
                "office.sandmanor.sandman",
                "Elf candidate",
                "Centaur mentor",
                &[
                    "office.sandmanor.sandman→Elf candidate: bounded regional proof question",
                    "Elf candidate→Centaur mentor: coastal demonstration evidence",
                    "Centaur mentor→coastal access users: witnessed safety receipt",
                ],
                "coastal users receive a reproducible safety result without automatic transition",
                "office.sandmanor.sandman",
                "coastal access users",
            ),
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
            roles(
                "retrieved patient",
                "office.glaushouse.prima-donna",
                "emergency operator",
                "Nightingale",
                &[
                    "office.glaushouse.prima-donna→emergency operator: narrow stabilization scope",
                    "emergency operator→Nightingale: intervention and stop evidence",
                    "Nightingale→retrieved patient: care and refusal receipt",
                ],
                "the patient is stabilized or lawfully stopped without elective consent being inferred",
                "office.glaushouse.prima-donna",
                "retrieved patient",
            ),
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
            roles(
                "procedure subject",
                "office.glaushouse.prima-donna",
                "Prima Donna authority",
                "Sandmanor proof body",
                &[
                    "office.glaushouse.prima-donna→Prima Donna authority: consent-bounded clearance question",
                    "Prima Donna authority→Sandmanor proof body: separated proof and clinical record",
                    "Sandmanor proof body→procedure subject: conditional or refused procedure receipt",
                ],
                "the subject receives a separate proof, consent, and clearance disposition",
                "office.glaushouse.prima-donna",
                "procedure subject",
            ),
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
            roles(
                "recovering worker",
                "office.glaushouse.prima-donna",
                "Persephone service",
                "worksite receiver",
                &[
                    "office.glaushouse.prima-donna→Persephone service: bounded transfer clearance",
                    "Persephone service→worksite receiver: explicit restriction record",
                    "worksite receiver→recovering worker: accepted or refused transfer receipt",
                ],
                "the worker receives a restricted transfer or continued recovery without loss of standing",
                "office.glaushouse.prima-donna",
                "recovering worker",
            ),
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
    roles: OperationalRoleEnvelope,
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
        affected_actor: roles.affected_actor,
        decision_owner: roles.decision_owner,
        executor: roles.executor,
        witness: roles.witness,
        ordered_handoffs: roles.ordered_handoffs,
        completion_condition: roles.completion_condition,
        restitution_owner: roles.restitution_owner,
        recipient: roles.recipient,
    }
}

#[allow(clippy::too_many_arguments)]
const fn roles(
    affected_actor: &'static str,
    decision_owner: &'static str,
    executor: &'static str,
    witness: &'static str,
    ordered_handoffs: &'static [&'static str],
    completion_condition: &'static str,
    restitution_owner: &'static str,
    recipient: &'static str,
) -> OperationalRoleEnvelope {
    OperationalRoleEnvelope {
        affected_actor,
        decision_owner,
        executor,
        witness,
        ordered_handoffs,
        completion_condition,
        restitution_owner,
        recipient,
    }
}

const fn house_decision_office(house: House) -> &'static str {
    match house {
        House::Stonebend => "office.stonebend.hypergiant",
        House::Sandmanor => "office.sandmanor.sandman",
        House::Glaushouse => "office.glaushouse.prima-donna",
        House::Flynt => "office.flynt.tross",
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct FunctionalLoreArchiveHeader {
    schema_version: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FunctionalLoreArchiveEnvelopeV1 {
    format: String,
    schema_version: u16,
    checksum: String,
    payload: FunctionalLoreArchivePayloadV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FunctionalLoreArchivePayloadV1 {
    observed_at: u64,
    records: Vec<WireFunctionalLoreRecordV1>,
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
    #[serde(default)]
    affected_actor: String,
    #[serde(default)]
    decision_owner: String,
    #[serde(default)]
    executor: String,
    #[serde(default)]
    witness: String,
    #[serde(default)]
    ordered_handoffs: Vec<String>,
    #[serde(default)]
    completion_condition: String,
    #[serde(default)]
    restitution_owner: String,
    #[serde(default)]
    recipient: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireFunctionalLoreRecordV1 {
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

impl From<WireFunctionalLoreRecordV1> for WireFunctionalLoreRecord {
    fn from(record: WireFunctionalLoreRecordV1) -> Self {
        Self {
            id: record.id,
            house: record.house,
            authority_function: record.authority_function,
            authority_class: record.authority_class,
            authority_decision_id: record.authority_decision_id,
            authority_actor: record.authority_actor,
            authority_office: record.authority_office,
            route: record.route,
            location: record.location,
            jurisdiction: record.jurisdiction,
            entities: record.entities,
            dominant_verb: record.dominant_verb,
            trigger: record.trigger,
            evidence: record.evidence,
            uncertainty: record.uncertainty,
            player_choices: record.player_choices,
            lawful_state_changes: record.lawful_state_changes,
            persistence_and_replay: record.persistence_and_replay,
            presentation: record.presentation,
            failure_and_refusal: record.failure_and_refusal,
            affected_actor: String::new(),
            decision_owner: String::new(),
            executor: String::new(),
            witness: String::new(),
            ordered_handoffs: Vec::new(),
            completion_condition: String::new(),
            restitution_owner: String::new(),
            recipient: String::new(),
        }
    }
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
            affected_actor: definition.affected_actor.into(),
            decision_owner: definition.decision_owner.into(),
            executor: definition.executor.into(),
            witness: definition.witness.into(),
            ordered_handoffs: strings(definition.ordered_handoffs),
            completion_condition: definition.completion_condition.into(),
            restitution_owner: definition.restitution_owner.into(),
            recipient: definition.recipient.into(),
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

#[cfg(test)]
mod migration_tests {
    use std::path::Path;

    use super::*;

    fn v1_record(record: &FunctionalLoreRecord) -> WireFunctionalLoreRecordV1 {
        let record = WireFunctionalLoreRecord::from(record);
        WireFunctionalLoreRecordV1 {
            id: record.id,
            house: record.house,
            authority_function: record.authority_function,
            authority_class: record.authority_class,
            authority_decision_id: record.authority_decision_id,
            authority_actor: record.authority_actor,
            authority_office: record.authority_office,
            route: record.route,
            location: record.location,
            jurisdiction: record.jurisdiction,
            entities: record.entities,
            dominant_verb: record.dominant_verb,
            trigger: record.trigger,
            evidence: record.evidence,
            uncertainty: record.uncertainty,
            player_choices: record.player_choices,
            lawful_state_changes: record.lawful_state_changes,
            persistence_and_replay: record.persistence_and_replay,
            presentation: record.presentation,
            failure_and_refusal: record.failure_and_refusal,
        }
    }

    #[test]
    fn authentic_schema_one_checksum_is_verified_before_role_migration() {
        let world = WorldSession::load_or_canonical_at(Path::new(env!("CARGO_MANIFEST_DIR")))
            .expect("live world");
        let catalog = FunctionalLoreCatalog::instantiate(&world, CausalPosition::new(43))
            .expect("functional lore");
        let payload = FunctionalLoreArchivePayloadV1 {
            observed_at: catalog.observed_at().get(),
            records: catalog.records().iter().map(v1_record).collect(),
        };
        let envelope = FunctionalLoreArchiveEnvelopeV1 {
            format: FUNCTIONAL_LORE_ARCHIVE_FORMAT.into(),
            schema_version: 1,
            checksum: checksum_for(&payload).expect("schema-one checksum"),
            payload,
        };
        let encoded = serde_json::to_string_pretty(&envelope).expect("schema-one archive");

        assert_eq!(
            FunctionalLoreCatalog::replay(&encoded, &world).expect("schema-one migration"),
            catalog
        );

        let damaged = encoded.replacen(
            "public return lane",
            "unconstitutionally closed return lane",
            1,
        );
        assert_eq!(
            FunctionalLoreCatalog::replay(&damaged, &world),
            Err(FunctionalLoreError::ChecksumMismatch)
        );
    }
}
