//! Bounded The Q preparation and contest evidence, never an accession reducer.
//!
//! All attestations require an authenticating caller. This records the newer
//! design under an explicit mandate without amending the four-House succession
//! law. A completed contest cannot change office, Form, tenure or private grants.
use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Contract {
    pub authorization: Attestation,
    pub legacy_review: Attestation,
    pub assessors: BTreeSet<String>,
    pub judges: BTreeSet<String>,
    pub witnesses: BTreeSet<String>,
}

/// The external reviewer checks current standing, manifestability and conflicts.
/// Form mastery alone is not evidence of current eligibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Eligibility {
    pub person: String,
    pub valid_until: Date,
    pub certification: Attestation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assessment {
    pub affirmative: bool,
    pub finding: Attestation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Admission {
    pub id: String,
    pub eligibility: Eligibility,
    pub assessments: [Assessment; 3],
    pub consent: Attestation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Round {
    CloseOffPoseOff,
    SocialEngineering,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Judgment {
    pub winner: String,
    pub finding: Attestation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EndReason {
    Concession,
    InabilityToContinue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "step", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Action {
    Configure {
        contract: Contract,
    },
    Admit {
        admission: Admission,
    },
    Withdraw {
        entry: String,
    },
    Expire {
        entry: String,
    },
    Schedule {
        contest: String,
        entries: Vec<String>,
        at: Date,
        /// Current evidence for both participants, including the incumbent.
        readiness: [Eligibility; 2],
    },
    RecordRound {
        contest: String,
        round: Round,
        panel: [Judgment; 3],
    },
    RecordSuddenDeath {
        contest: String,
        loser: String,
        reason: EndReason,
        /// Concession is signed by the loser; inability by an execution reviewer.
        outcome: Attestation,
        witness: Attestation,
    },
    /// Intentionally blocked; no string mandate bypasses the enacted conflict.
    FinalizeSuccession {
        contest: String,
    },
    /// Requires a lawful vacancy event; the legacy world codec cannot supply it.
    OpenFinalVacancy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryStatus {
    Waiting,
    Withdrawn,
    Expired,
    InContest(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub admission: Admission,
    pub admitted: Date,
    pub status: EntryStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contest {
    pub id: String,
    pub participants: [String; 2],
    pub scheduled: Date,
    pub readiness: [Eligibility; 2],
    pub rounds: Vec<(Round, [Judgment; 3], String)>,
    pub sudden_death: Option<(String, EndReason, Attestation, Attestation)>,
    /// Verified contest result only; sovereign succession stays blocked.
    pub result: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct State {
    contract: Option<Contract>,
    entries: Vec<Entry>,
    front_since: Option<Date>,
    contest: Option<Contest>,
}

impl Governance {
    pub fn q_entries(&self) -> &[Entry] {
        &self.q.entries
    }
    pub fn q_contest(&self) -> Option<&Contest> {
        self.q.contest.as_ref()
    }
    fn q_contract(&self) -> Result<&Contract, Error> {
        self.q
            .contract
            .as_ref()
            .ok_or(Error::UnresolvedLaw(UnresolvedLaw::QRecordContract))
    }
    fn q_waiting(&self) -> Vec<&Entry> {
        self.q
            .entries
            .iter()
            .filter(|e| e.status == EntryStatus::Waiting)
            .collect()
    }
    fn q_independent(
        &self,
        proof: &Attestation,
        registry: &BTreeSet<String>,
        parties: &[String],
        cmd: &Command,
    ) -> Result<(), Error> {
        self.attest(proof, cmd)?;
        require(
            registry.contains(&proof.signer)
                && !parties.contains(&proof.signer)
                && proof.signer != self.policy()?.administrator
                && !self.is_officer(&proof.signer),
            "independent authorized Q reviewer required",
        )
    }
    fn q_eligibility(&self, e: &Eligibility, at: Date, cmd: &Command) -> Result<(), Error> {
        stable(&e.person)?;
        require(
            e.valid_until.day < self.calendar()? && e.valid_until >= at,
            "current Q eligibility required",
        )?;
        self.credential(&e.certification, &e.person, cmd)?;
        require(
            self.has_bodily_mastery(&e.person, BodilyForm::Manticorp)
                && Form::ALL.iter().all(|f| self.has_form(&e.person, *f))
                && Pair::ALL
                    .iter()
                    .all(|p| self.ambisynth(&e.person, *p).is_some()),
            "six Forms, three Ambisynth pairs and independent Manticorp Recipe required",
        )
    }
    fn q_claim(&mut self, proof: &Attestation, scope: &str, cmd: &Command) -> Result<(), Error> {
        self.claim(proof, EvidenceClaim::Q(cmd.id.clone(), scope.into()))
    }
    fn q_current_contest(&self, id: &str, cmd: &Command) -> Result<Contest, Error> {
        self.admin(&cmd.actor)?;
        let c = self
            .q
            .contest
            .as_ref()
            .ok_or(Error::Invalid("no scheduled Q contest"))?;
        require(
            c.id == id && c.result.is_none() && cmd.date >= c.scheduled,
            "wrong, completed or premature Q contest",
        )?;
        for readiness in &c.readiness {
            require(
                readiness.valid_until >= cmd.date,
                "contest eligibility expired",
            )?;
        }
        Ok(c.clone())
    }
    pub(super) fn reduce_q(&mut self, action: &Action, cmd: &Command) -> Result<(), Error> {
        if let Action::Configure { contract: c } = action {
            require(
                self.q.contract.is_none(),
                "Q record contract already configured",
            )?;
            self.attest(&c.authorization, cmd)?;
            require(
                cmd.actor == self.policy()?.authorization.signer
                    && c.authorization.signer == cmd.actor
                    && c.authorization.record != self.policy()?.authorization.record,
                "explicit Q record mandate required",
            )?;
            self.court(&c.legacy_review, &BTreeSet::from([cmd.actor.clone()]), cmd)?;
            require(
                c.authorization.record != c.legacy_review.record,
                "separate legacy law review required",
            )?;
            require(
                c.assessors.len() >= 3 && c.judges.len() >= 6 && !c.witnesses.is_empty(),
                "independent Q reviewer registries required",
            )?;
            for actor in c.assessors.iter().chain(&c.judges).chain(&c.witnesses) {
                stable(actor)?;
                require(
                    !self.is_officer(actor) && actor != &self.policy()?.administrator,
                    "Q administrator and officeholders cannot be reviewers",
                )?;
            }
            self.q_claim(&c.authorization, "contract", cmd)?;
            self.q_claim(&c.legacy_review, "legacy-review", cmd)?;
            self.q.contract = Some(c.clone());
            return Ok(());
        }
        self.q_contract()?;
        match action {
            Action::Configure { .. } => unreachable!(),
            Action::Admit { admission: a } => {
                self.admin(&cmd.actor)?;
                stable(&a.id)?;
                let person = &a.eligibility.person;
                self.q_eligibility(&a.eligibility, cmd.date, cmd)?;
                require(
                    self.tross() != Some(person.as_str()),
                    "incumbent cannot enter challenger Q",
                )?;
                require(
                    !self.q.entries.iter().any(|e| {
                        e.admission.id == a.id
                            || (e.admission.eligibility.person == *person
                                && matches!(
                                    e.status,
                                    EntryStatus::Waiting | EntryStatus::InContest(_)
                                ))
                    }),
                    "duplicate Q entry or candidate",
                )?;
                self.attest(&a.consent, cmd)?;
                require(a.consent.signer == *person, "candidate consent required")?;
                let mut signers = BTreeSet::new();
                let mut records = BTreeSet::from([
                    a.eligibility.certification.record.clone(),
                    a.consent.record.clone(),
                ]);
                require(
                    records.len() == 2,
                    "distinct consent and standing evidence required",
                )?;
                for assessment in &a.assessments {
                    self.q_independent(
                        &assessment.finding,
                        &self.q_contract()?.assessors,
                        std::slice::from_ref(person),
                        cmd,
                    )?;
                    require(
                        signers.insert(&assessment.finding.signer)
                            && records.insert(assessment.finding.record.clone()),
                        "three distinct Q assessors and findings required",
                    )?;
                }
                require(
                    a.assessments.iter().filter(|a| a.affirmative).count() >= 2,
                    "two of three affirmative Q assessments required",
                )?;
                self.q_claim(&a.eligibility.certification, "eligibility", cmd)?;
                self.q_claim(&a.consent, "consent", cmd)?;
                for (i, assessment) in a.assessments.iter().enumerate() {
                    self.q_claim(&assessment.finding, &format!("assessment.{i}"), cmd)?;
                }
                if self.q_waiting().is_empty() {
                    self.q.front_since = Some(cmd.date);
                }
                self.q.entries.push(Entry {
                    admission: a.clone(),
                    admitted: cmd.date,
                    status: EntryStatus::Waiting,
                });
            }
            Action::Withdraw { entry } | Action::Expire { entry } => {
                let old_front = self.q_waiting().first().map(|e| e.admission.id.clone());
                let i = self
                    .q
                    .entries
                    .iter()
                    .position(|e| e.admission.id == *entry && e.status == EntryStatus::Waiting)
                    .ok_or(Error::Invalid("waiting Q entry required"))?;
                let e = &self.q.entries[i];
                let status = if matches!(action, Action::Withdraw { .. }) {
                    require(
                        cmd.actor == e.admission.eligibility.person,
                        "only candidate can withdraw",
                    )?;
                    EntryStatus::Withdrawn
                } else {
                    self.admin(&cmd.actor)?;
                    require(
                        cmd.date > e.admission.eligibility.valid_until,
                        "Q eligibility has not expired",
                    )?;
                    EntryStatus::Expired
                };
                self.q.entries[i].status = status;
                if old_front.as_deref() == Some(entry) {
                    self.q.front_since = (!self.q_waiting().is_empty()).then_some(cmd.date);
                }
            }
            Action::Schedule {
                contest,
                entries,
                at,
                readiness,
            } => {
                self.admin(&cmd.actor)?;
                stable(contest)?;
                // Until succession/appeal/recovery law is integrated, a prior
                // contest cannot be discarded to schedule around its outcome.
                if self.q.contest.is_some() {
                    return Err(Error::UnresolvedLaw(
                        UnresolvedLaw::QSuccessionReconciliation,
                    ));
                }
                require(
                    at.day < self.calendar()? && *at >= cmd.date,
                    "valid future Q date required",
                )?;
                let incumbent = self.tross().ok_or(Error::UnresolvedLaw(
                    UnresolvedLaw::QSuccessionReconciliation,
                ))?;
                let needed = 1;
                let waiting = self.q_waiting();
                require(
                    waiting.len() >= needed && entries.len() == needed,
                    "earliest certified candidate(s) required; vacancy is preserved",
                )?;
                require(
                    waiting
                        .iter()
                        .take(needed)
                        .map(|e| &e.admission.id)
                        .eq(entries.iter()),
                    "The Q is neutral FIFO",
                )?;
                let deadline = self.add_days(
                    self.q
                        .front_since
                        .ok_or(Error::Invalid("missing queue clock"))?,
                    30,
                )?;
                require(
                    *at <= deadline,
                    "front challenger must be scheduled within thirty days",
                )?;
                let first = waiting[0].admission.eligibility.person.clone();
                let second = incumbent.to_owned();
                let participants = [first, second];
                for (e, person) in readiness.iter().zip(&participants) {
                    require(e.person == *person, "readiness subject mismatch")?;
                    self.q_eligibility(e, *at, cmd)?;
                    require(
                        !participants.contains(&e.certification.signer),
                        "participant cannot certify readiness",
                    )?;
                }
                for entry in waiting.iter().take(needed) {
                    require(
                        entry.admission.eligibility.valid_until >= *at,
                        "admission expires before contest",
                    )?;
                    for assessment in &entry.admission.assessments {
                        require(
                            !participants.contains(&assessment.finding.signer),
                            "assessor conflicts with contest participant",
                        )?;
                    }
                }
                for (i, e) in readiness.iter().enumerate() {
                    self.q_claim(&e.certification, &format!("readiness.{i}"), cmd)?;
                }
                for e in &mut self.q.entries {
                    if entries.contains(&e.admission.id) {
                        e.status = EntryStatus::InContest(contest.clone());
                    }
                }
                self.q.front_since = (!self.q_waiting().is_empty()).then_some(cmd.date);
                self.q.contest = Some(Contest {
                    id: contest.clone(),
                    participants,
                    scheduled: *at,
                    readiness: readiness.clone(),
                    rounds: vec![],
                    sudden_death: None,
                    result: None,
                });
            }
            Action::RecordRound {
                contest,
                round,
                panel,
            } => {
                let mut c = self.q_current_contest(contest, cmd)?;
                let expected = if c.rounds.is_empty() {
                    Round::CloseOffPoseOff
                } else {
                    Round::SocialEngineering
                };
                require(
                    c.rounds.len() < 2 && *round == expected,
                    "Q opening rounds must occur once in order",
                )?;
                let previous: BTreeSet<_> = c
                    .rounds
                    .iter()
                    .flat_map(|(_, panel, _)| panel.iter().map(|v| &v.finding.signer))
                    .collect();
                let assessors: BTreeSet<_> = self
                    .q
                    .entries
                    .iter()
                    .filter(|e| e.status == EntryStatus::InContest(contest.clone()))
                    .flat_map(|e| e.admission.assessments.iter().map(|a| &a.finding.signer))
                    .collect();
                let mut signers = BTreeSet::new();
                for vote in panel {
                    self.q_independent(
                        &vote.finding,
                        &self.q_contract()?.judges,
                        &c.participants,
                        cmd,
                    )?;
                    require(
                        c.participants.contains(&vote.winner)
                            && signers.insert(&vote.finding.signer)
                            && !previous.contains(&vote.finding.signer)
                            && !assessors.contains(&vote.finding.signer),
                        "distinct independent panels and participant-only verdicts required",
                    )?;
                }
                let winner = c
                    .participants
                    .iter()
                    .find(|person| panel.iter().filter(|v| &v.winner == *person).count() >= 2)
                    .ok_or(Error::Invalid("two of three Q verdict required"))?
                    .clone();
                for (i, vote) in panel.iter().enumerate() {
                    self.q_claim(&vote.finding, &format!("judgment.{i}"), cmd)?;
                }
                c.rounds.push((*round, panel.clone(), winner.clone()));
                if c.rounds.len() == 2 && c.rounds[0].2 == winner {
                    c.result = Some(winner);
                }
                self.q.contest = Some(c);
            }
            Action::RecordSuddenDeath {
                contest,
                loser,
                reason,
                outcome,
                witness,
            } => {
                let mut c = self.q_current_contest(contest, cmd)?;
                require(
                    c.rounds.len() == 2
                        && c.rounds[0].2 != c.rounds[1].2
                        && c.participants.contains(loser),
                    "Sudden Death requires a split contest and actual participant",
                )?;
                self.attest(outcome, cmd)?;
                match reason {
                    EndReason::Concession => require(
                        outcome.signer == *loser,
                        "concession must be authenticated to the conceding participant",
                    )?,
                    EndReason::InabilityToContinue => self.q_independent(
                        outcome,
                        &self.policy()?.execution_reviewers,
                        &c.participants,
                        cmd,
                    )?,
                }
                self.q_independent(witness, &self.q_contract()?.witnesses, &c.participants, cmd)?;
                require(
                    witness.signer != outcome.signer && witness.record != outcome.record,
                    "independent witness and outcome required",
                )?;
                self.q_claim(outcome, "sudden-death-outcome", cmd)?;
                self.q_claim(witness, "sudden-death-witness", cmd)?;
                c.result = c.participants.iter().find(|p| *p != loser).cloned();
                c.sudden_death = Some((loser.clone(), *reason, outcome.clone(), witness.clone()));
                self.q.contest = Some(c);
            }
            Action::FinalizeSuccession { .. } | Action::OpenFinalVacancy => {
                return Err(Error::UnresolvedLaw(
                    UnresolvedLaw::QSuccessionReconciliation,
                ));
            }
        }
        Ok(())
    }
}
