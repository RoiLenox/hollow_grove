//! Revision 100 Stonebend governance, above the existing shared event archive.
//!
//! Callers supply authenticated, independently certified records. Evidence IDs
//! refer to those records; this adapter does not invent facts or verify digital
//! signatures. Legacy selection models remain historical readers, never inputs
//! to this adapter's accession command. No Bond or gameplay archive is rewritten.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::{
    InterfaceAggregateId, InterfaceApplyOutcome, InterfaceDomain, InterfaceEventEnvelope,
    InterfaceEventId, InterfaceEventKind, InterfaceFailureState, InterfaceIdentityKind,
    InterfaceIdentityRef, SHARED_INTERFACE_ARCHIVE_VERSION, SharedInterfaceRuntime,
    decode_shared_interface_archive, encode_shared_interface_archive,
};
use crate::hollow_grove_contract::House;
use crate::institution::{IdentityId, InstitutionalBeingId, OfficeHolder, OfficeId};
use crate::world::glaushouse::clinical_authorization::{self, Authorizations, Procedure};
pub use crate::world::glaushouse::governance::Attestation;
use crate::world::session::WorldSession;

pub const SOURCE: &str = "STONEBEND_REV100_GOVERNANCE_AMENDMENT.md";
pub const RULE_SET: &str = "stonebend.governance.rev100";
pub const PATHS: [(&str, [&str; 3]); 3] = [
    ("Construction", ["Journeyman", "Brickmason", "Proliteriate"]),
    ("Craftsmanship", ["Craftsman", "Artisan", "Freemason"]),
    ("Duty", ["Squire", "Justice", "Hypergiant"]),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Office {
    Proliteriate,
    Freemason,
    Hypergiant,
}

impl Office {
    pub const ALL: [Self; 3] = [Self::Proliteriate, Self::Freemason, Self::Hypergiant];

    pub fn id(self) -> OfficeId {
        match self {
            Self::Proliteriate => super::proliteriate_office_id(),
            Self::Freemason => super::high_freemason_office_id(),
            Self::Hypergiant => super::hypergiant_office_id(),
        }
    }

    pub const fn form(self) -> &'static str {
        match self {
            Self::Proliteriate => "proliteriate",
            Self::Freemason => "freemason",
            Self::Hypergiant => "hypergiant",
        }
    }
}

/// A supplied, lawful administrative mandate, not an automatically enacted
/// policy. Year ordinals are in-world years; no wall-clock conversion is assumed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub authorization: Attestation,
    pub administrator: String,
    pub credential_reviewers: BTreeSet<String>,
    pub court_reviewers: BTreeSet<String>,
    pub first_proliteriate_year: Option<u64>,
    pub first_freemason_year: Option<u64>,
}

impl Policy {
    fn validate(&self) -> Result<(), Error> {
        stable(&self.authorization.record)?;
        stable(&self.authorization.signer)?;
        stable(&self.administrator)?;
        require(
            self.authorization.signer != self.administrator,
            "self-appointed administrator",
        )?;
        for actor in self
            .credential_reviewers
            .iter()
            .chain(&self.court_reviewers)
        {
            stable(actor)?;
        }
        if let (Some(p), Some(f)) = (self.first_proliteriate_year, self.first_freemason_year) {
            require(
                p.abs_diff(f) % 6 == 3,
                "professional elections must be staggered three years",
            )?;
        }
        Ok(())
    }

    fn first_year(&self, office: Office) -> Result<u64, Error> {
        match office {
            Office::Proliteriate => self.first_proliteriate_year,
            Office::Freemason => self.first_freemason_year,
            Office::Hypergiant => None,
        }
        .ok_or(Error::RequiresLegalParameter(
            "first professional election cohort",
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ElectoralRoll {
    pub office: Office,
    pub eligible: BTreeSet<String>,
    pub certification: Attestation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Vote {
    pub voter: String,
    pub candidate: String,
    pub receipt: Attestation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Election {
    pub id: String,
    pub office: Office,
    pub candidates: BTreeSet<String>,
    pub roll: ElectoralRoll,
    pub rounds: Vec<Vec<Vote>>,
    pub winner: String,
    pub tenure: String,
    pub special: bool,
    pub certification: Attestation,
    /// Each Acclamation candidate needs separate Claim and Yield verification.
    pub claims: BTreeMap<String, Attestation>,
    pub yield_reviews: BTreeMap<String, Attestation>,
    pub seal: Option<String>,
}

/// Explicit continuity of a previously lawful professional tenure. All proofs
/// refer to authenticated external records, not facts inferred from a title.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionalContinuity {
    pub office: Office,
    pub holder: String,
    pub tenure: String,
    pub source_rule_set: String,
    pub election_id: String,
    pub began: u64,
    pub expires: u64,
    pub election: Attestation,
    pub term: Attestation,
    pub qualification: Attestation,
    pub mastery: Attestation,
    pub court_review: Attestation,
    pub seal: Option<SealContinuity>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SealContinuity {
    pub id: String,
    /// Confirms the existing seal belongs to this same continuing tenure.
    /// This is not a new seal or a transfer to another holder.
    pub certification: Attestation,
}

impl ProfessionalContinuity {
    fn proofs(&self) -> impl Iterator<Item = &Attestation> {
        [
            &self.election,
            &self.term,
            &self.qualification,
            &self.mastery,
            &self.court_review,
        ]
        .into_iter()
        .chain(self.seal.iter().map(|s| &s.certification))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recall {
    pub office: Office,
    pub tenure: String,
    pub roll: ElectoralRoll,
    pub petition: Vec<Attestation>,
    /// Choices are exactly "remove" and "retain".
    pub ballots: Vec<Vote>,
    pub court_review: Attestation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrialStage {
    Challenge,
    Defense,
    Proof,
    Judgment,
    Reckoning,
}

impl TrialStage {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Challenge => "The Challenge",
            Self::Defense => "The Defense",
            Self::Proof => "The Proof",
            Self::Judgment => "The Judgment",
            Self::Reckoning => "The Reckoning",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Finding {
    Dismissed,
    ProtectiveRemedy,
    RemovalWarranted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Action {
    /// Created only from an explicit source snapshot; never from display names.
    Initialize {
        policy: Policy,
        institutional_snapshot: String,
    },
    /// Additive command: the original initialization encoding stays unchanged.
    InitializeReconciled {
        policy: Policy,
        institutional_snapshot: String,
        professional_tenures: Vec<ProfessionalContinuity>,
    },
    RecognizeMastery {
        person: String,
        form: String,
    },
    Qualify {
        person: String,
        office: Office,
    },
    CertifyElection {
        election: Election,
    },
    CertifyRecall {
        recall: Recall,
    },
    OpenTrial {
        case: String,
        initiating_office: Office,
    },
    AdvanceTrial {
        case: String,
        stage: TrialStage,
        finding: Option<Finding>,
    },
    SupportRemoval {
        case: String,
        office: Office,
    },
    Recuse {
        case: String,
        office: Office,
    },
    Resign {
        office: Office,
        tenure: String,
    },
    RecordDeath {
        office: Office,
        tenure: String,
    },
    /// Explicit versioned clinical policy and authenticated decision records.
    UpdateClinicalAuthorization {
        update: clinical_authorization::Update,
    },
    /// Clearance and consent are references to actual external records, not
    /// clinical privileges granted by winning an election.
    Manifest {
        form: String,
        ordinary_reversion: String,
        clearance: Attestation,
        consent: Attestation,
    },
    ObserveYear,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Command {
    pub id: String,
    pub expected_revision: u64,
    pub year: u64,
    pub actor: String,
    pub evidence: BTreeSet<String>,
    pub action: Action,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tenure {
    pub id: String,
    pub office: Office,
    pub holder: String,
    pub began: u64,
    pub expires: Option<u64>,
    pub ended: Option<u64>,
    pub ending: Option<String>,
    /// Durable link to the accepted event that closed this tenure.
    pub closure_event: Option<String>,
    pub seal: Option<String>,
    pub imported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trial {
    pub target_tenure: String,
    pub initiator: String,
    pub stage: TrialStage,
    pub finding: Option<Finding>,
    pub supporters: BTreeMap<Office, String>,
    pub recusals: BTreeSet<Office>,
    /// Retained across resignation, recusal and replacement of a supporting
    /// tenure so a former party cannot later adjudicate the same case.
    pub professional_participants: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Governance {
    journal: SharedInterfaceRuntime,
    policy: Option<Policy>,
    source_snapshot: String,
    year: u64,
    tenures: Vec<Tenure>,
    qualifications: BTreeSet<(String, Office)>,
    mastery: BTreeSet<(String, String)>,
    expressions: BTreeMap<String, (String, String)>,
    trials: BTreeMap<String, Trial>,
    elections: BTreeSet<String>,
    ballot_receipts: BTreeSet<String>,
    clinical: Authorizations,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Invalid(&'static str),
    RequiresLegalParameter(&'static str),
    Archive(String),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stonebend governance rejected: {self:?}")
    }
}
impl std::error::Error for Error {}

fn require(condition: bool, reason: &'static str) -> Result<(), Error> {
    if condition {
        Ok(())
    } else {
        Err(Error::Invalid(reason))
    }
}
fn stable(value: &str) -> Result<(), Error> {
    IdentityId::new(value)
        .map(|_| ())
        .map_err(|_| Error::Invalid("invalid stable ID"))
}
fn archive(error: impl fmt::Display) -> Error {
    Error::Archive(error.to_string())
}

fn ordinary(form: &str) -> bool {
    matches!(
        form,
        "gremlin" | "goblin" | "ghoul" | "spectre" | "troll" | "orc" | "ogre" | "troglodyte"
    )
}

impl Governance {
    pub fn new(world: &WorldSession, policy: Policy, year: u64) -> Result<Self, Error> {
        let mut result = Self::default();
        result.execute(Command {
            id: "event.stonebend.governance.initialize".into(),
            expected_revision: 0,
            year,
            actor: policy.authorization.signer.clone(),
            evidence: BTreeSet::from([policy.authorization.record.clone()]),
            action: Action::Initialize {
                policy,
                institutional_snapshot: world.persisted_state_output(),
            },
        })?;
        Ok(result)
    }

    /// Creates a separate replayable result, without modifying the source.
    /// Supplying records never relaxes the normal import's missing-proof gate.
    pub fn from_reconciled_snapshot(
        world: &WorldSession,
        policy: Policy,
        year: u64,
        professional_tenures: Vec<ProfessionalContinuity>,
    ) -> Result<Self, Error> {
        let mut evidence = BTreeSet::from([policy.authorization.record.clone()]);
        evidence.extend(
            professional_tenures
                .iter()
                .flat_map(|t| t.proofs())
                .map(|p| p.record.clone()),
        );
        let mut result = Self::default();
        result.execute(Command {
            id: "event.stonebend.governance.initialize".into(),
            expected_revision: 0,
            year,
            actor: policy.authorization.signer.clone(),
            evidence,
            action: Action::InitializeReconciled {
                policy,
                institutional_snapshot: world.persisted_state_output(),
                professional_tenures,
            },
        })?;
        Ok(result)
    }

    pub fn revision(&self) -> u64 {
        self.journal.events().len() as u64
    }
    pub const fn year(&self) -> u64 {
        self.year
    }
    pub fn tenures(&self) -> &[Tenure] {
        &self.tenures
    }
    pub fn active_seal(&self) -> Option<&str> {
        self.incumbent(Office::Freemason)
            .and_then(|t| t.seal.as_deref())
    }
    pub fn trial(&self, case: &str) -> Option<&Trial> {
        self.trials.get(case)
    }
    pub fn incumbent(&self, office: Office) -> Option<&Tenure> {
        self.tenures.iter().find(|t| {
            t.office == office && t.ended.is_none() && t.expires.is_none_or(|end| self.year < end)
        })
    }
    pub fn may_manifest(&self, person: &str, form: &str) -> bool {
        if form == "hypergiant" {
            self.incumbent(Office::Hypergiant)
                .is_some_and(|t| t.holder == person)
        } else {
            self.mastery.contains(&(person.into(), form.into()))
        }
    }
    pub fn expressed_form(&self, person: &str) -> Option<&str> {
        self.expressions.get(person).map(|(form, fallback)| {
            if self.may_manifest(person, form) {
                form.as_str()
            } else {
                fallback.as_str()
            }
        })
    }

    /// Project verified current holders into the existing authority catalog.
    /// Retain inactive source holders and project closed tenures as inactive.
    /// These holder records do not replace the archive's full tenure history.
    /// Memberships, witnesses, other Houses and earned Deck XP are untouched.
    pub fn world_session(&self) -> Result<WorldSession, Error> {
        let world = WorldSession::from_persisted_output(&self.source_snapshot).map_err(archive)?;
        let mut state = world.institutional().clone();
        for holder in &mut state.catalog.office_holders {
            if let Some(office) = Office::ALL.iter().find(|o| o.id() == holder.office) {
                // An imported incumbent retains the original active record
                // only while that imported tenure continues. Never reactivate
                // a historical row merely because the same person wins again.
                holder.active &= self
                    .incumbent(*office)
                    .is_some_and(|t| t.imported && t.holder == holder.being.as_str());
            }
        }
        for tenure in &self.tenures {
            let holder = OfficeHolder {
                office: tenure.office.id(),
                being: InstitutionalBeingId::new(&tenure.holder).map_err(archive)?,
                active: self
                    .incumbent(tenure.office)
                    .is_some_and(|t| t.id == tenure.id),
            };
            if !state.catalog.office_holders.contains(&holder) {
                state.catalog.office_holders.push(holder);
            }
        }
        WorldSession::from_institutional_state(state).map_err(|e| archive(format!("{e:?}")))
    }

    fn admin(&self, actor: &str) -> Result<(), Error> {
        let policy = self
            .policy
            .as_ref()
            .ok_or(Error::Invalid("not initialized"))?;
        require(
            actor == policy.administrator
                && !self
                    .tenures
                    .iter()
                    .any(|t| t.ended.is_none() && t.holder == actor),
            "administrator must be independent",
        )
    }
    fn court(&self, actor: &str, parties: &[&str]) -> Result<(), Error> {
        let policy = self
            .policy
            .as_ref()
            .ok_or(Error::Invalid("not initialized"))?;
        require(
            policy.court_reviewers.contains(actor)
                && !parties.contains(&actor)
                && !Office::ALL
                    .iter()
                    .any(|o| self.incumbent(*o).is_some_and(|t| t.holder == actor)),
            "independent Court review required",
        )
    }
    fn attest(&self, proof: &Attestation, command: &Command) -> Result<(), Error> {
        stable(&proof.record)?;
        stable(&proof.signer)?;
        require(
            command.evidence.contains(&proof.record),
            "attestation missing from event evidence",
        )
    }
    fn roll(&self, roll: &ElectoralRoll, office: Office, cmd: &Command) -> Result<(), Error> {
        require(
            roll.office == office && !roll.eligible.is_empty(),
            "wrong or empty electoral roll",
        )?;
        self.attest(&roll.certification, cmd)?;
        self.admin(&roll.certification.signer)?;
        for voter in &roll.eligible {
            stable(voter)?;
        }
        Ok(())
    }
    fn tally(
        &self,
        votes: &[Vote],
        roll: &ElectoralRoll,
        choices: &BTreeSet<String>,
        cmd: &Command,
    ) -> Result<BTreeMap<String, u64>, Error> {
        let mut voters = BTreeSet::new();
        let mut receipts = BTreeSet::new();
        let mut counts = choices
            .iter()
            .map(|c| (c.clone(), 0))
            .collect::<BTreeMap<_, _>>();
        for vote in votes {
            self.attest(&vote.receipt, cmd)?;
            require(
                vote.receipt.signer == vote.voter
                    && roll.eligible.contains(&vote.voter)
                    && voters.insert(&vote.voter)
                    && receipts.insert(&vote.receipt.record),
                "invalid or duplicate ballot",
            )?;
            *counts
                .get_mut(&vote.candidate)
                .ok_or(Error::Invalid("ineligible ballot choice"))? += 1;
        }
        Ok(counts)
    }

    pub fn execute(&mut self, command: Command) -> Result<InterfaceApplyOutcome, Error> {
        self.apply_command(command, false)
    }

    // Legacy clinical rules are available only while decoding pre-transition
    // history. They cannot authorize a new live manifestation.
    fn apply_command(
        &mut self,
        command: Command,
        replay: bool,
    ) -> Result<InterfaceApplyOutcome, Error> {
        let event = self.envelope(&command)?;
        if let Some(previous) = self.journal.event(&event.event_id) {
            return if previous == &event {
                Ok(InterfaceApplyOutcome::Idempotent)
            } else {
                Err(Error::Invalid("conflicting event ID"))
            };
        }
        require(
            command.expected_revision == self.revision(),
            "stale revision",
        )?;
        require(command.year >= self.year, "time moved backwards")?;
        let mut next = self.clone();
        next.year = command.year;
        for t in &mut next.tenures {
            if t.ended.is_none() && t.expires.is_some_and(|end| end <= next.year) {
                t.ended = t.expires;
                t.ending = Some("term-expired".into());
                t.closure_event = Some(command.id.clone());
            }
        }
        next.reduce(&command, replay)?;
        next.journal.apply(event).map_err(archive)?;
        *self = next;
        Ok(InterfaceApplyOutcome::Applied)
    }

    fn reduce(&mut self, cmd: &Command, replay: bool) -> Result<(), Error> {
        stable(&cmd.id)?;
        stable(&cmd.actor)?;
        require(!cmd.evidence.is_empty(), "evidence required")?;
        for evidence in &cmd.evidence {
            stable(evidence)?;
        }
        let initialization = match &cmd.action {
            Action::Initialize {
                policy,
                institutional_snapshot,
            } => Some((policy, institutional_snapshot, &[][..])),
            Action::InitializeReconciled {
                policy,
                institutional_snapshot,
                professional_tenures,
            } => Some((
                policy,
                institutional_snapshot,
                professional_tenures.as_slice(),
            )),
            _ => None,
        };
        if let Some((policy, institutional_snapshot, professional_tenures)) = initialization {
            require(
                self.policy.is_none() && self.revision() == 0,
                "already initialized",
            )?;
            policy.validate()?;
            self.attest(&policy.authorization, cmd)?;
            require(
                cmd.actor == policy.authorization.signer,
                "policy authorization mismatch",
            )?;
            let world =
                WorldSession::from_persisted_output(institutional_snapshot).map_err(archive)?;
            self.policy = Some(policy.clone());
            self.source_snapshot = institutional_snapshot.clone();
            self.import_professionals(&world, professional_tenures, cmd)?;
            for office in Office::ALL {
                for holder in world
                    .institutional()
                    .catalog
                    .office_holders
                    .iter()
                    .filter(|h| h.active && h.office == office.id())
                {
                    if office != Office::Hypergiant {
                        continue;
                    }
                    self.tenures.push(Tenure {
                        id: format!("tenure.imported.{}", office.id().as_str()),
                        office,
                        holder: holder.being.as_str().into(),
                        began: cmd.year,
                        expires: None,
                        ended: None,
                        ending: None,
                        closure_event: None,
                        seal: None,
                        imported: true,
                    });
                }
            }
            return Ok(());
        }
        require(self.policy.is_some(), "not initialized")?;
        match &cmd.action {
            Action::Initialize { .. } | Action::InitializeReconciled { .. } => unreachable!(),
            Action::RecognizeMastery { person, form } => {
                self.credential(&cmd.actor, person)?;
                require(
                    ordinary(form) || matches!(form.as_str(), "proliteriate" | "freemason"),
                    "not an earnable Stonebend Form",
                )?;
                self.mastery.insert((person.clone(), form.clone()));
            }
            Action::Qualify { person, office } => {
                self.credential(&cmd.actor, person)?;
                // Hypergiant qualification is independently evidenced Justice
                // standing, not learned Hypergiant Form or office promotion.
                self.qualifications.insert((person.clone(), *office));
            }
            Action::CertifyElection { election } => self.elect(election, cmd)?,
            Action::CertifyRecall { recall } => self.recall(recall, cmd)?,
            Action::OpenTrial {
                case,
                initiating_office,
            } => {
                stable(case)?;
                require(
                    *initiating_office != Office::Hypergiant,
                    "sovereign cannot initiate own Trial",
                )?;
                require(
                    self.incumbent(*initiating_office)
                        .is_some_and(|t| t.holder == cmd.actor),
                    "only an elected professional incumbent may challenge",
                )?;
                let target = self
                    .incumbent(Office::Hypergiant)
                    .ok_or(Error::Invalid("sovereign vacancy"))?;
                require(!self.trials.contains_key(case), "case ID already used")?;
                self.trials.insert(
                    case.clone(),
                    Trial {
                        target_tenure: target.id.clone(),
                        initiator: cmd.actor.clone(),
                        stage: TrialStage::Challenge,
                        finding: None,
                        supporters: BTreeMap::new(),
                        recusals: BTreeSet::new(),
                        professional_participants: BTreeSet::from([cmd.actor.clone()]),
                    },
                );
            }
            Action::AdvanceTrial {
                case,
                stage,
                finding,
            } => self.advance_trial(case, *stage, *finding, cmd)?,
            Action::SupportRemoval { case, office } | Action::Recuse { case, office } => {
                require(*office != Office::Hypergiant, "not a professional office")?;
                let incumbent = self
                    .incumbent(*office)
                    .ok_or(Error::Invalid("professional vacancy"))?;
                require(
                    incumbent.holder == cmd.actor,
                    "support requires current incumbent",
                )?;
                let tenure = incumbent.id.clone();
                let trial = self
                    .trials
                    .get_mut(case)
                    .ok_or(Error::Invalid("unknown Trial"))?;
                require(trial.stage != TrialStage::Reckoning, "Trial is closed")?;
                trial.professional_participants.insert(cmd.actor.clone());
                if matches!(cmd.action, Action::Recuse { .. }) {
                    trial.recusals.insert(*office);
                    trial.supporters.remove(office);
                } else {
                    require(!trial.recusals.contains(office), "office recused")?;
                    trial.supporters.insert(*office, tenure);
                }
            }
            Action::Resign { office, tenure } | Action::RecordDeath { office, tenure } => {
                let t = self
                    .incumbent(*office)
                    .ok_or(Error::Invalid("office vacant"))?;
                require(t.id == *tenure, "wrong tenure")?;
                if matches!(cmd.action, Action::Resign { .. }) {
                    require(cmd.actor == t.holder, "only incumbent may resign")?;
                } else {
                    self.court(&cmd.actor, &[&t.holder])?;
                }
                self.end(*office, "lawful-vacancy", &cmd.id)?;
            }
            Action::UpdateClinicalAuthorization { update } => {
                let policy = self
                    .policy
                    .as_ref()
                    .ok_or(Error::Invalid("not initialized"))?;
                self.clinical
                    .apply(
                        update,
                        clinical_authorization::Context {
                            house: House::Stonebend,
                            year: cmd.year,
                            actor: &cmd.actor,
                            evidence: &cmd.evidence,
                            policy_authorization: &policy.authorization,
                        },
                    )
                    .map_err(Error::Invalid)?;
            }
            Action::Manifest {
                form,
                ordinary_reversion,
                clearance,
                consent,
            } => {
                require(self.may_manifest(&cmd.actor, form), "no Form entitlement")?;
                require(
                    ordinary(ordinary_reversion)
                        && self.may_manifest(&cmd.actor, ordinary_reversion),
                    "earned ordinary reversion required",
                )?;
                self.attest(clearance, cmd)?;
                self.attest(consent, cmd)?;
                require(
                    consent.signer == cmd.actor
                        && clearance.signer != cmd.actor
                        && consent.record != clearance.record,
                    "consent and independent clearance required",
                )?;
                if self.clinical.active() {
                    self.clinical
                        .authorize(
                            &Procedure {
                                house: House::Stonebend,
                                form: form.clone(),
                            },
                            &cmd.actor,
                            cmd.year,
                            clearance,
                            consent,
                        )
                        .map_err(Error::Invalid)?;
                } else if replay {
                    // Preserve the exact historical rule, not a new credential.
                    let world = self.world_session()?;
                    require(
                        world
                            .institutional()
                            .catalog
                            .office_holders
                            .iter()
                            .any(|h| {
                                h.active
                                    && h.office == crate::world::glaushouse::prima_donna_office_id()
                                    && h.being.as_str() == clearance.signer
                            }),
                        "existing Glaushouse clearance authority required",
                    )?;
                } else {
                    return Err(Error::RequiresLegalParameter(
                        "clinical authorization policy v2",
                    ));
                }
                self.expressions.insert(
                    cmd.actor.clone(),
                    (form.clone(), ordinary_reversion.clone()),
                );
            }
            Action::ObserveYear => self.admin(&cmd.actor)?,
        }
        // Safe reversion is part of the same accepted state transition.
        let revoke = self
            .expressions
            .iter()
            .filter(|(person, (form, _))| !self.may_manifest(person, form))
            .map(|(p, _)| p.clone())
            .collect::<Vec<_>>();
        for person in revoke {
            let body = self
                .expressions
                .get_mut(&person)
                .expect("existing expression");
            body.0.clone_from(&body.1);
        }
        Ok(())
    }

    fn import_professionals(
        &mut self,
        world: &WorldSession,
        records: &[ProfessionalContinuity],
        cmd: &Command,
    ) -> Result<(), Error> {
        let holders = world
            .institutional()
            .catalog
            .office_holders
            .iter()
            .filter(|h| h.active && Office::ALL.iter().any(|o| o.id() == h.office))
            .collect::<Vec<_>>();
        let expected = holders
            .iter()
            .filter(|h| h.office != Office::Hypergiant.id())
            .count();
        if records.len() < expected {
            return Err(Error::RequiresLegalParameter(
                "legacy professional term and seal",
            ));
        }
        require(
            records.len() == expected,
            "unexpected professional continuity record",
        )?;
        let mut offices = BTreeSet::new();
        let mut evidence = BTreeSet::new();
        let policy = self.policy.as_ref().expect("initialized").clone();
        let parties = holders
            .iter()
            .map(|h| h.being.as_str())
            .chain(std::iter::once(policy.administrator.as_str()))
            .collect::<Vec<_>>();
        for record in records {
            require(
                record.office != Office::Hypergiant && offices.insert(record.office),
                "duplicate or nonprofessional continuity office",
            )?;
            stable(&record.holder)?;
            stable(&record.tenure)?;
            stable(&record.source_rule_set)?;
            stable(&record.election_id)?;
            require(
                holders
                    .iter()
                    .any(|h| h.office == record.office.id() && h.being.as_str() == record.holder),
                "continuity holder does not match source snapshot",
            )?;
            require(
                holders
                    .iter()
                    .filter(|h| h.being.as_str() == record.holder)
                    .count()
                    == 1,
                "incompatible simultaneous offices",
            )?;
            require(
                !holders
                    .iter()
                    .any(|h| h.being.as_str() == policy.administrator),
                "administrator must be independent",
            )?;
            require(
                !self.tenures.iter().any(|t| t.id == record.tenure)
                    && record.tenure
                        != format!("tenure.imported.{}", Office::Hypergiant.id().as_str()),
                "tenure ID reused",
            )?;
            require(
                self.elections.insert(record.election_id.clone()),
                "election ID reused",
            )?;
            let epoch = policy.first_year(record.office)?;
            if !(record.began >= epoch
                && record.began <= cmd.year
                && cmd.year < record.expires
                && record.expires - record.began <= 6
                && (record.expires - epoch).is_multiple_of(6))
            {
                return Err(Error::RequiresLegalParameter(
                    "legacy term reconciliation with election cohorts",
                ));
            }
            for proof in record.proofs() {
                self.attest(proof, cmd)?;
                require(
                    evidence.insert(&proof.record),
                    "continuity proofs require distinct records",
                )?;
            }
            self.admin(&record.election.signer)?;
            self.admin(&record.term.signer)?;
            self.credential(&record.qualification.signer, &record.holder)?;
            self.credential(&record.mastery.signer, &record.holder)?;
            require(
                !holders.iter().any(|h| {
                    h.being.as_str() == record.qualification.signer
                        || h.being.as_str() == record.mastery.signer
                }),
                "independent credential reviewer required",
            )?;
            self.court(&record.court_review.signer, &parties)?;
            let seal = match (&record.seal, record.office) {
                (Some(seal), Office::Freemason) => {
                    stable(&seal.id)?;
                    self.admin(&seal.certification.signer)?;
                    require(
                        !self
                            .tenures
                            .iter()
                            .any(|t| t.seal.as_ref() == Some(&seal.id)),
                        "seal ID reused",
                    )?;
                    Some(seal.id.clone())
                }
                (None, Office::Proliteriate) => None,
                (None, Office::Freemason) => {
                    return Err(Error::RequiresLegalParameter(
                        "legacy Freemason seal certification",
                    ));
                }
                _ => return Err(Error::Invalid("only elected Freemason bears seal")),
            };
            self.qualifications
                .insert((record.holder.clone(), record.office));
            self.mastery
                .insert((record.holder.clone(), record.office.form().into()));
            self.tenures.push(Tenure {
                id: record.tenure.clone(),
                office: record.office,
                holder: record.holder.clone(),
                began: record.began,
                expires: Some(record.expires),
                ended: None,
                ending: None,
                closure_event: None,
                seal,
                imported: true,
            });
        }
        Ok(())
    }

    fn credential(&self, reviewer: &str, person: &str) -> Result<(), Error> {
        stable(person)?;
        require(
            reviewer != person
                && self
                    .policy
                    .as_ref()
                    .expect("initialized")
                    .credential_reviewers
                    .contains(reviewer),
            "independent credential reviewer required",
        )
    }

    fn elect(&mut self, e: &Election, cmd: &Command) -> Result<(), Error> {
        self.admin(&cmd.actor)?;
        self.roll(&e.roll, e.office, cmd)?;
        self.attest(&e.certification, cmd)?;
        self.admin(&e.certification.signer)?;
        stable(&e.id)?;
        stable(&e.tenure)?;
        stable(&e.winner)?;
        require(
            !self.elections.contains(&e.id) && !self.tenures.iter().any(|t| t.id == e.tenure),
            "election or tenure ID reused",
        )?;
        require(
            !e.candidates.is_empty() && !e.candidates.contains(&cmd.actor),
            "independent election administrator required",
        )?;
        for candidate in &e.candidates {
            require(
                self.qualifications.contains(&(candidate.clone(), e.office)),
                "candidate lacks independent qualification",
            )?;
            if e.office != Office::Hypergiant {
                require(
                    self.may_manifest(candidate, e.office.form()),
                    "candidate lacks professional Form mastery",
                )?;
            }
            require(
                !Office::ALL
                    .iter()
                    .filter(|o| **o != e.office)
                    .any(|o| self.incumbent(*o).is_some_and(|t| t.holder == *candidate)),
                "incompatible simultaneous offices",
            )?;
        }
        self.reserve_ballots(e.rounds.iter().flatten())?;
        require(
            (1..=2).contains(&e.rounds.len()),
            "first ballot and optional runoff required",
        )?;
        let first = self.tally(&e.rounds[0], &e.roll, &e.candidates, cmd)?;
        let winner = majority(&first);
        if let Some(winner) = winner {
            require(
                e.rounds.len() == 1 && winner == e.winner,
                "wrong majority winner or unnecessary runoff",
            )?;
        } else {
            require(
                e.rounds.len() == 2 && !e.rounds[0].is_empty(),
                "no majority; runoff required",
            )?;
            let mut ranked = first.iter().collect::<Vec<_>>();
            ranked.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
            require(ranked.len() >= 2, "no runoff pair")?;
            if ranked.len() > 2 && ranked[1].1 == ranked[2].1 {
                return Err(Error::RequiresLegalParameter("runoff qualification tie"));
            }
            let finalists = BTreeSet::from([ranked[0].0.clone(), ranked[1].0.clone()]);
            let last = self.tally(&e.rounds[1], &e.roll, &finalists, cmd)?;
            require(
                majority(&last) == Some(e.winner.as_str()),
                "runoff has no certified majority winner",
            )?;
        }
        let expires = if e.office == Office::Hypergiant {
            require(
                self.incumbent(e.office).is_none() && !e.special && e.seal.is_none(),
                "Acclamation requires vacancy",
            )?;
            for (proofs, office) in [
                (&e.claims, Office::Freemason),
                (&e.yield_reviews, Office::Proliteriate),
            ] {
                require(
                    proofs.keys().cloned().collect::<BTreeSet<_>>() == e.candidates,
                    "every candidate requires Claim and Yield verification",
                )?;
                for (candidate, proof) in proofs {
                    self.attest(proof, cmd)?;
                    require(
                        proof.signer != *candidate
                            && self
                                .incumbent(office)
                                .is_some_and(|t| t.holder == proof.signer),
                        "independent incumbent Claim/Yield verification required",
                    )?;
                }
            }
            None
        } else {
            require(
                e.claims.is_empty() && e.yield_reviews.is_empty(),
                "Acclamation proofs on professional election",
            )?;
            require(
                self.incumbent(e.office).is_none(),
                "professional tenure still occupied",
            )?;
            let epoch = self
                .policy
                .as_ref()
                .expect("initialized")
                .first_year(e.office)?;
            require(cmd.year >= epoch, "before first election cohort")?;
            if e.special {
                let previous = self
                    .tenures
                    .iter()
                    .rev()
                    .find(|t| t.office == e.office)
                    .ok_or(Error::RequiresLegalParameter("special election remainder"))?;
                let end = previous
                    .expires
                    .ok_or(Error::RequiresLegalParameter("legacy professional term"))?;
                require(cmd.year < end, "expired term requires regular election")?;
                Some(end)
            } else {
                require(
                    (cmd.year - epoch).is_multiple_of(6),
                    "outside six-year election cohort",
                )?;
                Some(
                    cmd.year
                        .checked_add(6)
                        .ok_or(Error::Invalid("term overflow"))?,
                )
            }
        };
        if e.office == Office::Freemason {
            let seal = e
                .seal
                .as_ref()
                .ok_or(Error::Invalid("independently certified seal required"))?;
            stable(seal)?;
            require(
                !self.tenures.iter().any(|t| t.seal.as_ref() == Some(seal)),
                "seal ID reused",
            )?;
            require(
                e.certification.signer != e.winner
                    && self
                        .tenures
                        .iter()
                        .rev()
                        .find(|t| t.office == Office::Freemason)
                        .is_none_or(|t| t.holder != e.certification.signer),
                "outgoing or incoming bearer cannot certify seal",
            )?;
        } else {
            require(e.seal.is_none(), "only elected Freemason bears seal")?;
        }
        self.tenures.push(Tenure {
            id: e.tenure.clone(),
            office: e.office,
            holder: e.winner.clone(),
            began: cmd.year,
            expires,
            ended: None,
            ending: None,
            closure_event: None,
            seal: e.seal.clone(),
            imported: false,
        });
        self.elections.insert(e.id.clone());
        Ok(())
    }

    fn recall(&mut self, r: &Recall, cmd: &Command) -> Result<(), Error> {
        self.admin(&cmd.actor)?;
        require(
            r.office != Office::Hypergiant,
            "sovereign removal requires Trial of Duty",
        )?;
        self.roll(&r.roll, r.office, cmd)?;
        let t = self
            .incumbent(r.office)
            .ok_or(Error::Invalid("office vacant"))?;
        require(t.id == r.tenure, "wrong recall tenure")?;
        self.attest(&r.court_review, cmd)?;
        let parties = [t.holder.as_str(), cmd.actor.as_str()]
            .into_iter()
            .chain(r.petition.iter().map(|p| p.signer.as_str()))
            .chain(r.ballots.iter().map(|v| v.voter.as_str()))
            .collect::<Vec<_>>();
        self.court(&r.court_review.signer, &parties)?;
        let mut petitioners = BTreeSet::new();
        let mut receipts = BTreeSet::new();
        for p in &r.petition {
            self.attest(p, cmd)?;
            require(
                r.roll.eligible.contains(&p.signer)
                    && petitioners.insert(&p.signer)
                    && receipts.insert(&p.record),
                "invalid recall petition",
            )?;
        }
        require(
            petitioners.len() as u128 * 100 >= r.roll.eligible.len() as u128 * 20,
            "recall petition below 20 percent",
        )?;
        self.reserve_ballots(r.ballots.iter())?;
        let counts = self.tally(
            &r.ballots,
            &r.roll,
            &BTreeSet::from(["remove".into(), "retain".into()]),
            cmd,
        )?;
        require(
            r.ballots.len() as u128 * 100 >= r.roll.eligible.len() as u128 * 40,
            "recall turnout below 40 percent",
        )?;
        if majority(&counts) == Some("remove") {
            self.end(r.office, "certified-recall", &cmd.id)?;
        }
        Ok(())
    }

    fn reserve_ballots<'a>(&mut self, votes: impl Iterator<Item = &'a Vote>) -> Result<(), Error> {
        for vote in votes {
            require(
                self.ballot_receipts.insert(vote.receipt.record.clone()),
                "ballot receipt reused across rounds or elections",
            )?;
        }
        Ok(())
    }

    fn advance_trial(
        &mut self,
        case: &str,
        stage: TrialStage,
        finding: Option<Finding>,
        cmd: &Command,
    ) -> Result<(), Error> {
        let trial = self
            .trials
            .get(case)
            .ok_or(Error::Invalid("unknown Trial"))?
            .clone();
        let target = self
            .incumbent(Office::Hypergiant)
            .ok_or(Error::Invalid("sovereign vacancy"))?;
        require(
            target.id == trial.target_tenure,
            "Trial cannot target successor tenure",
        )?;
        let parties = trial
            .professional_participants
            .iter()
            .map(String::as_str)
            .chain(std::iter::once(target.holder.as_str()))
            .collect::<Vec<_>>();
        self.court(&cmd.actor, &parties)?;
        let expected = match trial.stage {
            TrialStage::Challenge => TrialStage::Defense,
            TrialStage::Defense => TrialStage::Proof,
            TrialStage::Proof => TrialStage::Judgment,
            TrialStage::Judgment => TrialStage::Reckoning,
            TrialStage::Reckoning => return Err(Error::Invalid("Trial closed")),
        };
        require(stage == expected, "Trial stage skipped")?;
        require(
            (stage == TrialStage::Judgment) == finding.is_some(),
            "finding belongs to Judgment",
        )?;
        if stage == TrialStage::Reckoning && trial.finding == Some(Finding::RemovalWarranted) {
            for office in [Office::Proliteriate, Office::Freemason] {
                let t = self
                    .incumbent(office)
                    .ok_or(Error::Invalid("removal cannot count a vacancy"))?;
                require(
                    !trial.recusals.contains(&office)
                        && trial.supporters.get(&office) == Some(&t.id),
                    "both current professional tenures must support removal",
                )?;
            }
            self.end(Office::Hypergiant, "trial-of-duty-removal", &cmd.id)?;
        }
        let trial = self.trials.get_mut(case).expect("existing Trial");
        trial.stage = stage;
        if finding.is_some() {
            trial.finding = finding;
        }
        Ok(())
    }

    fn end(&mut self, office: Office, reason: &str, event: &str) -> Result<(), Error> {
        let id = self
            .incumbent(office)
            .ok_or(Error::Invalid("office vacant"))?
            .id
            .clone();
        let t = self
            .tenures
            .iter_mut()
            .find(|t| t.id == id)
            .expect("incumbent");
        t.ended = Some(self.year);
        t.ending = Some(reason.into());
        t.closure_event = Some(event.into());
        Ok(())
    }

    fn envelope(&self, cmd: &Command) -> Result<InterfaceEventEnvelope, Error> {
        let aggregate = InterfaceAggregateId::new(RULE_SET).map_err(archive)?;
        let command_json = serde_json::to_string(cmd).map_err(archive)?;
        // Same FNV-1a corruption-detection convention used by House handoffs.
        // This is not authentication; external evidence must be authenticated.
        let checksum = command_json
            .bytes()
            .fold(0xcbf29ce484222325_u64, |hash, byte| {
                (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
            });
        let related_case = match &cmd.action {
            Action::OpenTrial { case, .. }
            | Action::AdvanceTrial { case, .. }
            | Action::SupportRemoval { case, .. }
            | Action::Recuse { case, .. } => {
                Some(InterfaceAggregateId::new(case).map_err(archive)?)
            }
            _ => None,
        };
        let reference =
            |kind, id: &str| InterfaceIdentityRef::new(kind, id, SOURCE).map_err(archive);
        Ok(InterfaceEventEnvelope {
            event_id: InterfaceEventId::new(&cmd.id).map_err(archive)?,
            source_aggregate: aggregate.clone(),
            receiving_aggregate: aggregate,
            source_domain: InterfaceDomain::House(House::Stonebend),
            receiving_domain: InterfaceDomain::House(House::Stonebend),
            authority: reference(InterfaceIdentityKind::Authority, &cmd.actor)?,
            subjects: vec![],
            evidence: cmd
                .evidence
                .iter()
                .map(|id| reference(InterfaceIdentityKind::Evidence, id))
                .collect::<Result<_, _>>()?,
            kind: InterfaceEventKind::HouseContinuityEvent,
            semantic_sequence: cmd.expected_revision,
            causal_parent: None,
            related_case,
            related_amendment: None,
            responsible_remedy_institution: None,
            failure_state: InterfaceFailureState::None,
            migration_version: SHARED_INTERFACE_ARCHIVE_VERSION,
            historical_links: vec![],
            payload: BTreeMap::from([
                ("rule-set".into(), RULE_SET.into()),
                (
                    "command-checksum".into(),
                    format!("fnv1a64:{checksum:016x}"),
                ),
                ("command".into(), command_json),
            ]),
        })
    }

    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        encode_shared_interface_archive(&self.journal).map_err(archive)
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        let journal = decode_shared_interface_archive(bytes).map_err(archive)?;
        let mut result = Self::default();
        for event in journal.events() {
            let cmd: Command = serde_json::from_str(
                event
                    .payload
                    .get("command")
                    .ok_or(Error::Invalid("not a Stonebend governance archive"))?,
            )
            .map_err(archive)?;
            require(
                result.envelope(&cmd)? == *event,
                "event envelope and command disagree",
            )?;
            result.apply_command(cmd, true)?;
        }
        require(
            result.policy.is_some(),
            "missing policy and source snapshot",
        )?;
        Ok(result)
    }
}

fn majority(counts: &BTreeMap<String, u64>) -> Option<&str> {
    let total = counts.values().map(|n| u128::from(*n)).sum::<u128>();
    counts
        .iter()
        .find(|(_, n)| u128::from(**n) * 2 > total)
        .map(|(name, _)| name.as_str())
}
