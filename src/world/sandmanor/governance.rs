//! Revision 100 elections and Contest of Improvement, above the shared archive.
//! Evidence references must be authenticated by the calling authority adapter.
//! Historical guardian/lineage records are never converted into elected seats.
use super::milestone::{
    self, CandidateBaseline, CrowdJudgment, CrowdVerdict, ImprovementEvidence, PublicTrial,
    TeachingIntegrityFinding, TrialDomain,
};
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
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub const SOURCE: &str = "SANDMANOR_REV100_GOVERNANCE_AMENDMENT.md";
pub const RULE_SET: &str = "sandmanor.governance.rev100";
pub const FORM_PATHS: [(&str, [&str; 4]); 2] = [
    ("North", ["Gnome", "Satyr", "Minotaur", "Hecaton"]),
    ("South", ["Elf", "Faun", "Centaur", "Pegasus"]),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Office {
    Mino,
    Cent,
    Sandman,
}
impl Office {
    pub const ALL: [Self; 3] = [Self::Mino, Self::Cent, Self::Sandman];
    pub const GOVERNORS: [Self; 2] = [Self::Mino, Self::Cent];
    pub fn id(self) -> OfficeId {
        match self {
            Self::Mino => super::mino_office_id(),
            Self::Cent => super::cent_office_id(),
            Self::Sandman => super::sandman_office_id(),
        }
    }
    pub const fn form(self) -> &'static str {
        match self {
            Self::Mino => "hecaton",
            Self::Cent => "pegasus",
            Self::Sandman => "sandman",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecallRule {
    pub petition_basis_points: u16,
    pub turnout_basis_points: u16,
    /// Removal votes must strictly exceed this share of valid votes cast.
    pub removal_basis_points: u16,
    pub authorization: Attestation,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpecialTerm {
    UnexpiredRemainder,
    FourYears,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecialElectionRule {
    pub term: SpecialTerm,
    pub authorization: Attestation,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub authorization: Attestation,
    pub administrator: String,
    pub credential_reviewers: BTreeSet<String>,
    pub court_reviewers: BTreeSet<String>,
    pub recall: BTreeMap<Office, RecallRule>,
    pub special_elections: BTreeMap<Office, SpecialElectionRule>,
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
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recall {
    pub office: Office,
    pub tenure: String,
    pub roll: ElectoralRoll,
    pub petition: Vec<Attestation>,
    pub ballots: Vec<Vote>,
    pub court_review: Attestation,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Performance {
    pub person: String,
    pub baseline: BTreeMap<TrialDomain, u16>,
    pub final_competencies: BTreeMap<TrialDomain, u16>,
    pub baseline_evidence: Attestation,
    pub improvement_evidence: Attestation,
    pub integration_demonstrated: bool,
    pub imitation_only: bool,
    pub degraded_other_candidate: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LearningTrial {
    pub id: String,
    pub domain: TrialDomain,
    pub candidate: String,
    pub teacher: String,
    pub evidence: Attestation,
    pub candidate_completed_work: bool,
    pub valid: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Teaching {
    pub teacher: String,
    pub sincere: bool,
    pub complete_enough_for_safety: bool,
    pub deliberately_false: bool,
    pub deliberately_dangerous: bool,
    pub certification: Attestation,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Judgment {
    pub vote: Vote,
    pub assessed_improvement: u32,
    pub conflicted: bool,
    pub coerced: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContestProof {
    pub performances: BTreeMap<Office, Performance>,
    pub trials: Vec<LearningTrial>,
    pub teaching: Vec<Teaching>,
    pub spectators: BTreeSet<String>,
    pub roll_certification: Attestation,
    pub judgments: Vec<Judgment>,
    pub court_review: Attestation,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PauseReason {
    GovernorVacancyOrReplacement,
    Incapacity,
    Compromised,
    Tie,
    NoWinner,
    TeachingSabotage,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum ContestStatus {
    Open,
    Paused { reason: PauseReason },
    Completed { winner: String },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Contest {
    pub id: String,
    pub court_case: String,
    pub governor_tenures: BTreeMap<Office, String>,
    /// All governors who have participated, including replaced tenures.
    pub participants: BTreeSet<String>,
    /// Ballot participants across attempts cannot later review this Contest.
    /// Reconstructed from journal commands, not a new persisted archive field.
    pub voters: BTreeSet<String>,
    pub attempt: u64,
    pub status: ContestStatus,
    pub valid_votes: BTreeMap<String, u32>,
    pub excluded_judgments: BTreeSet<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Action {
    Initialize {
        policy: Policy,
        institutional_snapshot: String,
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
    Resign {
        office: Office,
        tenure: String,
    },
    RecordDeath {
        office: Office,
        tenure: String,
    },
    OpenContest {
        id: String,
        court_case: String,
    },
    PauseContest {
        id: String,
        reason: PauseReason,
    },
    ResumeContest {
        id: String,
    },
    CertifyContest {
        id: String,
        proof: ContestProof,
        sovereign_tenure: String,
    },
    /// Explicit versioned clinical policy and authenticated decision records.
    UpdateClinicalAuthorization {
        update: clinical_authorization::Update,
    },
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Tenure {
    pub id: String,
    pub office: Office,
    pub holder: String,
    pub began: u64,
    pub expires: Option<u64>,
    pub ended: Option<u64>,
    pub ending: Option<String>,
    pub closure_event: Option<String>,
    pub imported: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Governance {
    clinical: Authorizations,
    journal: SharedInterfaceRuntime,
    policy: Option<Policy>,
    source_snapshot: String,
    year: u64,
    tenures: Vec<Tenure>,
    qualifications: BTreeSet<(String, Office)>,
    mastery: BTreeSet<(String, String)>,
    expressions: BTreeMap<String, (String, String)>,
    elections: BTreeSet<String>,
    receipts: BTreeSet<String>,
    contests: BTreeMap<String, Contest>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Invalid(&'static str),
    RequiresLegalParameter(&'static str),
    Archive(String),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sandmanor governance rejected: {self:?}")
    }
}
impl std::error::Error for Error {}
fn require(ok: bool, reason: &'static str) -> Result<(), Error> {
    if ok {
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
fn archive(e: impl fmt::Display) -> Error {
    Error::Archive(e.to_string())
}
fn ordinary(form: &str) -> bool {
    matches!(
        form,
        "gnome" | "satyr" | "minotaur" | "hecaton" | "elf" | "faun" | "centaur" | "pegasus"
    )
}
fn majority(counts: &BTreeMap<String, u64>) -> Option<&str> {
    let total = counts.values().map(|n| u128::from(*n)).sum::<u128>();
    counts
        .iter()
        .find(|(_, n)| u128::from(**n) * 2 > total)
        .map(|(s, _)| s.as_str())
}
impl Governance {
    pub fn new(world: &WorldSession, policy: Policy, year: u64) -> Result<Self, Error> {
        let mut evidence = BTreeSet::from([policy.authorization.record.clone()]);
        evidence.extend(
            policy
                .recall
                .values()
                .map(|r| r.authorization.record.clone()),
        );
        evidence.extend(
            policy
                .special_elections
                .values()
                .map(|r| r.authorization.record.clone()),
        );
        let mut result = Self::default();
        result.execute(Command {
            id: "event.sandmanor.governance.initialize".into(),
            expected_revision: 0,
            year,
            actor: policy.authorization.signer.clone(),
            evidence,
            action: Action::Initialize {
                policy,
                institutional_snapshot: world.persisted_state_output(),
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
    pub fn contests(&self) -> &BTreeMap<String, Contest> {
        &self.contests
    }
    pub fn incumbent(&self, office: Office) -> Option<&Tenure> {
        self.tenures.iter().find(|t| {
            t.office == office && t.ended.is_none() && t.expires.is_none_or(|e| self.year < e)
        })
    }
    pub fn may_manifest(&self, person: &str, form: &str) -> bool {
        if form == "sandman" {
            self.incumbent(Office::Sandman)
                .is_some_and(|t| t.holder == person)
        } else {
            self.mastery.contains(&(person.into(), form.into()))
        }
    }
    pub fn expressed_form(&self, person: &str) -> Option<&str> {
        self.expressions.get(person).map(|(f, _)| f.as_str())
    }
    pub fn world_session(&self) -> Result<WorldSession, Error> {
        let source = WorldSession::from_persisted_output(&self.source_snapshot).map_err(archive)?;
        let mut state = source.institutional().clone();
        for h in &mut state.catalog.office_holders {
            if let Some(o) = Office::ALL.iter().find(|o| o.id() == h.office) {
                h.active &= self
                    .incumbent(*o)
                    .is_some_and(|t| t.imported && t.holder == h.being.as_str());
            }
        }
        for t in &self.tenures {
            let h = OfficeHolder {
                office: t.office.id(),
                being: InstitutionalBeingId::new(&t.holder).map_err(archive)?,
                active: self.incumbent(t.office).is_some_and(|c| c.id == t.id),
            };
            if !state.catalog.office_holders.contains(&h) {
                state.catalog.office_holders.push(h);
            }
        }
        WorldSession::from_institutional_state(state).map_err(|e| archive(format!("{e:?}")))
    }
    fn policy(&self) -> Result<&Policy, Error> {
        self.policy
            .as_ref()
            .ok_or(Error::Invalid("not initialized"))
    }
    fn admin(&self, actor: &str) -> Result<(), Error> {
        require(
            self.policy()?.administrator == actor
                && !Office::ALL
                    .iter()
                    .any(|o| self.incumbent(*o).is_some_and(|t| t.holder == actor)),
            "independent administrator required",
        )
    }
    fn court(&self, actor: &str, parties: &BTreeSet<String>) -> Result<(), Error> {
        require(
            self.policy()?.court_reviewers.contains(actor)
                && actor != self.policy()?.administrator
                && !parties.contains(actor)
                && !Office::ALL
                    .iter()
                    .any(|o| self.incumbent(*o).is_some_and(|t| t.holder == actor)),
            "independent Court review required",
        )
    }
    fn credential(&self, reviewer: &str, parties: &BTreeSet<String>) -> Result<(), Error> {
        require(
            self.policy()?.credential_reviewers.contains(reviewer) && !parties.contains(reviewer),
            "independent credential reviewer required",
        )
    }
    fn attest(&self, p: &Attestation, cmd: &Command) -> Result<(), Error> {
        stable(&p.record)?;
        stable(&p.signer)?;
        require(
            cmd.evidence.contains(&p.record),
            "attestation missing from evidence",
        )
    }
    fn roll(&self, r: &ElectoralRoll, office: Office, cmd: &Command) -> Result<(), Error> {
        require(
            r.office == office && !r.eligible.is_empty(),
            "wrong or empty regional roll",
        )?;
        for voter in &r.eligible {
            stable(voter)?;
        }
        self.attest(&r.certification, cmd)?;
        self.admin(&r.certification.signer)
    }
    fn reserve_votes<'a>(&mut self, votes: impl Iterator<Item = &'a Vote>) -> Result<(), Error> {
        for v in votes {
            require(
                self.receipts.insert(v.receipt.record.clone()),
                "ballot receipt reused",
            )?;
        }
        Ok(())
    }
    fn tally(
        &self,
        votes: &[Vote],
        eligible: &BTreeSet<String>,
        choices: &BTreeSet<String>,
        cmd: &Command,
    ) -> Result<BTreeMap<String, u64>, Error> {
        let mut voters = BTreeSet::new();
        let mut result = choices
            .iter()
            .map(|s| (s.clone(), 0))
            .collect::<BTreeMap<_, _>>();
        for v in votes {
            self.attest(&v.receipt, cmd)?;
            require(
                v.receipt.signer == v.voter
                    && eligible.contains(&v.voter)
                    && voters.insert(&v.voter),
                "invalid or duplicate voter",
            )?;
            *result
                .get_mut(&v.candidate)
                .ok_or(Error::Invalid("ineligible ballot choice"))? += 1;
        }
        Ok(result)
    }
    pub fn execute(&mut self, cmd: Command) -> Result<InterfaceApplyOutcome, Error> {
        self.apply_command(cmd, false)
    }

    // Only archive decoding may apply the historical, pre-transition rule.
    fn apply_command(
        &mut self,
        cmd: Command,
        replay: bool,
    ) -> Result<InterfaceApplyOutcome, Error> {
        let event = self.envelope(&cmd)?;
        if let Some(old) = self.journal.event(&event.event_id) {
            return if old == &event {
                Ok(InterfaceApplyOutcome::Idempotent)
            } else {
                Err(Error::Invalid("conflicting event ID"))
            };
        }
        require(cmd.expected_revision == self.revision(), "stale revision")?;
        require(cmd.year >= self.year, "time moved backwards")?;
        let mut next = self.clone();
        next.year = cmd.year;
        for t in &mut next.tenures {
            if t.ended.is_none() && t.expires.is_some_and(|e| e <= next.year) {
                t.ended = t.expires;
                t.ending = Some("term-expired".into());
                t.closure_event = Some(cmd.id.clone());
            }
        }
        next.pause_invalid_contests();
        next.reduce(&cmd, replay)?;
        next.pause_invalid_contests();
        let revert = next
            .expressions
            .iter()
            .filter(|(p, (f, _))| !next.may_manifest(p, f))
            .map(|(p, _)| p.clone())
            .collect::<Vec<_>>();
        for person in revert {
            let body = next
                .expressions
                .get_mut(&person)
                .expect("existing expression");
            body.0.clone_from(&body.1);
        }
        next.journal.apply(event).map_err(archive)?;
        *self = next;
        Ok(InterfaceApplyOutcome::Applied)
    }
    fn reduce(&mut self, cmd: &Command, replay: bool) -> Result<(), Error> {
        stable(&cmd.id)?;
        stable(&cmd.actor)?;
        require(!cmd.evidence.is_empty(), "evidence required")?;
        for id in &cmd.evidence {
            stable(id)?;
        }
        if let Action::Initialize {
            policy,
            institutional_snapshot,
        } = &cmd.action
        {
            require(
                self.policy.is_none() && self.revision() == 0,
                "already initialized",
            )?;
            self.attest(&policy.authorization, cmd)?;
            stable(&policy.administrator)?;
            require(
                policy.authorization.signer == cmd.actor && cmd.actor != policy.administrator,
                "independent policy authorization required",
            )?;
            for person in policy
                .credential_reviewers
                .iter()
                .chain(&policy.court_reviewers)
            {
                stable(person)?;
            }
            let source =
                WorldSession::from_persisted_output(institutional_snapshot).map_err(archive)?;
            self.policy = Some(policy.clone());
            self.source_snapshot = institutional_snapshot.clone();
            for h in &source.institutional().catalog.office_holders {
                if !h.active {
                    continue;
                }
                if Office::GOVERNORS.iter().any(|o| o.id() == h.office) {
                    return Err(Error::RequiresLegalParameter(
                        "legacy regional election and tenure evidence",
                    ));
                }
                if h.office == Office::Sandman.id() {
                    self.tenures.push(Tenure {
                        id: "tenure.sandmanor.imported-sandman".into(),
                        office: Office::Sandman,
                        holder: h.being.as_str().into(),
                        began: cmd.year,
                        expires: None,
                        ended: None,
                        ending: None,
                        closure_event: None,
                        imported: true,
                    });
                }
            }
            for (o, r) in &policy.recall {
                require(
                    *o != Office::Sandman
                        && r.petition_basis_points > 0
                        && r.petition_basis_points <= 10000
                        && r.turnout_basis_points > 0
                        && r.turnout_basis_points <= 10000
                        && r.removal_basis_points > 0
                        && r.removal_basis_points <= 10000,
                    "invalid supplied recall rule",
                )?;
                self.attest(&r.authorization, cmd)?;
                self.court(&r.authorization.signer, &BTreeSet::new())?;
            }
            for (o, r) in &policy.special_elections {
                require(
                    *o != Office::Sandman,
                    "regional special election rule required",
                )?;
                self.attest(&r.authorization, cmd)?;
                self.court(&r.authorization.signer, &BTreeSet::new())?;
            }
            return Ok(());
        }
        self.policy()?;
        match &cmd.action {
            Action::Initialize { .. } => unreachable!(),
            Action::RecognizeMastery { person, form } => {
                stable(person)?;
                require(ordinary(form), "not an ordinary Sandmanor Form")?;
                self.credential(&cmd.actor, &BTreeSet::from([person.clone()]))?;
                self.mastery.insert((person.clone(), form.clone()));
            }
            Action::Qualify { person, office } => {
                stable(person)?;
                require(
                    *office != Office::Sandman && self.may_manifest(person, office.form()),
                    "advanced regional Form mastery required",
                )?;
                self.credential(&cmd.actor, &BTreeSet::from([person.clone()]))?;
                self.qualifications.insert((person.clone(), *office));
            }
            Action::CertifyElection { election } => self.elect(election, cmd)?,
            Action::CertifyRecall { recall } => self.recall(recall, cmd)?,
            Action::Resign { office, tenure } | Action::RecordDeath { office, tenure } => {
                let t = self
                    .incumbent(*office)
                    .ok_or(Error::Invalid("office vacant"))?;
                require(t.id == *tenure, "wrong tenure")?;
                if matches!(cmd.action, Action::Resign { .. }) {
                    require(t.holder == cmd.actor, "only incumbent may resign")?;
                } else {
                    self.court(&cmd.actor, &BTreeSet::from([t.holder.clone()]))?;
                }
                self.end(*office, "lawful-vacancy", cmd)?;
            }
            Action::OpenContest { id, court_case } => {
                stable(id)?;
                stable(court_case)?;
                require(!self.contests.contains_key(id), "contest ID reused")?;
                require(
                    !self
                        .contests
                        .values()
                        .any(|c| !matches!(c.status, ContestStatus::Completed { .. })),
                    "unresolved Contest already exists",
                )?;
                require(
                    self.incumbent(Office::Sandman).is_none(),
                    "sovereign vacancy required",
                )?;
                let pair = self.current_pair()?;
                let participants = self.governor_people();
                self.court(&cmd.actor, &participants)?;
                self.contests.insert(
                    id.clone(),
                    Contest {
                        id: id.clone(),
                        court_case: court_case.clone(),
                        governor_tenures: pair,
                        participants,
                        voters: BTreeSet::new(),
                        attempt: 1,
                        status: ContestStatus::Open,
                        valid_votes: BTreeMap::new(),
                        excluded_judgments: BTreeSet::new(),
                    },
                );
            }
            Action::PauseContest { id, reason } => {
                require(
                    matches!(reason, PauseReason::Incapacity | PauseReason::Compromised),
                    "external pause must record incapacity or compromise",
                )?;
                let c = self
                    .contests
                    .get(id)
                    .ok_or(Error::Invalid("unknown Contest"))?;
                require(
                    !matches!(c.status, ContestStatus::Completed { .. }),
                    "Contest is closed",
                )?;
                let parties = c.participants.union(&c.voters).cloned().collect();
                self.court(&cmd.actor, &parties)?;
                self.contests.get_mut(id).expect("existing Contest").status =
                    ContestStatus::Paused { reason: *reason };
            }
            Action::ResumeContest { id } => {
                let c = self
                    .contests
                    .get(id)
                    .ok_or(Error::Invalid("unknown Contest"))?;
                require(
                    matches!(c.status, ContestStatus::Paused { .. }),
                    "Contest is not paused",
                )?;
                let mut parties = c.participants.clone();
                parties.extend(self.governor_people());
                let court_parties = parties.union(&c.voters).cloned().collect();
                self.court(&cmd.actor, &court_parties)?;
                require(
                    self.incumbent(Office::Sandman).is_none(),
                    "sovereign vacancy required",
                )?;
                let pair = self.current_pair()?;
                let c = self.contests.get_mut(id).expect("existing Contest");
                c.attempt = c
                    .attempt
                    .checked_add(1)
                    .ok_or(Error::Invalid("attempt overflow"))?;
                c.participants = parties;
                c.governor_tenures = pair;
                c.status = ContestStatus::Open;
                c.valid_votes.clear();
                c.excluded_judgments.clear();
            }
            Action::CertifyContest {
                id,
                proof,
                sovereign_tenure,
            } => self.contest(id, proof, sovereign_tenure, cmd)?,
            Action::UpdateClinicalAuthorization { update } => {
                let policy = self
                    .policy
                    .as_ref()
                    .ok_or(Error::Invalid("not initialized"))?;
                self.clinical
                    .apply(
                        update,
                        clinical_authorization::Context {
                            house: House::Sandmanor,
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
                                house: House::Sandmanor,
                                form: form.clone(),
                            },
                            &cmd.actor,
                            cmd.year,
                            clearance,
                            consent,
                        )
                        .map_err(Error::Invalid)?;
                } else if replay {
                    // Preserve accepted history without inventing credentials.
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
        Ok(())
    }
    fn elect(&mut self, e: &Election, cmd: &Command) -> Result<(), Error> {
        self.admin(&cmd.actor)?;
        require(
            e.office != Office::Sandman,
            "Sandman requires Contest of Improvement",
        )?;
        self.attest(&e.certification, cmd)?;
        self.admin(&e.certification.signer)?;
        self.roll(&e.roll, e.office, cmd)?;
        stable(&e.id)?;
        stable(&e.tenure)?;
        require(
            !self.elections.contains(&e.id) && !self.tenures.iter().any(|t| t.id == e.tenure),
            "election or tenure ID reused",
        )?;
        require(
            self.incumbent(e.office).is_none(),
            "regional office occupied",
        )?;
        require(
            !e.candidates.is_empty() && !e.candidates.contains(&cmd.actor),
            "independent administrator required",
        )?;
        for person in &e.candidates {
            require(
                self.qualifications.contains(&(person.clone(), e.office))
                    && self.may_manifest(person, e.office.form()),
                "candidate lacks regional qualification",
            )?;
            require(
                !Office::ALL
                    .iter()
                    .any(|o| self.incumbent(*o).is_some_and(|t| t.holder == *person)),
                "incompatible simultaneous offices",
            )?;
        }
        self.reserve_votes(e.rounds.iter().flatten())?;
        require(
            (1..=2).contains(&e.rounds.len()),
            "first ballot and optional runoff required",
        )?;
        let first = self.tally(&e.rounds[0], &e.roll.eligible, &e.candidates, cmd)?;
        if let Some(winner) = majority(&first) {
            require(
                e.rounds.len() == 1 && winner == e.winner,
                "wrong majority winner or unnecessary runoff",
            )?;
        } else {
            require(
                e.rounds.len() == 2 && !e.rounds[0].is_empty(),
                "runoff required",
            )?;
            let mut ranked = first.iter().collect::<Vec<_>>();
            ranked.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
            require(ranked.len() >= 2, "no runoff pair")?;
            if ranked.len() > 2 && ranked[1].1 == ranked[2].1 {
                return Err(Error::RequiresLegalParameter("runoff qualification tie"));
            }
            let finalists = BTreeSet::from([ranked[0].0.clone(), ranked[1].0.clone()]);
            let last = self.tally(&e.rounds[1], &e.roll.eligible, &finalists, cmd)?;
            require(
                majority(&last) == Some(e.winner.as_str()),
                "no certified runoff majority",
            )?;
        }
        let previous = self.tenures.iter().rev().find(|t| t.office == e.office);
        let early_vacancy = previous
            .and_then(|t| t.expires)
            .filter(|end| cmd.year < *end);
        require(
            e.special == early_vacancy.is_some(),
            "special election must match an unexpired vacancy",
        )?;
        let expires = if let Some(remainder) = early_vacancy {
            let rule = self.policy()?.special_elections.get(&e.office).ok_or(
                Error::RequiresLegalParameter("regional special-election term"),
            )?;
            match rule.term {
                SpecialTerm::UnexpiredRemainder => remainder,
                SpecialTerm::FourYears => cmd
                    .year
                    .checked_add(4)
                    .ok_or(Error::Invalid("term overflow"))?,
            }
        } else {
            cmd.year
                .checked_add(4)
                .ok_or(Error::Invalid("term overflow"))?
        };
        self.tenures.push(Tenure {
            id: e.tenure.clone(),
            office: e.office,
            holder: e.winner.clone(),
            began: cmd.year,
            expires: Some(expires),
            ended: None,
            ending: None,
            closure_event: None,
            imported: false,
        });
        self.elections.insert(e.id.clone());
        Ok(())
    }
    fn recall(&mut self, r: &Recall, cmd: &Command) -> Result<(), Error> {
        self.admin(&cmd.actor)?;
        require(
            r.office != Office::Sandman,
            "regional recall cannot remove the Sandman",
        )?;
        let rule = self
            .policy()?
            .recall
            .get(&r.office)
            .ok_or(Error::RequiresLegalParameter("regional recall thresholds"))?
            .clone();
        self.roll(&r.roll, r.office, cmd)?;
        let t = self
            .incumbent(r.office)
            .ok_or(Error::Invalid("regional vacancy"))?;
        require(t.id == r.tenure, "wrong recall tenure")?;
        let parties = std::iter::once(t.holder.clone())
            .chain(r.petition.iter().map(|p| p.signer.clone()))
            .chain(r.ballots.iter().map(|v| v.voter.clone()))
            .collect();
        self.attest(&r.court_review, cmd)?;
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
            petitioners.len() as u128 * 10000
                >= r.roll.eligible.len() as u128 * u128::from(rule.petition_basis_points),
            "recall petition below lawful threshold",
        )?;
        self.reserve_votes(r.ballots.iter())?;
        let votes = self.tally(
            &r.ballots,
            &r.roll.eligible,
            &BTreeSet::from(["remove".into(), "retain".into()]),
            cmd,
        )?;
        require(
            r.ballots.len() as u128 * 10000
                >= r.roll.eligible.len() as u128 * u128::from(rule.turnout_basis_points),
            "recall turnout below lawful threshold",
        )?;
        if u128::from(votes["remove"]) * 10000
            > r.ballots.len() as u128 * u128::from(rule.removal_basis_points)
        {
            self.end(r.office, "certified-recall", cmd)?;
        }
        Ok(())
    }
    fn governor_people(&self) -> BTreeSet<String> {
        Office::GOVERNORS
            .iter()
            .filter_map(|o| self.incumbent(*o).map(|t| t.holder.clone()))
            .collect()
    }
    fn current_pair(&self) -> Result<BTreeMap<Office, String>, Error> {
        let mut pair = BTreeMap::new();
        for office in Office::GOVERNORS {
            let t = self
                .incumbent(office)
                .ok_or(Error::Invalid("both elected governors required"))?;
            require(
                self.qualifications.contains(&(t.holder.clone(), office))
                    && self.may_manifest(&t.holder, office.form()),
                "qualified governor required",
            )?;
            pair.insert(office, t.id.clone());
        }
        Ok(pair)
    }
    fn pause_invalid_contests(&mut self) {
        let pair = self.current_pair().ok();
        for c in self.contests.values_mut() {
            if matches!(c.status, ContestStatus::Open) && pair.as_ref() != Some(&c.governor_tenures)
            {
                c.status = ContestStatus::Paused {
                    reason: PauseReason::GovernorVacancyOrReplacement,
                };
            }
        }
    }
    fn contest(
        &mut self,
        id: &str,
        proof: &ContestProof,
        tenure: &str,
        cmd: &Command,
    ) -> Result<(), Error> {
        self.admin(&cmd.actor)?;
        let c = self
            .contests
            .get(id)
            .ok_or(Error::Invalid("unknown Contest"))?
            .clone();
        require(
            matches!(c.status, ContestStatus::Open),
            "Contest is not open",
        )?;
        require(
            self.current_pair()? == c.governor_tenures,
            "Contest governor tenures changed",
        )?;
        require(
            self.incumbent(Office::Sandman).is_none(),
            "sovereign vacancy required",
        )?;
        stable(tenure)?;
        require(
            !self.tenures.iter().any(|t| t.id == tenure),
            "tenure ID reused",
        )?;
        require(
            proof.performances.keys().copied().collect::<BTreeSet<_>>()
                == Office::GOVERNORS.into_iter().collect(),
            "both governor performances required",
        )?;
        let candidates = self.governor_people();
        self.attest(&proof.roll_certification, cmd)?;
        self.admin(&proof.roll_certification.signer)?;
        require(
            !proof.spectators.is_empty(),
            "certified spectator roll required",
        )?;
        for p in &proof.spectators {
            stable(p)?;
        }
        let mut parties = c.participants.clone();
        parties.extend(c.voters.iter().cloned());
        parties.extend(proof.judgments.iter().map(|j| j.vote.voter.clone()));
        self.attest(&proof.court_review, cmd)?;
        self.court(&proof.court_review.signer, &parties)?;
        let mut improvement = BTreeMap::new();
        for office in Office::GOVERNORS {
            let p = &proof.performances[&office];
            require(
                self.incumbent(office).is_some_and(|t| t.holder == p.person),
                "performance is not from sitting governor",
            )?;
            for a in [&p.baseline_evidence, &p.improvement_evidence] {
                self.attest(a, cmd)?;
                self.credential(&a.signer, &c.participants)?;
            }
            require(
                p.baseline_evidence.record != p.improvement_evidence.record,
                "distinct baseline and improvement records required",
            )?;
            let person = IdentityId::new(&p.person).map_err(archive)?;
            let baseline = CandidateBaseline {
                candidate: person.clone(),
                competencies: p.baseline.clone(),
                weaknesses: vec![],
                evidence: vec![
                    super::EvidenceId::new(&p.baseline_evidence.record).map_err(archive)?,
                ],
            };
            let final_evidence = ImprovementEvidence {
                candidate: person.clone(),
                final_competencies: p.final_competencies.clone(),
                evidence: vec![
                    super::EvidenceId::new(&p.improvement_evidence.record).map_err(archive)?,
                ],
                integration_demonstrated: p.integration_demonstrated,
                imitation_only: p.imitation_only,
                degraded_other_candidate: p.degraded_other_candidate,
            };
            improvement.insert(
                person.clone(),
                milestone::assess_improvement(&person, &baseline, &final_evidence)
                    .map_err(|e| archive(format!("{e:?}")))?,
            );
        }
        let mut trial_ids = BTreeSet::new();
        let mut trials = Vec::new();
        let mut learned = BTreeSet::new();
        for t in &proof.trials {
            self.attest(&t.evidence, cmd)?;
            self.credential(&t.evidence.signer, &c.participants)?;
            require(trial_ids.insert(&t.id), "duplicate learning trial")?;
            if t.valid && t.candidate_completed_work {
                learned.insert(t.candidate.clone());
            }
            trials.push(PublicTrial {
                id: milestone::TrialId::new(&t.id).map_err(archive)?,
                domain: t.domain,
                candidate: IdentityId::new(&t.candidate).map_err(archive)?,
                teacher: IdentityId::new(&t.teacher).map_err(archive)?,
                evidence: vec![super::EvidenceId::new(&t.evidence.record).map_err(archive)?],
                candidate_completed_work: t.candidate_completed_work,
                valid: t.valid,
            });
        }
        require(
            learned == candidates,
            "both governors must practice and demonstrate learning",
        )?;
        let mut teaching = Vec::new();
        let mut teachers = BTreeSet::new();
        for t in &proof.teaching {
            self.attest(&t.certification, cmd)?;
            self.credential(&t.certification.signer, &c.participants)?;
            require(
                candidates.contains(&t.teacher) && teachers.insert(&t.teacher),
                "exactly one teaching finding per governor required",
            )?;
            teaching.push(TeachingIntegrityFinding {
                teacher: IdentityId::new(&t.teacher).map_err(archive)?,
                sincere: t.sincere,
                complete_enough_for_safety: t.complete_enough_for_safety,
                deliberately_false: t.deliberately_false,
                deliberately_dangerous: t.deliberately_dangerous,
            });
        }
        require(teachers.len() == 2, "both teaching findings required")?;
        self.reserve_votes(proof.judgments.iter().map(|j| &j.vote))?;
        let mut judgments = Vec::new();
        for j in &proof.judgments {
            self.attest(&j.vote.receipt, cmd)?;
            require(
                j.vote.receipt.signer == j.vote.voter,
                "ballot signer mismatch",
            )?;
            judgments.push(CrowdJudgment {
                id: milestone::CrowdJudgmentId::new(&j.vote.receipt.record).map_err(archive)?,
                voter: IdentityId::new(&j.vote.voter).map_err(archive)?,
                candidate: IdentityId::new(&j.vote.candidate).map_err(archive)?,
                assessed_improvement: j.assessed_improvement,
                eligible: proof.spectators.contains(&j.vote.voter),
                conflicted: j.conflicted || c.participants.contains(&j.vote.voter),
                coerced: j.coerced,
            });
        }
        let north = IdentityId::new(&proof.performances[&Office::Mino].person).map_err(archive)?;
        let south = IdentityId::new(&proof.performances[&Office::Cent].person).map_err(archive)?;
        let (tally, verdict) = milestone::evaluate_reciprocal_improvement(
            [&north, &south],
            &improvement,
            &trials,
            &teaching,
            &judgments,
        )
        .map_err(|e| archive(format!("{e:?}")))?;
        let status = match verdict {
            CrowdVerdict::Winner(winner) => {
                let office = if winner == north {
                    Office::Mino
                } else {
                    Office::Cent
                };
                self.end(office, "sandman-accession", cmd)?;
                self.tenures.push(Tenure {
                    id: tenure.into(),
                    office: Office::Sandman,
                    holder: winner.as_str().into(),
                    began: cmd.year,
                    expires: None,
                    ended: None,
                    ending: None,
                    closure_event: None,
                    imported: false,
                });
                ContestStatus::Completed {
                    winner: winner.as_str().into(),
                }
            }
            CrowdVerdict::Tie => ContestStatus::Paused {
                reason: PauseReason::Tie,
            },
            CrowdVerdict::NoWinner => ContestStatus::Paused {
                reason: PauseReason::NoWinner,
            },
            CrowdVerdict::VoidForTeachingSabotage(_) => ContestStatus::Paused {
                reason: PauseReason::TeachingSabotage,
            },
        };
        let c = self.contests.get_mut(id).expect("existing Contest");
        c.status = status;
        c.voters
            .extend(proof.judgments.iter().map(|j| j.vote.voter.clone()));
        c.valid_votes = tally
            .valid_votes
            .into_iter()
            .map(|(id, n)| (id.as_str().into(), n))
            .collect();
        c.excluded_judgments = tally
            .excluded_judgments
            .into_iter()
            .map(|id| id.as_str().into())
            .collect();
        Ok(())
    }
    fn end(&mut self, office: Office, reason: &str, cmd: &Command) -> Result<(), Error> {
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
        t.closure_event = Some(cmd.id.clone());
        Ok(())
    }
    fn envelope(&self, cmd: &Command) -> Result<InterfaceEventEnvelope, Error> {
        let aggregate = InterfaceAggregateId::new(RULE_SET).map_err(archive)?;
        let json = serde_json::to_string(cmd).map_err(archive)?;
        let checksum = json.bytes().fold(0xcbf29ce484222325_u64, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(0x100000001b3)
        });
        // Case references are derivable from prior journal state during replay.
        let case = match &cmd.action {
            Action::OpenContest { court_case, .. } => Some(court_case.as_str()),
            Action::PauseContest { id, .. }
            | Action::ResumeContest { id }
            | Action::CertifyContest { id, .. } => {
                self.contests.get(id).map(|c| c.court_case.as_str())
            }
            _ => None,
        };
        let reference =
            |kind, id: &str| InterfaceIdentityRef::new(kind, id, SOURCE).map_err(archive);
        Ok(InterfaceEventEnvelope {
            event_id: InterfaceEventId::new(&cmd.id).map_err(archive)?,
            source_aggregate: aggregate.clone(),
            receiving_aggregate: aggregate,
            source_domain: InterfaceDomain::House(House::Sandmanor),
            receiving_domain: InterfaceDomain::House(House::Sandmanor),
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
            related_case: case
                .map(InterfaceAggregateId::new)
                .transpose()
                .map_err(archive)?,
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
                ("command".into(), json),
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
                    .ok_or(Error::Invalid("not a Sandmanor governance archive"))?,
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
