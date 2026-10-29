//! Revision 100 civic governance above the shared interface journal.
//! Callers authenticate evidence, legal mandates and clinical clearances.
//! This module grants no clinical privilege and never rewrites legacy Deck XP.
use crate::constitutional::{
    InterfaceAggregateId, InterfaceApplyOutcome, InterfaceDomain, InterfaceEventEnvelope,
    InterfaceEventId, InterfaceEventKind, InterfaceFailureState, InterfaceIdentityKind,
    InterfaceIdentityRef, SHARED_INTERFACE_ARCHIVE_VERSION, SharedInterfaceRuntime,
    decode_shared_interface_archive, encode_shared_interface_archive,
};
use crate::hollow_grove_contract::House;
use crate::institution::{IdentityId, InstitutionId, InstitutionalBeingId, OfficeHolder, OfficeId};
use crate::world::session::WorldSession;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub const SOURCE: &str = "GLAUSHOUSE_REV100_GOVERNANCE_AMENDMENT.md";
pub const RULE_SET: &str = "glaushouse.governance.rev100";
pub const ORDINARY_FORMS: [&str; 8] = [
    "pixie", "sprite", "fairie", "spirit", "siren", "nymph", "sylph", "muse",
];
pub const CARE_ROLES: [(&str, &str, &str); 2] = [
    ("Matron", "Presensor", "Emerald"),
    ("Marshall", "Preceptor", "Jade"),
];

/// Current institutions, deliberately distinct from the legacy five-seat enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MuseHouse {
    Calli,
    Cleo,
    Melanie,
    Sherry,
    Talia,
    Terri,
    Erin,
    Polly,
    Nia,
}
impl MuseHouse {
    pub const ALL: [Self; 9] = [
        Self::Calli,
        Self::Cleo,
        Self::Melanie,
        Self::Sherry,
        Self::Talia,
        Self::Terri,
        Self::Erin,
        Self::Polly,
        Self::Nia,
    ];
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Calli => "calli",
            Self::Cleo => "cleo",
            Self::Melanie => "melanie",
            Self::Sherry => "sherry",
            Self::Talia => "talia",
            Self::Terri => "terri",
            Self::Erin => "erin",
            Self::Polly => "polly",
            Self::Nia => "nia",
        }
    }
    pub const fn name(self) -> &'static str {
        match self {
            Self::Calli => "Calli",
            Self::Cleo => "Cleo",
            Self::Melanie => "Melanie",
            Self::Sherry => "Sherry",
            Self::Talia => "Talia",
            Self::Terri => "Terri",
            Self::Erin => "Erin",
            Self::Polly => "Polly",
            Self::Nia => "Nia",
        }
    }
    pub fn institution_id(self) -> InstitutionId {
        InstitutionId::new(format!("institution.glaushouse.muse-house.{}", self.slug()))
            .expect("canonical ID")
    }
    pub fn office_id(self) -> OfficeId {
        OfficeId::new(format!("office.glaushouse.persephone-de-{}", self.slug()))
            .expect("canonical ID")
    }
}
pub fn house_of_persephone_id() -> InstitutionId {
    InstitutionId::new("institution.glaushouse.house-of-persephone").expect("canonical ID")
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Office {
    Persephone(MuseHouse),
    PrimaDonna,
}
impl Office {
    pub fn all() -> impl Iterator<Item = Self> {
        MuseHouse::ALL
            .into_iter()
            .map(Self::Persephone)
            .chain([Self::PrimaDonna])
    }
    pub fn id(self) -> OfficeId {
        match self {
            Self::Persephone(h) => h.office_id(),
            Self::PrimaDonna => super::prima_donna_office_id(),
        }
    }
}

/// Zero-based day within a supplied legal calendar; no real-world conversion.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Date {
    pub year: u64,
    pub day: u64,
}
impl Date {
    fn after_years(self, years: u64) -> Result<Self, Error> {
        Ok(Self {
            year: self
                .year
                .checked_add(years)
                .ok_or(Error::Invalid("year overflow"))?,
            day: self.day,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Attestation {
    pub record: String,
    pub signer: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub authorization: Attestation,
    pub administrator: String,
    pub credential_reviewers: BTreeSet<String>,
    pub court_reviewers: BTreeSet<String>,
    pub clearance_reviewers: BTreeSet<String>,
    pub days_per_year: Option<u64>,
    /// Explicit assignment of three seats to each of three two-year cohorts.
    pub first_election_years: BTreeMap<MuseHouse, u64>,
    pub special_elections_fill_remainder: Option<Attestation>,
    /// The canon does not settle the seat of a Persephone who wins The Call.
    pub corination_vacates_own_seat: Option<Attestation>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ElectoralRoll {
    pub house: MuseHouse,
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
    pub house: MuseHouse,
    pub candidates: BTreeSet<String>,
    pub roll: ElectoralRoll,
    pub rounds: Vec<Vec<Vote>>,
    pub winner: String,
    pub tenure: String,
    pub begins: Date,
    pub expires: Date,
    pub special: bool,
    pub certification: Attestation,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ElectionStatus {
    Pending,
    Vetoed,
    Overturned,
    Rejected,
    Installed,
    Expired,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ElectionRecord {
    pub election: Election,
    pub posted: Date,
    pub status: ElectionStatus,
    pub participants: BTreeSet<String>,
    pub court_case: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recall {
    pub house: MuseHouse,
    pub tenure: String,
    pub court_case: String,
    pub roll: ElectoralRoll,
    pub petition: Vec<Attestation>,
    pub ballots: Vec<Vote>,
    pub court_review: Attestation,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeatVote {
    pub house: MuseHouse,
    pub tenure: String,
    pub choice: Option<String>,
    pub receipt: Attestation,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Audition {
    pub evaluation: Attestation,
    /// This governance demonstration never authorizes a patient intervention.
    pub non_patient_exposing: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Testimony {
    pub service_record: Attestation,
    pub witness_record: Attestation,
    /// Authenticated lawful admission, need-to-know handling and public redaction.
    pub privacy_review: Attestation,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CallStage {
    Pitch,
    Audition,
    Testimony,
    Ovation,
    Corination,
}
impl CallStage {
    pub const ALL: [Self; 5] = [
        Self::Pitch,
        Self::Audition,
        Self::Testimony,
        Self::Ovation,
        Self::Corination,
    ];
    pub const fn name(self) -> &'static str {
        match self {
            Self::Pitch => "The Pitch",
            Self::Audition => "Audition",
            Self::Testimony => "Testimony",
            Self::Ovation => "Ovation",
            Self::Corination => "Corination",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Call {
    pub id: String,
    pub court_case: String,
    pub candidates: BTreeSet<String>,
    pub participants: BTreeSet<String>,
    pub stage: CallStage,
    pub rounds: u64,
    pub selected: Option<String>,
    pub last_ballot: Vec<SeatVote>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Removal {
    pub tenure: String,
    pub court_case: String,
    pub grounds: Attestation,
    pub notice_and_response: Attestation,
    pub appeals_resolved: Attestation,
    pub court_review: Attestation,
    pub ballots: Vec<SeatVote>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Action {
    Initialize {
        policy: Policy,
        institutional_snapshot: String,
    },
    AdmitNightingale {
        person: String,
    },
    RecordAgency {
        person: String,
        house: MuseHouse,
    },
    RecognizeMastery {
        person: String,
        form: String,
    },
    Qualify {
        person: String,
        office: Office,
    },
    ProposeElection {
        election: Election,
    },
    VetoElection {
        election: String,
        constitutional_ground: String,
    },
    ReviewVeto {
        election: String,
        court_case: String,
        uphold: bool,
    },
    CertifyElection {
        election: String,
    },
    CertifyRecall {
        recall: Recall,
    },
    OpenCall {
        id: String,
        court_case: String,
        candidates: BTreeSet<String>,
        screening: Attestation,
    },
    Audition {
        id: String,
        reviews: BTreeMap<String, Audition>,
    },
    Testimony {
        id: String,
        reviews: BTreeMap<String, Testimony>,
    },
    Ovation {
        id: String,
        ballots: Vec<SeatVote>,
    },
    Corinate {
        id: String,
        tenure: String,
        certification: Attestation,
        appeals_resolved: Attestation,
    },
    RemoveSovereign {
        removal: Removal,
    },
    Resign {
        office: Office,
        tenure: String,
    },
    RecordDeath {
        office: Office,
        tenure: String,
    },
    Manifest {
        form: String,
        ordinary_reversion: String,
        clearance: Attestation,
        consent: Attestation,
    },
    ObserveDate,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Command {
    pub id: String,
    pub expected_revision: u64,
    pub at: Date,
    pub actor: String,
    pub evidence: BTreeSet<String>,
    pub action: Action,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Tenure {
    pub id: String,
    pub office: Office,
    pub holder: String,
    pub began: Date,
    /// Actual authority starts at certification, never retroactively at a
    /// nominal term boundary that passed while the election was under review.
    pub installed: Date,
    pub expires: Date,
    pub ended: Option<Date>,
    pub ending: Option<String>,
    pub closure_event: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Governance {
    journal: SharedInterfaceRuntime,
    policy: Option<Policy>,
    source_snapshot: String,
    at: Date,
    citizens: BTreeSet<String>,
    agencies: BTreeMap<String, MuseHouse>,
    mastery: BTreeSet<(String, String)>,
    qualifications: BTreeSet<(String, Office)>,
    tenures: Vec<Tenure>,
    elections: BTreeMap<String, ElectionRecord>,
    receipts: BTreeSet<String>,
    calls: BTreeMap<String, Call>,
    /// Case -> (target tenure, all prior participants), for independent reviews.
    removal_cases: BTreeMap<String, (String, BTreeSet<String>)>,
    expressions: BTreeMap<String, (String, String)>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Invalid(&'static str),
    RequiresLegalParameter(&'static str),
    Archive(String),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Glaushouse governance rejected: {self:?}")
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
fn stable(id: &str) -> Result<(), Error> {
    IdentityId::new(id)
        .map(|_| ())
        .map_err(|_| Error::Invalid("invalid stable ID"))
}
fn archive(e: impl fmt::Display) -> Error {
    Error::Archive(e.to_string())
}
fn ordinary(form: &str) -> bool {
    ORDINARY_FORMS.contains(&form)
}
fn majority(votes: &BTreeMap<String, u64>) -> Option<&str> {
    let total: u128 = votes.values().map(|v| u128::from(*v)).sum();
    votes
        .iter()
        .find(|(_, n)| u128::from(**n) * 2 > total)
        .map(|(p, _)| p.as_str())
}

impl Governance {
    pub fn new(world: &WorldSession, policy: Policy, at: Date) -> Result<Self, Error> {
        let mut evidence = BTreeSet::from([policy.authorization.record.clone()]);
        for proof in [
            &policy.special_elections_fill_remainder,
            &policy.corination_vacates_own_seat,
        ]
        .into_iter()
        .flatten()
        {
            evidence.insert(proof.record.clone());
        }
        let mut g = Self::default();
        g.execute(Command {
            id: "event.glaushouse.governance.initialize".into(),
            expected_revision: 0,
            at,
            actor: policy.authorization.signer.clone(),
            evidence,
            action: Action::Initialize {
                policy,
                institutional_snapshot: world.persisted_state_output(),
            },
        })?;
        Ok(g)
    }
    pub fn revision(&self) -> u64 {
        self.journal.events().len() as u64
    }
    pub const fn date(&self) -> Date {
        self.at
    }
    pub fn tenures(&self) -> &[Tenure] {
        &self.tenures
    }
    pub fn calls(&self) -> &BTreeMap<String, Call> {
        &self.calls
    }
    pub fn elections(&self) -> &BTreeMap<String, ElectionRecord> {
        &self.elections
    }
    pub fn is_nightingale(&self, person: &str) -> bool {
        self.citizens.contains(person)
    }
    pub fn agency(&self, person: &str) -> Option<MuseHouse> {
        self.agencies.get(person).copied()
    }
    pub fn incumbent(&self, office: Office) -> Option<&Tenure> {
        self.tenures
            .iter()
            .find(|t| t.office == office && t.ended.is_none() && self.at < t.expires)
    }
    pub fn has_mastery(&self, person: &str, form: &str) -> bool {
        self.mastery.contains(&(person.into(), form.into()))
    }
    pub fn may_manifest(&self, person: &str, form: &str) -> bool {
        match form {
            "prima-donna" => self
                .incumbent(Office::PrimaDonna)
                .is_some_and(|t| t.holder == person),
            "persephone" => {
                self.has_mastery(person, form)
                    && MuseHouse::ALL.iter().any(|h| {
                        self.incumbent(Office::Persephone(*h))
                            .is_some_and(|t| t.holder == person)
                    })
            }
            _ => ordinary(form) && self.has_mastery(person, form),
        }
    }
    pub fn expressed_form(&self, person: &str) -> Option<&str> {
        self.expressions.get(person).map(|(f, _)| f.as_str())
    }
    pub fn world_session(&self) -> Result<WorldSession, Error> {
        let world = WorldSession::from_persisted_output(&self.source_snapshot).map_err(archive)?;
        let mut state = world.institutional().clone();
        for h in &mut state.catalog.office_holders {
            if Office::all().any(|o| o.id() == h.office) {
                h.active = false;
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
            actor == self.policy()?.administrator && !self.is_officer(actor),
            "independent administrator required",
        )
    }
    fn is_officer(&self, actor: &str) -> bool {
        Office::all().any(|o| self.incumbent(o).is_some_and(|t| t.holder == actor))
    }
    fn court(&self, actor: &str, parties: &BTreeSet<String>) -> Result<(), Error> {
        require(
            self.policy()?.court_reviewers.contains(actor)
                && actor != self.policy()?.administrator
                && !parties.contains(actor)
                && !self.is_officer(actor),
            "independent Court review required",
        )
    }
    fn credential(&self, actor: &str, parties: &BTreeSet<String>) -> Result<(), Error> {
        require(
            self.policy()?.credential_reviewers.contains(actor) && !parties.contains(actor),
            "independent credential reviewer required",
        )
    }
    fn attest(&self, proof: &Attestation, cmd: &Command) -> Result<(), Error> {
        stable(&proof.record)?;
        stable(&proof.signer)?;
        require(
            cmd.evidence.contains(&proof.record),
            "attestation missing from evidence",
        )
    }
    fn distinct_proofs(&self, proofs: &[&Attestation], cmd: &Command) -> Result<(), Error> {
        let mut seen = BTreeSet::new();
        for p in proofs {
            self.attest(p, cmd)?;
            require(seen.insert(&p.record), "distinct proof records required")?;
        }
        Ok(())
    }
    fn validate_date(&self, at: Date) -> Result<(), Error> {
        if let Some(days) = self.policy()?.days_per_year {
            require(days > 0 && at.day < days, "invalid calendar day")
        } else if at.day != 0 {
            Err(Error::RequiresLegalParameter("in-world calendar"))
        } else {
            Ok(())
        }
    }
    fn day_number(&self, at: Date) -> Result<u64, Error> {
        self.validate_date(at)?;
        let days = self
            .policy()?
            .days_per_year
            .ok_or(Error::RequiresLegalParameter(
                "in-world calendar for fourteen-day veto",
            ))?;
        at.year
            .checked_mul(days)
            .and_then(|n| n.checked_add(at.day))
            .ok_or(Error::Invalid("calendar overflow"))
    }
    fn eligible(&self, person: &str, office: Office) -> bool {
        self.citizens.contains(person)
            && self.has_mastery(person, "muse")
            && self.qualifications.contains(&(person.into(), office))
            && match office {
                Office::PrimaDonna => true,
                Office::Persephone(h) => {
                    self.agency(person) == Some(h) && self.has_mastery(person, "persephone")
                }
            }
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
        t.ended = Some(cmd.at);
        t.ending = Some(reason.into());
        t.closure_event = Some(cmd.id.clone());
        Ok(())
    }
    pub fn execute(&mut self, cmd: Command) -> Result<InterfaceApplyOutcome, Error> {
        let event = self.envelope(&cmd)?;
        if let Some(old) = self.journal.event(&event.event_id) {
            return if old == &event {
                Ok(InterfaceApplyOutcome::Idempotent)
            } else {
                Err(Error::Invalid("conflicting event ID"))
            };
        }
        require(cmd.expected_revision == self.revision(), "stale revision")?;
        require(cmd.at >= self.at, "time moved backwards")?;
        let mut next = self.clone();
        next.at = cmd.at;
        if next.policy.is_some() {
            next.validate_date(cmd.at)?;
        }
        for t in &mut next.tenures {
            if t.ended.is_none() && t.expires <= cmd.at {
                t.ended = Some(t.expires);
                t.ending = Some("term-expired".into());
                t.closure_event = Some(cmd.id.clone());
            }
        }
        for record in next.elections.values_mut() {
            if record.election.expires <= cmd.at
                && matches!(
                    record.status,
                    ElectionStatus::Pending | ElectionStatus::Vetoed | ElectionStatus::Overturned
                )
            {
                record.status = ElectionStatus::Expired;
            }
        }
        next.reduce(&cmd)?;
        let revoke = next
            .expressions
            .iter()
            .filter(|(p, (f, _))| !next.may_manifest(p, f))
            .map(|(p, _)| p.clone())
            .collect::<Vec<_>>();
        for p in revoke {
            let body = next.expressions.get_mut(&p).expect("expression");
            body.0.clone_from(&body.1);
        }
        next.journal.apply(event).map_err(archive)?;
        *self = next;
        Ok(InterfaceApplyOutcome::Applied)
    }
    fn reduce(&mut self, cmd: &Command) -> Result<(), Error> {
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
            for p in policy
                .credential_reviewers
                .iter()
                .chain(&policy.court_reviewers)
                .chain(&policy.clearance_reviewers)
            {
                stable(p)?;
            }
            require(policy.days_per_year != Some(0), "empty calendar year")?;
            if !policy.first_election_years.is_empty() {
                require(
                    policy.first_election_years.len() == 9,
                    "complete nine-seat cohort assignment required",
                )?;
                let first = *policy
                    .first_election_years
                    .values()
                    .min()
                    .expect("nonempty");
                let mut groups = BTreeMap::<u64, usize>::new();
                for year in policy.first_election_years.values() {
                    *groups.entry(year - first).or_default() += 1;
                }
                require(
                    groups == BTreeMap::from([(0, 3), (2, 3), (4, 3)]),
                    "three equal cohorts staggered two years required",
                )?;
            }
            let source =
                WorldSession::from_persisted_output(institutional_snapshot).map_err(archive)?;
            if source
                .institutional()
                .catalog
                .office_holders
                .iter()
                .any(|h| h.active && Office::all().any(|o| o.id() == h.office))
            {
                return Err(Error::RequiresLegalParameter(
                    "legacy Glaushouse election and term reconciliation",
                ));
            }
            self.policy = Some(policy.clone());
            self.source_snapshot = institutional_snapshot.clone();
            self.validate_date(cmd.at)?;
            for proof in [
                &policy.special_elections_fill_remainder,
                &policy.corination_vacates_own_seat,
            ]
            .into_iter()
            .flatten()
            {
                self.attest(proof, cmd)?;
                self.court(&proof.signer, &BTreeSet::new())?;
            }
            return Ok(());
        }
        self.policy()?;
        match &cmd.action {
            Action::Initialize { .. } => unreachable!(),
            Action::AdmitNightingale { person } => {
                stable(person)?;
                self.credential(&cmd.actor, &BTreeSet::from([person.clone()]))?;
                self.citizens.insert(person.clone());
            }
            Action::RecordAgency { person, house } => {
                require(self.is_nightingale(person), "admitted Nightingale required")?;
                self.credential(&cmd.actor, &BTreeSet::from([person.clone()]))?;
                require(
                    MuseHouse::ALL.iter().all(|h| {
                        self.incumbent(Office::Persephone(*h))
                            .is_none_or(|t| t.holder != *person || h == house)
                    }),
                    "incumbent must retain originating Agency",
                )?;
                self.agencies.insert(person.clone(), *house);
            }
            Action::RecognizeMastery { person, form } => {
                stable(person)?;
                require(
                    ordinary(form) || form == "persephone",
                    "not an earnable Glaushouse Form",
                )?;
                require(
                    form != "persephone" || self.has_mastery(person, "muse"),
                    "post-Muse mastery required",
                )?;
                self.credential(&cmd.actor, &BTreeSet::from([person.clone()]))?;
                self.mastery.insert((person.clone(), form.clone()));
            }
            Action::Qualify { person, office } => {
                require(
                    self.is_nightingale(person) && self.has_mastery(person, "muse"),
                    "admitted Muse master required",
                )?;
                if let Office::Persephone(h) = office {
                    require(
                        self.agency(person) == Some(*h) && self.has_mastery(person, "persephone"),
                        "originating Agency and advanced mastery required",
                    )?;
                }
                self.credential(&cmd.actor, &BTreeSet::from([person.clone()]))?;
                self.qualifications.insert((person.clone(), *office));
            }
            Action::ProposeElection { election } => self.propose(election, cmd)?,
            Action::VetoElection {
                election,
                constitutional_ground,
            } => {
                let sovereign = self
                    .incumbent(Office::PrimaDonna)
                    .ok_or(Error::Invalid("sovereign vacancy grants no veto"))?;
                require(
                    sovereign.holder == cmd.actor,
                    "only incumbent Prima Donna may veto",
                )?;
                require(
                    cmd.evidence.contains(constitutional_ground),
                    "documented constitutional ground required",
                )?;
                let record = self
                    .elections
                    .get(election)
                    .ok_or(Error::Invalid("unknown election"))?;
                require(
                    record.status == ElectionStatus::Pending
                        && !record.participants.contains(&cmd.actor),
                    "unavailable or conflicted veto",
                )?;
                require(
                    self.day_number(cmd.at)? - self.day_number(record.posted)? < 14,
                    "fourteen-day veto window closed",
                )?;
                let record = self.elections.get_mut(election).expect("election");
                record.status = ElectionStatus::Vetoed;
                record.participants.insert(cmd.actor.clone());
            }
            Action::ReviewVeto {
                election,
                court_case,
                uphold,
            } => {
                stable(court_case)?;
                let record = self
                    .elections
                    .get(election)
                    .ok_or(Error::Invalid("unknown election"))?;
                require(
                    record.status == ElectionStatus::Vetoed,
                    "no unresolved veto",
                )?;
                self.court(&cmd.actor, &record.participants)?;
                let record = self.elections.get_mut(election).expect("election");
                record.court_case = Some(court_case.clone());
                record.status = if *uphold {
                    ElectionStatus::Rejected
                } else {
                    ElectionStatus::Overturned
                };
            }
            Action::CertifyElection { election } => self.install(election, cmd)?,
            Action::CertifyRecall { recall } => self.recall(recall, cmd)?,
            Action::OpenCall {
                id,
                court_case,
                candidates,
                screening,
            } => {
                self.admin(&cmd.actor)?;
                stable(id)?;
                stable(court_case)?;
                require(
                    !self.calls.contains_key(id)
                        && self
                            .calls
                            .values()
                            .all(|c| c.stage == CallStage::Corination),
                    "Call already exists or remains unresolved",
                )?;
                require(
                    !candidates.is_empty() && !candidates.contains(&cmd.actor),
                    "eligible candidates and independent administrator required",
                )?;
                for p in candidates {
                    require(
                        self.eligible(p, Office::PrimaDonna),
                        "candidate lacks independent leadership qualification",
                    )?;
                }
                self.attest(screening, cmd)?;
                self.court(&screening.signer, candidates)?;
                self.calls.insert(
                    id.clone(),
                    Call {
                        id: id.clone(),
                        court_case: court_case.clone(),
                        candidates: candidates.clone(),
                        participants: candidates.clone(),
                        stage: CallStage::Pitch,
                        rounds: 0,
                        selected: None,
                        last_ballot: vec![],
                    },
                );
            }
            Action::Audition { id, reviews } => {
                self.admin(&cmd.actor)?;
                let c = self.calls.get(id).ok_or(Error::Invalid("unknown Call"))?;
                require(
                    c.stage == CallStage::Pitch
                        && reviews.keys().cloned().collect::<BTreeSet<_>>() == c.candidates,
                    "Audition follows The Pitch for all candidates",
                )?;
                for review in reviews.values() {
                    require(
                        review.non_patient_exposing,
                        "Audition grants no patient intervention authority",
                    )?;
                    self.attest(&review.evaluation, cmd)?;
                    self.credential(&review.evaluation.signer, &c.participants)?;
                }
                self.calls.get_mut(id).expect("Call").stage = CallStage::Audition;
            }
            Action::Testimony { id, reviews } => {
                self.admin(&cmd.actor)?;
                let c = self.calls.get(id).ok_or(Error::Invalid("unknown Call"))?;
                require(
                    c.stage == CallStage::Audition
                        && reviews.keys().cloned().collect::<BTreeSet<_>>() == c.candidates,
                    "Testimony follows Audition for all candidates",
                )?;
                for review in reviews.values() {
                    self.distinct_proofs(
                        &[
                            &review.service_record,
                            &review.witness_record,
                            &review.privacy_review,
                        ],
                        cmd,
                    )?;
                    self.credential(&review.service_record.signer, &c.participants)?;
                    require(
                        !c.participants.contains(&review.witness_record.signer),
                        "candidate cannot witness own Testimony",
                    )?;
                    self.court(&review.privacy_review.signer, &c.participants)?;
                }
                self.calls.get_mut(id).expect("Call").stage = CallStage::Testimony;
            }
            Action::Ovation { id, ballots } => self.ovation(id, ballots, cmd)?,
            Action::Corinate {
                id,
                tenure,
                certification,
                appeals_resolved,
            } => self.corinate(id, tenure, certification, appeals_resolved, cmd)?,
            Action::RemoveSovereign { removal } => self.remove_sovereign(removal, cmd)?,
            Action::Resign { office, tenure } | Action::RecordDeath { office, tenure } => {
                let t = self
                    .incumbent(*office)
                    .ok_or(Error::Invalid("office vacant"))?;
                require(t.id == *tenure, "wrong tenure")?;
                if matches!(cmd.action, Action::Resign { .. }) {
                    require(cmd.actor == t.holder, "only incumbent may resign")?;
                } else {
                    self.court(&cmd.actor, &BTreeSet::from([t.holder.clone()]))?;
                }
                self.end(*office, "lawful-vacancy", cmd)?;
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
                        && self.has_mastery(&cmd.actor, ordinary_reversion),
                    "earned ordinary reversion required",
                )?;
                self.distinct_proofs(&[clearance, consent], cmd)?;
                require(
                    consent.signer == cmd.actor
                        && clearance.signer != cmd.actor
                        && self
                            .policy()?
                            .clearance_reviewers
                            .contains(&clearance.signer),
                    "independent clinical clearance and personal consent required",
                )?;
                self.expressions.insert(
                    cmd.actor.clone(),
                    (form.clone(), ordinary_reversion.clone()),
                );
            }
            Action::ObserveDate => self.admin(&cmd.actor)?,
        }
        Ok(())
    }
    fn roll(&self, r: &ElectoralRoll, house: MuseHouse, cmd: &Command) -> Result<(), Error> {
        require(
            r.house == house && !r.eligible.is_empty(),
            "wrong or empty professional roll",
        )?;
        for p in &r.eligible {
            stable(p)?;
        }
        self.attest(&r.certification, cmd)?;
        self.admin(&r.certification.signer)
    }
    fn reserve(&mut self, proof: &Attestation, cmd: &Command) -> Result<(), Error> {
        self.attest(proof, cmd)?;
        require(
            self.receipts.insert(proof.record.clone()),
            "ballot receipt reused",
        )
    }
    fn tally(
        &mut self,
        ballots: &[Vote],
        roll: &ElectoralRoll,
        choices: &BTreeSet<String>,
        cmd: &Command,
    ) -> Result<BTreeMap<String, u64>, Error> {
        let mut voters = BTreeSet::new();
        let mut counts = choices
            .iter()
            .map(|s| (s.clone(), 0))
            .collect::<BTreeMap<_, _>>();
        for v in ballots {
            self.reserve(&v.receipt, cmd)?;
            require(
                v.receipt.signer == v.voter
                    && roll.eligible.contains(&v.voter)
                    && voters.insert(&v.voter),
                "invalid or duplicate voter",
            )?;
            *counts
                .get_mut(&v.candidate)
                .ok_or(Error::Invalid("ineligible ballot choice"))? += 1;
        }
        Ok(counts)
    }
    fn propose(&mut self, e: &Election, cmd: &Command) -> Result<(), Error> {
        self.admin(&cmd.actor)?;
        self.attest(&e.certification, cmd)?;
        self.admin(&e.certification.signer)?;
        self.roll(&e.roll, e.house, cmd)?;
        stable(&e.id)?;
        stable(&e.tenure)?;
        require(
            !self.elections.contains_key(&e.id)
                && !self.tenures.iter().any(|t| t.id == e.tenure)
                && !self
                    .elections
                    .values()
                    .any(|r| r.election.tenure == e.tenure),
            "election or tenure ID reused",
        )?;
        require(
            !self.elections.values().any(|r| {
                r.election.house == e.house
                    && matches!(
                        r.status,
                        ElectionStatus::Pending
                            | ElectionStatus::Vetoed
                            | ElectionStatus::Overturned
                    )
            }),
            "unresolved election for seat",
        )?;
        self.day_number(cmd.at)?;
        self.validate_date(e.begins)?;
        self.validate_date(e.expires)?;
        require(
            cmd.at <= e.begins && e.begins < e.expires,
            "invalid certified term dates",
        )?;
        let epoch = *self
            .policy()?
            .first_election_years
            .get(&e.house)
            .ok_or(Error::RequiresLegalParameter("first election cohorts"))?;
        if e.special {
            self.policy()?
                .special_elections_fill_remainder
                .as_ref()
                .ok_or(Error::RequiresLegalParameter(
                    "special election term disposition",
                ))?;
            require(
                self.incumbent(Office::Persephone(e.house)).is_none(),
                "special election requires vacancy",
            )?;
            let previous = self
                .tenures
                .iter()
                .rev()
                .find(|t| t.office == Office::Persephone(e.house))
                .ok_or(Error::RequiresLegalParameter("special election remainder"))?;
            require(
                e.expires == previous.expires && e.begins < previous.expires,
                "special election cannot extend remainder",
            )?;
        } else {
            require(
                e.begins.year >= epoch
                    && (e.begins.year - epoch).is_multiple_of(6)
                    && e.expires == e.begins.after_years(6)?,
                "six-year cohort term required",
            )?;
            require(
                self.incumbent(Office::Persephone(e.house))
                    .is_none_or(|t| t.expires <= e.begins),
                "election cannot shorten incumbent term",
            )?;
            require(
                self.tenures
                    .iter()
                    .rev()
                    .find(|t| t.office == Office::Persephone(e.house))
                    .is_none_or(|t| t.expires <= e.begins),
                "unexpired vacancy requires special election",
            )?;
        }
        require(
            !e.candidates.is_empty() && !e.candidates.contains(&cmd.actor),
            "independent administrator required",
        )?;
        for p in &e.candidates {
            require(
                self.eligible(p, Office::Persephone(e.house)),
                "candidate lacks originating Agency or advanced qualification",
            )?;
            require(
                Office::all()
                    .filter(|o| *o != Office::Persephone(e.house))
                    .all(|o| self.incumbent(o).is_none_or(|t| t.holder != *p)),
                "incompatible simultaneous offices",
            )?;
        }
        require(
            (1..=2).contains(&e.rounds.len()),
            "first ballot and optional runoff required",
        )?;
        let first = self.tally(&e.rounds[0], &e.roll, &e.candidates, cmd)?;
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
            let last = self.tally(&e.rounds[1], &e.roll, &finalists, cmd)?;
            require(
                majority(&last) == Some(e.winner.as_str()),
                "no certified runoff majority",
            )?;
        }
        let mut participants = e.candidates.clone();
        participants.extend(e.rounds.iter().flatten().map(|v| v.voter.clone()));
        if let Some(t) = self.incumbent(Office::Persephone(e.house)) {
            participants.insert(t.holder.clone());
        }
        self.elections.insert(
            e.id.clone(),
            ElectionRecord {
                election: e.clone(),
                posted: cmd.at,
                status: ElectionStatus::Pending,
                participants,
                court_case: None,
            },
        );
        Ok(())
    }
    fn install(&mut self, id: &str, cmd: &Command) -> Result<(), Error> {
        self.admin(&cmd.actor)?;
        let r = self
            .elections
            .get(id)
            .ok_or(Error::Invalid("unknown election"))?
            .clone();
        require(
            r.status == ElectionStatus::Overturned
                || (r.status == ElectionStatus::Pending
                    && self.day_number(cmd.at)? - self.day_number(r.posted)? >= 14),
            "election remains under review or rejected",
        )?;
        let e = r.election;
        let office = Office::Persephone(e.house);
        require(
            self.incumbent(office).is_none() && e.begins <= cmd.at && cmd.at < e.expires,
            "vacancy within certified term required",
        )?;
        require(
            self.eligible(&e.winner, office) && !self.is_officer(&e.winner),
            "winner no longer eligible or holds another office",
        )?;
        self.tenures.push(Tenure {
            id: e.tenure,
            office,
            holder: e.winner,
            began: e.begins,
            installed: cmd.at,
            expires: e.expires,
            ended: None,
            ending: None,
            closure_event: None,
        });
        self.elections.get_mut(id).expect("election").status = ElectionStatus::Installed;
        Ok(())
    }
    fn recall(&mut self, r: &Recall, cmd: &Command) -> Result<(), Error> {
        self.admin(&cmd.actor)?;
        self.roll(&r.roll, r.house, cmd)?;
        stable(&r.court_case)?;
        let office = Office::Persephone(r.house);
        let t = self
            .incumbent(office)
            .ok_or(Error::Invalid("seat vacant"))?;
        require(t.id == r.tenure, "wrong recall tenure")?;
        let mut parties = BTreeSet::from([t.holder.clone()]);
        parties.extend(r.petition.iter().map(|p| p.signer.clone()));
        parties.extend(r.ballots.iter().map(|v| v.voter.clone()));
        self.removal_parties(&r.court_case, &r.tenure, &mut parties)?;
        self.attest(&r.court_review, cmd)?;
        self.court(&r.court_review.signer, &parties)?;
        let mut people = BTreeSet::new();
        let mut records = BTreeSet::new();
        for p in &r.petition {
            self.attest(p, cmd)?;
            require(
                r.roll.eligible.contains(&p.signer)
                    && people.insert(&p.signer)
                    && records.insert(&p.record),
                "invalid or duplicate petitioner",
            )?;
        }
        require(
            people.len() as u128 * 5 >= r.roll.eligible.len() as u128,
            "recall requires twenty-percent petition",
        )?;
        let votes = self.tally(
            &r.ballots,
            &r.roll,
            &BTreeSet::from(["remove".into(), "retain".into()]),
            cmd,
        )?;
        require(
            r.ballots.len() as u128 * 5 >= r.roll.eligible.len() as u128 * 2,
            "recall requires forty-percent turnout",
        )?;
        self.removal_cases
            .insert(r.court_case.clone(), (r.tenure.clone(), parties));
        if u128::from(votes["remove"]) * 2 > r.ballots.len() as u128 {
            self.end(office, "certified-recall", cmd)?;
        }
        Ok(())
    }
    fn removal_parties(
        &self,
        case: &str,
        tenure: &str,
        parties: &mut BTreeSet<String>,
    ) -> Result<(), Error> {
        if let Some((target, previous)) = self.removal_cases.get(case) {
            require(target == tenure, "Court case target changed")?;
            parties.extend(previous.iter().cloned());
        }
        Ok(())
    }
    fn seat_tally(
        &self,
        ballots: &[SeatVote],
        choices: &BTreeSet<String>,
    ) -> Result<BTreeMap<String, u64>, Error> {
        let mut seats = BTreeSet::new();
        let mut receipts = BTreeSet::new();
        let mut people = BTreeSet::new();
        let mut counts = choices
            .iter()
            .map(|s| (s.clone(), 0))
            .collect::<BTreeMap<_, _>>();
        for v in ballots {
            let t = self
                .incumbent(Office::Persephone(v.house))
                .ok_or(Error::Invalid("vacant seats cannot vote"))?;
            require(
                t.id == v.tenure
                    && t.holder == v.receipt.signer
                    && seats.insert(v.house)
                    && receipts.insert(&v.receipt.record)
                    && people.insert(&v.receipt.signer),
                "one ballot per current elected seat required",
            )?;
            require(
                self.eligible(&t.holder, t.office),
                "elector no longer qualified",
            )?;
            if let Some(choice) = &v.choice {
                *counts
                    .get_mut(choice)
                    .ok_or(Error::Invalid("invalid seat ballot choice"))? += 1;
            }
        }
        Ok(counts)
    }
    fn ovation(&mut self, id: &str, ballots: &[SeatVote], cmd: &Command) -> Result<(), Error> {
        self.admin(&cmd.actor)?;
        let c = self
            .calls
            .get(id)
            .ok_or(Error::Invalid("unknown Call"))?
            .clone();
        require(
            matches!(c.stage, CallStage::Testimony | CallStage::Ovation),
            "Ovation follows Testimony",
        )?;
        if let Some(winner) = &c.selected {
            require(
                !self
                    .seat_tally(&c.last_ballot, &c.candidates)
                    .is_ok_and(|votes| votes.get(winner).is_some_and(|n| *n >= 6)),
                "certified winner requires Corination, not another ballot",
            )?;
        }
        for p in &c.candidates {
            require(
                self.eligible(p, Office::PrimaDonna),
                "candidate no longer eligible",
            )?;
        }
        let counts = self.seat_tally(ballots, &c.candidates)?;
        for v in ballots {
            self.reserve(&v.receipt, cmd)?;
        }
        let c = self.calls.get_mut(id).expect("Call");
        c.stage = CallStage::Ovation;
        c.rounds = c
            .rounds
            .checked_add(1)
            .ok_or(Error::Invalid("ballot round overflow"))?;
        c.selected = counts
            .iter()
            .find(|(_, n)| **n >= 6)
            .map(|(p, _)| p.clone());
        c.participants
            .extend(ballots.iter().map(|v| v.receipt.signer.clone()));
        c.last_ballot = ballots.to_vec();
        Ok(())
    }
    fn corinate(
        &mut self,
        id: &str,
        tenure: &str,
        certification: &Attestation,
        appeals: &Attestation,
        cmd: &Command,
    ) -> Result<(), Error> {
        self.admin(&cmd.actor)?;
        stable(tenure)?;
        let c = self
            .calls
            .get(id)
            .ok_or(Error::Invalid("unknown Call"))?
            .clone();
        require(c.stage == CallStage::Ovation, "Corination follows Ovation")?;
        let winner = c
            .selected
            .as_ref()
            .ok_or(Error::Invalid("six affirmative seat votes required"))?;
        require(
            self.seat_tally(&c.last_ballot, &c.candidates)?
                .get(winner)
                .copied()
                .unwrap_or(0)
                >= 6,
            "six current certified seat votes required",
        )?;
        require(
            self.incumbent(Office::PrimaDonna).is_none(),
            "sovereign term must end before accession",
        )?;
        require(
            self.eligible(winner, Office::PrimaDonna),
            "winner no longer eligible",
        )?;
        require(
            !self.tenures.iter().any(|t| t.id == tenure)
                && !self.elections.values().any(|r| r.election.tenure == tenure),
            "tenure ID reused",
        )?;
        self.distinct_proofs(&[certification, appeals], cmd)?;
        self.court(&certification.signer, &c.participants)?;
        self.court(&appeals.signer, &c.participants)?;
        if let Some(h) = MuseHouse::ALL.into_iter().find(|h| {
            self.incumbent(Office::Persephone(*h))
                .is_some_and(|t| t.holder == *winner)
        }) {
            self.policy()?.corination_vacates_own_seat.as_ref().ok_or(
                Error::RequiresLegalParameter("winning Persephone's seat disposition"),
            )?;
            self.end(Office::Persephone(h), "certified-sovereign-accession", cmd)?;
        }
        self.tenures.push(Tenure {
            id: tenure.into(),
            office: Office::PrimaDonna,
            holder: winner.clone(),
            began: cmd.at,
            installed: cmd.at,
            expires: cmd.at.after_years(9)?,
            ended: None,
            ending: None,
            closure_event: None,
        });
        self.calls.get_mut(id).expect("Call").stage = CallStage::Corination;
        Ok(())
    }
    fn remove_sovereign(&mut self, r: &Removal, cmd: &Command) -> Result<(), Error> {
        self.admin(&cmd.actor)?;
        stable(&r.court_case)?;
        let t = self
            .incumbent(Office::PrimaDonna)
            .ok_or(Error::Invalid("sovereign vacancy"))?;
        require(t.id == r.tenure, "wrong removal tenure")?;
        let mut parties = BTreeSet::from([t.holder.clone()]);
        parties.extend(r.ballots.iter().map(|v| v.receipt.signer.clone()));
        self.removal_parties(&r.court_case, &r.tenure, &mut parties)?;
        self.distinct_proofs(
            &[
                &r.grounds,
                &r.notice_and_response,
                &r.appeals_resolved,
                &r.court_review,
            ],
            cmd,
        )?;
        for proof in [
            &r.grounds,
            &r.notice_and_response,
            &r.appeals_resolved,
            &r.court_review,
        ] {
            self.court(&proof.signer, &parties)?;
        }
        let counts = self.seat_tally(
            &r.ballots,
            &BTreeSet::from(["remove".into(), "retain".into()]),
        )?;
        for v in &r.ballots {
            self.reserve(&v.receipt, cmd)?;
        }
        self.removal_cases
            .insert(r.court_case.clone(), (r.tenure.clone(), parties));
        if counts["remove"] >= 6 {
            self.end(Office::PrimaDonna, "certified-removal", cmd)?;
        }
        Ok(())
    }
    fn envelope(&self, cmd: &Command) -> Result<InterfaceEventEnvelope, Error> {
        let aggregate = InterfaceAggregateId::new(RULE_SET).map_err(archive)?;
        let json = serde_json::to_string(cmd).map_err(archive)?;
        let checksum = json.bytes().fold(0xcbf29ce484222325_u64, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(0x100000001b3)
        });
        let case = match &cmd.action {
            Action::OpenCall { court_case, .. } | Action::ReviewVeto { court_case, .. } => {
                Some(court_case.as_str())
            }
            Action::Audition { id, .. }
            | Action::Testimony { id, .. }
            | Action::Ovation { id, .. }
            | Action::Corinate { id, .. } => self.calls.get(id).map(|c| c.court_case.as_str()),
            Action::CertifyRecall { recall } => Some(recall.court_case.as_str()),
            Action::RemoveSovereign { removal } => Some(removal.court_case.as_str()),
            _ => None,
        };
        let reference =
            |kind, id: &str| InterfaceIdentityRef::new(kind, id, SOURCE).map_err(archive);
        Ok(InterfaceEventEnvelope {
            event_id: InterfaceEventId::new(&cmd.id).map_err(archive)?,
            source_aggregate: aggregate.clone(),
            receiving_aggregate: aggregate,
            source_domain: InterfaceDomain::House(House::Glaushouse),
            receiving_domain: InterfaceDomain::House(House::Glaushouse),
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
                    .ok_or(Error::Invalid("not a Glaushouse governance archive"))?,
            )
            .map_err(archive)?;
            require(
                result.envelope(&cmd)? == *event,
                "event envelope and command disagree",
            )?;
            result.execute(cmd)?;
        }
        require(
            result.policy.is_some(),
            "missing policy and source snapshot",
        )?;
        Ok(result)
    }
}
