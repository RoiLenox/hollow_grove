//! Revolution C Flynt governance above the shared constitutional event archive.
//!
//! A trusted authority adapter authenticates all actors and external records.
//! Signer strings and FNV checksums are not authentication. Legacy Flynt catalogs
//! and succession records remain historical inputs, never live accession rules.
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

#[path = "equipment.rs"]
pub mod equipment;
#[path = "q.rs"]
pub mod q;

use super::synthesis::{
    BodilyForm, Convergence, ConvergenceInterpretation, ConvergenceVerification, ExpressionRecord,
    PairTerm, SynthesisState,
};

use crate::constitutional::{
    InterfaceAggregateId, InterfaceApplyOutcome, InterfaceDomain, InterfaceEventEnvelope,
    InterfaceEventId, InterfaceEventKind, InterfaceFailureState, InterfaceIdentityKind,
    InterfaceIdentityRef, SHARED_INTERFACE_ARCHIVE_VERSION, SharedInterfaceRuntime,
    decode_shared_interface_archive, encode_shared_interface_archive,
};
use crate::hollow_grove_contract::House;
use crate::institution::{
    IdentityId, InstitutionCatalog, InstitutionalBeingId, InstitutionalEntityId,
    InstitutionalRelationship, Office as CatalogOffice, OfficeHolder, OfficeId, OfficeScope,
    RelationshipId, RelationshipKind, Visibility,
};
pub use crate::world::glaushouse::governance::{Attestation, Date};
use crate::world::session::WorldSession;

pub const SOURCE: &str = "docs/FLYNT_REV_C_GOVERNANCE.md";
pub const RULE_SET: &str = "flynt.governance.revolution-c.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Form {
    Gargoyle,
    Medusa,
    Werewolf,
    Vampire,
    Merman,
    Mermaid,
}
impl Form {
    pub const ALL: [Self; 6] = [
        Self::Gargoyle,
        Self::Medusa,
        Self::Werewolf,
        Self::Vampire,
        Self::Merman,
        Self::Mermaid,
    ];
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Gargoyle | Self::Werewolf | Self::Merman)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Pair {
    GargoyleMedusa,
    WerewolfVampire,
    MermanMermaid,
}
impl Pair {
    pub const ALL: [Self; 3] = [
        Self::GargoyleMedusa,
        Self::WerewolfVampire,
        Self::MermanMermaid,
    ];
    pub const fn forms(self) -> [Form; 2] {
        match self {
            Self::GargoyleMedusa => [Form::Gargoyle, Form::Medusa],
            Self::WerewolfVampire => [Form::Werewolf, Form::Vampire],
            Self::MermanMermaid => [Form::Merman, Form::Mermaid],
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Office {
    Navigator,
    Chief,
    Mariner,
}
impl Office {
    pub const ALL: [Self; 3] = [Self::Navigator, Self::Chief, Self::Mariner];
    pub fn id(self) -> OfficeId {
        OfficeId::new(match self {
            Self::Navigator => "office.flynt.navigator",
            Self::Chief => "office.flynt.chief",
            Self::Mariner => "office.flynt.mariner",
        })
        .expect("stable commander office")
    }
    pub const fn title(self) -> &'static str {
        match self {
            Self::Navigator => "Navigator of the Tross",
            Self::Chief => "Tross Chief",
            Self::Mariner => "Mariner of the Tross",
        }
    }
    pub const fn pair(self) -> Pair {
        match self {
            Self::Navigator => Pair::GargoyleMedusa,
            Self::Chief => Pair::WerewolfVampire,
            Self::Mariner => Pair::MermanMermaid,
        }
    }
    pub const fn career(self) -> [CareerRank; 3] {
        match self {
            Self::Navigator => [
                CareerRank::Steerman,
                CareerRank::Helmsman,
                CareerRank::Navigator,
            ],
            Self::Chief => [CareerRank::Captain, CareerRank::Major, CareerRank::Chief],
            Self::Mariner => [
                CareerRank::Privateer,
                CareerRank::Corsair,
                CareerRank::Marquise,
            ],
        }
    }
    /// Lore identities of established characters, never inherited office rights.
    pub const fn historical_covers(self) -> [&'static str; 2] {
        match self {
            Self::Navigator => ["Bro White", "Johnny Apple"],
            Self::Chief => ["The Beautiful Beast", "Tommy Rose"],
            Self::Mariner => ["Cinderellaman", "Jimmy Pumps"],
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CareerRank {
    Steerman,
    Helmsman,
    Navigator,
    Captain,
    Major,
    Chief,
    Privateer,
    Corsair,
    Marquise,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    MysteryMen,
    WeFairyMen,
    Gallows,
}
impl Face {
    pub const ALL: [Self; 3] = [Self::MysteryMen, Self::WeFairyMen, Self::Gallows];
    pub const fn purpose(self) -> &'static str {
        match self {
            Self::MysteryMen => "Lawful formal intelligence and security",
            Self::WeFairyMen => "Roaming undercover work",
            Self::Gallows => "Underground infiltration; not the Syndicate",
        }
    }
}

/// Explicitly scoped external mastery evidence; neither a title nor a cover.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FormMastery {
    pub person: String,
    pub form: Form,
    pub certification: Attestation,
}
/// Import of an independently certified Ambisynth credential. Each of the two
/// Form records and the pair-specific switching demonstration remains distinct.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AmbisynthCredential {
    pub person: String,
    pub pair: Pair,
    pub forms: [FormMastery; 2],
    pub switching: Attestation,
    pub certification: Attestation,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LegalDays {
    pub value: u64,
    pub authorization: Attestation,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub authorization: Attestation,
    pub administrator: String,
    pub credential_reviewers: BTreeSet<String>,
    pub court_reviewers: BTreeSet<String>,
    /// Attest actual execution outcomes only; never clinical privileges.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub execution_reviewers: BTreeSet<String>,
    pub first_elections: BTreeMap<Office, Date>,
    pub days_per_year: Option<LegalDays>,
    pub veto_days: Option<LegalDays>,
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
    pub special: bool,
    pub certification: Attestation,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Action {
    Initialize {
        policy: Policy,
        institutional_snapshot: String,
    },
    RecognizeCareer {
        person: String,
        rank: CareerRank,
        certification: Attestation,
    },
    QualifyCommander {
        office: Office,
        credential: AmbisynthCredential,
        competence: Attestation,
    },
    RecordElection {
        election: Election,
    },
    VetoElection {
        election: String,
        grounds: Attestation,
    },
    ResolveVeto {
        election: String,
        upheld: bool,
        review: Attestation,
    },
    SeatCommander {
        election: String,
        tenure: String,
        appeals_resolved: Attestation,
    },
    ResignCommander {
        office: Office,
        tenure: String,
    },
    RequestRecall {
        office: Office,
    },
    ObserveDate,
    RecordFormMastery {
        mastery: FormMastery,
    },
    CertifyAmbisynth {
        credential: AmbisynthCredential,
        terminology: PairTerm,
    },
    RecordExpression {
        record: ExpressionRecord,
    },
    AdoptConvergenceInterpretation {
        interpretation: ConvergenceInterpretation,
        authorization: Attestation,
        legacy_conflict_review: Attestation,
    },
    CertifyConvergence {
        verification: ConvergenceVerification,
    },
    Q {
        #[serde(rename = "request")]
        action: q::Action,
    },
    TripleCrown {
        request: equipment::Action,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Command {
    pub id: String,
    pub expected_revision: u64,
    pub date: Date,
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
    pub expires: Date,
    pub ended: Option<Date>,
    pub closure_event: Option<String>,
    pub ending: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingElection {
    pub election: Election,
    pub recorded: Date,
    pub deadline: Date,
    pub veto: Option<(String, Option<bool>)>,
    pub tenure: Option<String>,
    pub participants: BTreeSet<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Governance {
    journal: SharedInterfaceRuntime,
    policy: Option<Policy>,
    source_snapshot: String,
    date: Date,
    tross: Option<String>,
    careers: BTreeSet<(String, CareerRank)>,
    qualifications: BTreeMap<(String, Office), AmbisynthCredential>,
    tenures: Vec<Tenure>,
    elections: BTreeMap<String, PendingElection>,
    credential_claims: BTreeMap<String, (EvidenceClaim, String)>,
    synthesis: SynthesisState,
    q: q::State,
    crowns: BTreeMap<String, equipment::Custody>,
    ballot_receipts: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EvidenceClaim {
    Career(String, CareerRank),
    Form(String, Form),
    Switching(String, Pair),
    Ambisynth(String, Pair),
    Command(String, Office),
    Execution(String, String, BodilyForm),
    Consent(String, String, BodilyForm),
    ConvergenceExecution(String, Convergence, String),
    ConvergenceCompetence(String, Convergence, String),
    Q(String, String),
    TripleCrown(String, String),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnresolvedLaw {
    Calendar,
    FirstElectionCohort,
    VetoWindow,
    RecallThresholds,
    SpecialElectionTerm,
    RunoffQualificationTie,
    ExistingCommanderContinuity,
    SourceSovereignContinuity,
    ConvergenceInterpretation,
    LegacyConvergenceConflict,
    QRecordContract,
    QSuccessionReconciliation,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Invalid(&'static str),
    UnresolvedLaw(UnresolvedLaw),
    Archive(String),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Flynt governance rejected: {self:?}")
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
        .map_err(|_| Error::Invalid("invalid stable identity"))
}
fn archive(error: impl fmt::Display) -> Error {
    Error::Archive(error.to_string())
}

impl Governance {
    pub fn new(world: &WorldSession, policy: Policy, date: Date) -> Result<Self, Error> {
        let institutional_snapshot = world.persisted_state_output();
        // The legacy codec starts from canonical holders. Do not initialize a
        // new adapter if that would silently restore or replace its source Tross.
        // Historical Initialize events still replay their original stored bytes.
        let restored =
            WorldSession::from_persisted_output(&institutional_snapshot).map_err(archive)?;
        let active_tross = |session: &WorldSession| {
            let mut holders = session
                .institutional()
                .catalog
                .office_holders
                .iter()
                .filter(|h| h.active && h.office == super::tross_office_id())
                .map(|h| h.being.as_str().to_owned())
                .collect::<Vec<_>>();
            holders.sort();
            holders
        };
        if active_tross(world) != active_tross(&restored) {
            return Err(Error::UnresolvedLaw(
                UnresolvedLaw::SourceSovereignContinuity,
            ));
        }
        let mut evidence = BTreeSet::from([policy.authorization.record.clone()]);
        for p in [&policy.days_per_year, &policy.veto_days]
            .into_iter()
            .flatten()
        {
            evidence.insert(p.authorization.record.clone());
        }
        let mut g = Self::default();
        g.execute(Command {
            id: "event.flynt.governance.initialize".into(),
            expected_revision: 0,
            date,
            actor: policy.authorization.signer.clone(),
            evidence,
            action: Action::Initialize {
                policy,
                institutional_snapshot,
            },
        })?;
        Ok(g)
    }
    pub fn revision(&self) -> u64 {
        self.journal.events().len() as u64
    }
    pub fn date(&self) -> Date {
        self.date
    }
    pub fn tross(&self) -> Option<&str> {
        self.tross.as_deref()
    }
    pub fn source_snapshot(&self) -> &str {
        &self.source_snapshot
    }
    pub fn tenures(&self) -> &[Tenure] {
        &self.tenures
    }
    pub fn election(&self, id: &str) -> Option<&PendingElection> {
        self.elections.get(id)
    }
    pub fn has_career(&self, person: &str, rank: CareerRank) -> bool {
        self.careers.contains(&(person.into(), rank))
    }
    pub fn qualified(&self, person: &str, office: Office) -> bool {
        self.qualifications.contains_key(&(person.into(), office))
    }
    pub fn incumbent(&self, office: Office) -> Option<&Tenure> {
        self.tenures
            .iter()
            .rev()
            .find(|t| t.office == office && t.ended.is_none())
    }
    fn policy(&self) -> Result<&Policy, Error> {
        self.policy
            .as_ref()
            .ok_or(Error::Invalid("not initialized"))
    }
    fn is_officer(&self, person: &str) -> bool {
        self.tross() == Some(person)
            || Office::ALL
                .iter()
                .any(|o| self.incumbent(*o).is_some_and(|t| t.holder == person))
    }
    fn admin(&self, actor: &str) -> Result<(), Error> {
        require(
            self.policy()?.administrator == actor && !self.is_officer(actor),
            "independent election administrator required",
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
    fn credential(&self, proof: &Attestation, person: &str, cmd: &Command) -> Result<(), Error> {
        self.attest(proof, cmd)?;
        require(
            self.policy()?.credential_reviewers.contains(&proof.signer)
                && proof.signer != person
                && !self.is_officer(&proof.signer)
                && proof.signer != self.policy()?.administrator,
            "independent credential review required",
        )
    }
    fn court(
        &self,
        proof: &Attestation,
        parties: &BTreeSet<String>,
        cmd: &Command,
    ) -> Result<(), Error> {
        self.attest(proof, cmd)?;
        require(
            self.policy()?.court_reviewers.contains(&proof.signer)
                && !parties.contains(&proof.signer)
                && !self.is_officer(&proof.signer)
                && proof.signer != self.policy()?.administrator,
            "independent Court review required",
        )
    }
    fn pair_credential(&self, c: &AmbisynthCredential, cmd: &Command) -> Result<(), Error> {
        stable(&c.person)?;
        let expected = c.pair.forms().into_iter().collect::<BTreeSet<_>>();
        require(
            c.forms.iter().map(|f| f.form).collect::<BTreeSet<_>>() == expected,
            "both specific sibling Forms required",
        )?;
        let proofs = [
            &c.forms[0].certification,
            &c.forms[1].certification,
            &c.switching,
            &c.certification,
        ];
        require(
            proofs
                .iter()
                .map(|p| &p.record)
                .collect::<BTreeSet<_>>()
                .len()
                == 4,
            "distinct Form, switching and certification evidence required",
        )?;
        for f in &c.forms {
            require(f.person == c.person, "wrong mastery subject")?;
        }
        for proof in proofs {
            self.credential(proof, &c.person, cmd)?;
        }
        Ok(())
    }
    fn calendar(&self) -> Result<u64, Error> {
        self.policy()?
            .days_per_year
            .as_ref()
            .map(|p| p.value)
            .ok_or(Error::UnresolvedLaw(UnresolvedLaw::Calendar))
    }
    fn add_days(&self, date: Date, days: u64) -> Result<Date, Error> {
        let calendar = self.calendar()?;
        let total = date
            .day
            .checked_add(days)
            .ok_or(Error::Invalid("date overflow"))?;
        Ok(Date {
            year: date
                .year
                .checked_add(total / calendar)
                .ok_or(Error::Invalid("date overflow"))?,
            day: total % calendar,
        })
    }
    fn claim(&mut self, proof: &Attestation, scope: EvidenceClaim) -> Result<(), Error> {
        if let Some((existing, signer)) = self.credential_claims.get(&proof.record) {
            require(
                *existing == scope && *signer == proof.signer,
                "credential evidence reused for a different fact or signer",
            )?;
        } else {
            self.credential_claims
                .insert(proof.record.clone(), (scope, proof.signer.clone()));
        }
        Ok(())
    }
    fn claim_pair(&mut self, c: &AmbisynthCredential) -> Result<(), Error> {
        for f in &c.forms {
            self.claim(
                &f.certification,
                EvidenceClaim::Form(f.person.clone(), f.form),
            )?;
        }
        self.claim(
            &c.switching,
            EvidenceClaim::Switching(c.person.clone(), c.pair),
        )?;
        self.claim(
            &c.certification,
            EvidenceClaim::Ambisynth(c.person.clone(), c.pair),
        )
    }
    pub fn has_form(&self, person: &str, form: Form) -> bool {
        self.synthesis.forms.contains_key(&(person.into(), form))
    }
    pub fn ambisynth(&self, person: &str, pair: Pair) -> Option<&AmbisynthCredential> {
        self.synthesis
            .pairs
            .get(&(person.into(), pair))
            .map(|(c, _)| c)
    }
    pub fn expressed_form(&self, person: &str) -> Option<BodilyForm> {
        self.synthesis.expressed.get(person).copied()
    }
    pub fn has_bodily_mastery(&self, person: &str, form: BodilyForm) -> bool {
        match form {
            BodilyForm::Sibling(f) => self.has_form(person, f),
            BodilyForm::Chimera => self.convergence(person, Convergence::Chimera).is_some(),
            BodilyForm::Manticorp => self.convergence(person, Convergence::Manticorp).is_some(),
        }
    }
    pub fn convergence(
        &self,
        person: &str,
        outcome: Convergence,
    ) -> Option<&ConvergenceVerification> {
        self.synthesis.convergences.get(&(person.into(), outcome))
    }
    /// Obligation only: no queue entry, contest, office transfer or new powers.
    pub fn must_enter_q(&self, person: &str) -> bool {
        self.convergence(person, Convergence::Manticorp).is_some() && self.tross() != Some(person)
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
        require(cmd.date >= self.date, "time moved backwards")?;
        let mut next = self.clone();
        next.date = cmd.date;
        for t in &mut next.tenures {
            if t.ended.is_none() && t.expires <= cmd.date {
                t.ended = Some(t.expires);
                t.ending = Some("term-expired".into());
                t.closure_event = Some(cmd.id.clone());
            }
        }
        next.reduce(&cmd)?;
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
                cmd.actor == policy.authorization.signer && cmd.actor != policy.administrator,
                "independent policy mandate required",
            )?;
            for actor in policy
                .credential_reviewers
                .iter()
                .chain(&policy.court_reviewers)
                .chain(&policy.execution_reviewers)
            {
                stable(actor)?;
            }
            for parameter in [&policy.days_per_year, &policy.veto_days]
                .into_iter()
                .flatten()
            {
                self.attest(&parameter.authorization, cmd)?;
                require(
                    parameter.value > 0
                        && parameter.authorization.signer == cmd.actor
                        && parameter.authorization.record != policy.authorization.record,
                    "explicit independently authorized legal parameter required",
                )?;
            }
            if policy.first_elections.len() > 1 {
                let dates = policy.first_elections.values().collect::<Vec<_>>();
                require(
                    dates
                        .iter()
                        .all(|d| d.day == dates[0].day && d.year % 2 == dates[0].year % 2)
                        && dates
                            .iter()
                            .map(|d| d.year % 6)
                            .collect::<BTreeSet<_>>()
                            .len()
                            == dates.len(),
                    "commander cohorts must be staggered two years",
                )?;
            }
            let world =
                WorldSession::from_persisted_output(institutional_snapshot).map_err(archive)?;
            let holders = &world.institutional().catalog.office_holders;
            if holders
                .iter()
                .any(|h| h.active && Office::ALL.iter().any(|o| o.id() == h.office))
            {
                return Err(Error::UnresolvedLaw(
                    UnresolvedLaw::ExistingCommanderContinuity,
                ));
            }
            let sovereigns = holders
                .iter()
                .filter(|h| h.active && h.office == super::tross_office_id())
                .collect::<Vec<_>>();
            require(sovereigns.len() <= 1, "multiple Tross incumbents")?;
            self.tross = sovereigns.first().map(|h| h.being.as_str().to_owned());
            self.source_snapshot = institutional_snapshot.clone();
            self.policy = Some(policy.clone());
        }
        let p = self.policy()?;
        if let Some(calendar) = &p.days_per_year {
            require(
                cmd.date.day < calendar.value
                    && p.first_elections.values().all(|d| d.day < calendar.value),
                "day outside legal calendar",
            )?;
        } else if cmd.date.day != 0 {
            return Err(Error::UnresolvedLaw(UnresolvedLaw::Calendar));
        }
        match &cmd.action {
            Action::Q { action } => self.reduce_q(action, cmd)?,
            Action::TripleCrown { request } => self.reduce_crown(request, cmd)?,
            Action::Initialize { .. } => {}
            Action::RecognizeCareer {
                person,
                rank,
                certification,
            } => {
                stable(person)?;
                self.credential(certification, person, cmd)?;
                require(
                    cmd.actor == certification.signer,
                    "credential actor mismatch",
                )?;
                require(!self.has_career(person, *rank), "career already recognized")?;
                self.claim(certification, EvidenceClaim::Career(person.clone(), *rank))?;
                self.careers.insert((person.clone(), *rank));
            }
            Action::QualifyCommander {
                office,
                credential,
                competence,
            } => {
                self.pair_credential(credential, cmd)?;
                require(credential.pair == office.pair(), "wrong command pair")?;
                self.credential(competence, &credential.person, cmd)?;
                require(
                    cmd.actor == credential.certification.signer,
                    "qualification actor mismatch",
                )?;
                require(
                    !self.qualified(&credential.person, *office),
                    "command qualification already recognized",
                )?;
                self.claim_pair(credential)?;
                self.claim(
                    competence,
                    EvidenceClaim::Command(credential.person.clone(), *office),
                )?;
                self.qualifications
                    .insert((credential.person.clone(), *office), credential.clone());
            }
            Action::RecordElection { election } => self.record_election(election, cmd)?,
            Action::VetoElection { election, grounds } => {
                self.attest(grounds, cmd)?;
                require(
                    self.tross() == Some(cmd.actor.as_str()) && grounds.signer == cmd.actor,
                    "only incumbent Tross may veto",
                )?;
                let e = self
                    .elections
                    .get_mut(election)
                    .ok_or(Error::Invalid("unknown election"))?;
                require(
                    e.tenure.is_none() && e.veto.is_none() && cmd.date < e.deadline,
                    "veto closed or already exercised",
                )?;
                e.participants.insert(cmd.actor.clone());
                e.veto = Some((grounds.record.clone(), None));
            }
            Action::ResolveVeto {
                election,
                upheld,
                review,
            } => {
                let e = self
                    .elections
                    .get(election)
                    .ok_or(Error::Invalid("unknown election"))?;
                self.court(review, &e.participants, cmd)?;
                require(
                    cmd.actor == review.signer
                        && e.veto.as_ref().is_some_and(|v| v.1.is_none())
                        && e.tenure.is_none(),
                    "unresolved veto and independent decision required",
                )?;
                self.elections
                    .get_mut(election)
                    .expect("election")
                    .veto
                    .as_mut()
                    .expect("veto")
                    .1 = Some(*upheld);
            }
            Action::SeatCommander {
                election,
                tenure,
                appeals_resolved,
            } => {
                self.admin(&cmd.actor)?;
                stable(tenure)?;
                let e = self
                    .elections
                    .get(election)
                    .ok_or(Error::Invalid("unknown election"))?;
                self.court(appeals_resolved, &e.participants, cmd)?;
                require(
                    e.tenure.is_none()
                        && cmd.date >= e.deadline
                        && e.veto.as_ref().is_none_or(|v| v.1 == Some(false)),
                    "election waiting, veto unresolved/upheld, or already seated",
                )?;
                let expires = Date {
                    year: e
                        .recorded
                        .year
                        .checked_add(6)
                        .ok_or(Error::Invalid("term overflow"))?,
                    day: e.recorded.day,
                };
                require(cmd.date < expires, "election term expired")?;
                require(
                    self.incumbent(e.election.office).is_none()
                        && !self.is_officer(&e.election.winner),
                    "occupied or incompatible office",
                )?;
                require(
                    !self.tenures.iter().any(|t| t.id == *tenure),
                    "tenure ID reused",
                )?;
                let t = Tenure {
                    id: tenure.clone(),
                    office: e.election.office,
                    holder: e.election.winner.clone(),
                    began: cmd.date,
                    expires,
                    ended: None,
                    closure_event: None,
                    ending: None,
                };
                self.tenures.push(t);
                self.elections.get_mut(election).expect("election").tenure = Some(tenure.clone());
            }
            Action::ResignCommander { office, tenure } => {
                let t = self
                    .incumbent(*office)
                    .ok_or(Error::Invalid("office vacant"))?;
                require(
                    t.id == *tenure && t.holder == cmd.actor,
                    "only incumbent may resign own tenure",
                )?;
                let t = self
                    .tenures
                    .iter_mut()
                    .find(|t| t.id == *tenure)
                    .expect("tenure");
                t.ended = Some(cmd.date);
                t.ending = Some("resignation".into());
                t.closure_event = Some(cmd.id.clone());
            }
            Action::RequestRecall { .. } => {
                return Err(Error::UnresolvedLaw(UnresolvedLaw::RecallThresholds));
            }
            Action::ObserveDate => self.admin(&cmd.actor)?,
            Action::RecordFormMastery { mastery } => {
                stable(&mastery.person)?;
                self.credential(&mastery.certification, &mastery.person, cmd)?;
                require(
                    cmd.actor == mastery.certification.signer,
                    "mastery actor mismatch",
                )?;
                require(
                    !self.has_form(&mastery.person, mastery.form),
                    "Form mastery already recorded",
                )?;
                self.claim(
                    &mastery.certification,
                    EvidenceClaim::Form(mastery.person.clone(), mastery.form),
                )?;
                self.synthesis
                    .forms
                    .insert((mastery.person.clone(), mastery.form), mastery.clone());
            }
            Action::CertifyAmbisynth {
                credential: c,
                terminology,
            } => {
                self.pair_credential(c, cmd)?;
                require(
                    cmd.actor == c.certification.signer,
                    "Ambisynth certifier mismatch",
                )?;
                require(
                    self.ambisynth(&c.person, c.pair).is_none(),
                    "pair already certified",
                )?;
                for f in &c.forms {
                    require(
                        self.synthesis.forms.get(&(f.person.clone(), f.form)) == Some(f),
                        "both independently recorded Form masteries required",
                    )?;
                }
                self.claim_pair(c)?;
                self.synthesis
                    .pairs
                    .insert((c.person.clone(), c.pair), (c.clone(), *terminology));
            }
            Action::RecordExpression { record: r } => {
                self.attest(&r.execution, cmd)?;
                self.attest(&r.consent, cmd)?;
                require(
                    self.policy()?.execution_reviewers.contains(&cmd.actor)
                        && cmd.actor == r.execution.signer
                        && cmd.actor != r.person
                        && !self.is_officer(&cmd.actor),
                    "independent authenticated execution receipt required",
                )?;
                require(
                    r.consent.signer == r.person && r.consent.record != r.execution.record,
                    "separate personal consent required",
                )?;
                require(
                    self.has_bodily_mastery(&r.person, r.form),
                    "unmastered bodily Form",
                )?;
                require(
                    self.expressed_form(&r.person) == r.previous && r.previous != Some(r.form),
                    "stale or duplicate bodily expression",
                )?;
                if let (Some(BodilyForm::Sibling(from)), BodilyForm::Sibling(to)) =
                    (r.previous, r.form)
                    && let Some(pair) = Pair::ALL
                        .into_iter()
                        .find(|p| p.forms().contains(&from) && p.forms().contains(&to))
                {
                    require(
                        self.ambisynth(&r.person, pair).is_some(),
                        "pair-specific switching certification required",
                    )?;
                }
                self.claim(
                    &r.execution,
                    EvidenceClaim::Execution(cmd.id.clone(), r.person.clone(), r.form),
                )?;
                self.claim(
                    &r.consent,
                    EvidenceClaim::Consent(cmd.id.clone(), r.person.clone(), r.form),
                )?;
                self.synthesis.expressed.insert(r.person.clone(), r.form);
            }
            Action::AdoptConvergenceInterpretation {
                interpretation,
                authorization,
                legacy_conflict_review,
            } => {
                if *interpretation != ConvergenceInterpretation::RevolutionC99B {
                    return Err(Error::UnresolvedLaw(
                        UnresolvedLaw::LegacyConvergenceConflict,
                    ));
                }
                require(
                    self.synthesis.interpretation.is_none(),
                    "convergence interpretation already adopted",
                )?;
                self.attest(authorization, cmd)?;
                require(
                    cmd.actor == self.policy()?.authorization.signer
                        && authorization.signer == cmd.actor
                        && authorization.record != self.policy()?.authorization.record,
                    "explicit new convergence mandate required",
                )?;
                self.court(
                    legacy_conflict_review,
                    &BTreeSet::from([cmd.actor.clone()]),
                    cmd,
                )?;
                require(
                    authorization.record != legacy_conflict_review.record,
                    "distinct mandate and legacy conflict review required",
                )?;
                self.synthesis.interpretation = Some(*interpretation);
            }
            Action::CertifyConvergence { verification: v } => {
                if v.interpretation != ConvergenceInterpretation::RevolutionC99B {
                    return Err(Error::UnresolvedLaw(
                        UnresolvedLaw::LegacyConvergenceConflict,
                    ));
                }
                if self.synthesis.interpretation != Some(v.interpretation) {
                    return Err(Error::UnresolvedLaw(
                        UnresolvedLaw::ConvergenceInterpretation,
                    ));
                }
                stable(&v.person)?;
                stable(&v.recipe_revision)?;
                require(
                    v.recipe_id == v.outcome.recipe_id(),
                    "wrong convergence Recipe",
                )?;
                self.attest(&v.execution, cmd)?;
                self.credential(&v.competence, &v.person, cmd)?;
                require(
                    cmd.actor == v.competence.signer
                        && self
                            .policy()?
                            .execution_reviewers
                            .contains(&v.execution.signer)
                        && v.execution.signer != v.person
                        && !self.is_officer(&v.execution.signer)
                        && v.execution.signer != v.competence.signer
                        && v.execution.record != v.competence.record,
                    "independent verified Recipe execution and competence required",
                )?;
                require(
                    self.convergence(&v.person, v.outcome).is_none(),
                    "convergence already certified",
                )?;
                match v.outcome {
                    Convergence::Chimera => require(
                        [Form::Gargoyle, Form::Werewolf, Form::Merman]
                            .iter()
                            .all(|f| self.has_form(&v.person, *f)),
                        "Trifecta requires all three Current Forms",
                    )?,
                    Convergence::Manticorp => {
                        require(
                            Form::ALL.iter().all(|f| self.has_form(&v.person, *f)),
                            "Manticorp requires all six physical Forms",
                        )?;
                        require(
                            Pair::ALL
                                .iter()
                                .all(|p| self.ambisynth(&v.person, *p).is_some()),
                            "Manticorp requires all three independently certified Ambisynth pairs",
                        )?;
                    }
                }
                self.claim(
                    &v.execution,
                    EvidenceClaim::ConvergenceExecution(
                        v.person.clone(),
                        v.outcome,
                        v.recipe_revision.clone(),
                    ),
                )?;
                self.claim(
                    &v.competence,
                    EvidenceClaim::ConvergenceCompetence(
                        v.person.clone(),
                        v.outcome,
                        v.recipe_revision.clone(),
                    ),
                )?;
                self.synthesis
                    .convergences
                    .insert((v.person.clone(), v.outcome), v.clone());
            }
        }
        Ok(())
    }
    fn tally(
        &mut self,
        votes: &[Vote],
        roll: &ElectoralRoll,
        candidates: &BTreeSet<String>,
        cmd: &Command,
    ) -> Result<BTreeMap<String, u64>, Error> {
        let mut voters = BTreeSet::new();
        let mut counts = candidates
            .iter()
            .map(|c| (c.clone(), 0))
            .collect::<BTreeMap<_, _>>();
        for v in votes {
            self.attest(&v.receipt, cmd)?;
            require(
                v.receipt.signer == v.voter
                    && roll.eligible.contains(&v.voter)
                    && candidates.contains(&v.candidate)
                    && voters.insert(&v.voter)
                    && self.ballot_receipts.insert(v.receipt.record.clone()),
                "invalid, ineligible or repeated ballot",
            )?;
            *counts.get_mut(&v.candidate).expect("candidate") += 1;
        }
        Ok(counts)
    }
    fn record_election(&mut self, e: &Election, cmd: &Command) -> Result<(), Error> {
        self.admin(&cmd.actor)?;
        stable(&e.id)?;
        if e.special {
            return Err(Error::UnresolvedLaw(UnresolvedLaw::SpecialElectionTerm));
        }
        let first = *self
            .policy()?
            .first_elections
            .get(&e.office)
            .ok_or(Error::UnresolvedLaw(UnresolvedLaw::FirstElectionCohort))?;
        require(
            cmd.date >= first
                && (cmd.date.year - first.year).is_multiple_of(6)
                && cmd.date.day == first.day,
            "outside six-year cohort",
        )?;
        let window = self
            .policy()?
            .veto_days
            .as_ref()
            .ok_or(Error::UnresolvedLaw(UnresolvedLaw::VetoWindow))?
            .value;
        let deadline = self.add_days(cmd.date, window)?;
        let end = Date {
            year: cmd
                .date
                .year
                .checked_add(6)
                .ok_or(Error::Invalid("term overflow"))?,
            day: cmd.date.day,
        };
        require(deadline < end, "veto window consumes entire elected term")?;
        require(
            !self.elections.contains_key(&e.id) && self.incumbent(e.office).is_none(),
            "election reused or tenure occupied",
        )?;
        require(
            !self.elections.values().any(|p| {
                p.election.office == e.office
                    && p.tenure.is_none()
                    && p.veto.as_ref().is_none_or(|v| v.1 != Some(true))
                    && p.recorded.year + 6 > cmd.date.year
            }),
            "unresolved election already exists",
        )?;
        self.attest(&e.certification, cmd)?;
        self.attest(&e.roll.certification, cmd)?;
        require(
            e.certification.signer == cmd.actor
                && e.roll.certification.signer == cmd.actor
                && e.certification.record != e.roll.certification.record
                && e.roll.office == e.office
                && !e.roll.eligible.is_empty(),
            "certified professional peer roll required",
        )?;
        require(
            !e.candidates.is_empty() && !e.candidates.contains(&cmd.actor),
            "independent election candidates required",
        )?;
        for voter in &e.roll.eligible {
            stable(voter)?;
        }
        for person in &e.candidates {
            require(
                self.qualified(person, e.office) && !self.is_officer(person),
                "pair-qualified candidate without incompatible office required",
            )?;
        }
        require(
            (1..=2).contains(&e.rounds.len()),
            "one ballot and optional runoff required",
        )?;
        let first = self.tally(&e.rounds[0], &e.roll, &e.candidates, cmd)?;
        if let Some(winner) = majority(&first) {
            require(
                e.rounds.len() == 1 && winner == e.winner,
                "incorrect majority or unnecessary runoff",
            )?;
        } else {
            require(
                e.rounds.len() == 2 && !e.rounds[0].is_empty(),
                "majority or runoff required",
            )?;
            let mut ranked = first.iter().collect::<Vec<_>>();
            ranked.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
            require(ranked.len() >= 2, "runoff pair required")?;
            if ranked.len() > 2 && ranked[1].1 == ranked[2].1 {
                return Err(Error::UnresolvedLaw(UnresolvedLaw::RunoffQualificationTie));
            }
            let finalists = BTreeSet::from([ranked[0].0.clone(), ranked[1].0.clone()]);
            require(
                majority(&self.tally(&e.rounds[1], &e.roll, &finalists, cmd)?)
                    == Some(e.winner.as_str()),
                "runoff majority required",
            )?;
        }
        let mut participants = e.candidates.clone();
        participants.insert(cmd.actor.clone());
        participants.extend(e.rounds.iter().flatten().map(|v| v.voter.clone()));
        self.elections.insert(
            e.id.clone(),
            PendingElection {
                election: e.clone(),
                recorded: cmd.date,
                deadline,
                veto: None,
                tenure: None,
                participants,
            },
        );
        Ok(())
    }
    /// A bounded active command projection, not a mutation/merger of the legacy
    /// catalog. Source identities, private grants and other Houses stay intact.
    pub fn command_catalog(&self) -> Result<InstitutionCatalog, Error> {
        self.policy()?;
        let mut catalog = InstitutionCatalog::default();
        catalog.offices.push(CatalogOffice {
            id: super::tross_office_id(),
            name: "The Tross".into(),
            scope: OfficeScope::House,
            institution: None,
            house: Some(House::Flynt),
            singular: true,
            authority: vec!["FlyntSovereignty".into()],
        });
        if let Some(person) = self.tross() {
            catalog.office_holders.push(OfficeHolder {
                office: super::tross_office_id(),
                being: InstitutionalBeingId::new(person).map_err(archive)?,
                active: true,
            });
        }
        for office in Office::ALL {
            catalog.offices.push(CatalogOffice {
                id: office.id(),
                name: office.title().into(),
                scope: OfficeScope::House,
                institution: None,
                house: Some(House::Flynt),
                singular: true,
                authority: vec!["ScopedSeniorCommand".into()],
            });
            catalog.relationships.push(InstitutionalRelationship {
                id: RelationshipId::new(format!("relationship.{}.answers-to-tross", office.id()))
                    .map_err(archive)?,
                source: InstitutionalEntityId::Office(office.id()),
                kind: RelationshipKind::AnswersTo,
                target: InstitutionalEntityId::Office(super::tross_office_id()),
                authority: None,
                visibility: Visibility::Public,
            });
        }
        for t in &self.tenures {
            catalog.office_holders.push(OfficeHolder {
                office: t.office.id(),
                being: InstitutionalBeingId::new(&t.holder).map_err(archive)?,
                active: t.ended.is_none(),
            });
        }
        catalog
            .validate()
            .map_err(|error| Error::Archive(format!("{error:?}")))?;
        Ok(catalog)
    }
    fn envelope(&self, cmd: &Command) -> Result<InterfaceEventEnvelope, Error> {
        let aggregate = InterfaceAggregateId::new(RULE_SET).map_err(archive)?;
        let json = serde_json::to_string(cmd).map_err(archive)?;
        let checksum = json.bytes().fold(0xcbf29ce484222325_u64, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(0x100000001b3)
        });
        let reference =
            |kind, id: &str| InterfaceIdentityRef::new(kind, id, SOURCE).map_err(archive);
        Ok(InterfaceEventEnvelope {
            event_id: InterfaceEventId::new(&cmd.id).map_err(archive)?,
            source_aggregate: aggregate.clone(),
            receiving_aggregate: aggregate,
            source_domain: InterfaceDomain::House(House::Flynt),
            receiving_domain: InterfaceDomain::House(House::Flynt),
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
            related_case: None,
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
        let mut g = Self::default();
        for event in journal.events() {
            let cmd: Command = serde_json::from_str(
                event
                    .payload
                    .get("command")
                    .ok_or(Error::Invalid("not a Flynt governance archive"))?,
            )
            .map_err(archive)?;
            require(
                g.envelope(&cmd)? == *event,
                "event envelope and command disagree",
            )?;
            g.execute(cmd)?;
        }
        g.policy()?;
        Ok(g)
    }
}
fn majority(counts: &BTreeMap<String, u64>) -> Option<&str> {
    let total = counts.values().map(|n| u128::from(*n)).sum::<u128>();
    counts
        .iter()
        .find(|(_, n)| u128::from(**n) * 2 > total)
        .map(|(name, _)| name.as_str())
}
