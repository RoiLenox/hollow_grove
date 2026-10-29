//! Identity and custody of externally existing Triple Crown equipment.
//! No fabrication, ammunition, firing, damage or Factory authority is modeled.
use super::*;

pub const TRIPLE_CROWN_ID: &str = "equipment.flynt.triple-crown.revolution-c.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Component {
    GargoyleBolo,
    WerewolfCrossbow,
    MermanHarpoon,
}
impl Component {
    pub const ALL: [Self; 3] = [
        Self::GargoyleBolo,
        Self::WerewolfCrossbow,
        Self::MermanHarpoon,
    ];
    pub const fn form(self) -> Form {
        match self {
            Self::GargoyleBolo => Form::Gargoyle,
            Self::WerewolfCrossbow => Form::Werewolf,
            Self::MermanHarpoon => Form::Merman,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentRecord {
    pub component: Component,
    /// Identity of an existing tangible component, verified by the external inspector.
    pub item: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Registration {
    pub specification: String,
    pub item: String,
    pub owner: String,
    pub custodian: String,
    pub components: [ComponentRecord; 3],
    pub inspection: Attestation,
    pub owner_consent: Attestation,
    pub custodian_consent: Attestation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "step", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Action {
    Register {
        registration: Registration,
    },
    Transfer {
        item: String,
        from: String,
        to: String,
        inspection: Attestation,
        owner_consent: Attestation,
        outgoing_consent: Attestation,
        incoming_consent: Attestation,
    },
    /// Withdrawing presentation/custody permission is always possible.
    Release {
        item: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Custody {
    pub registration: Registration,
    pub custodian: Option<String>,
}

impl Governance {
    pub fn triple_crown(&self, item: &str) -> Option<&Custody> {
        self.crowns.get(item)
    }
    /// Bounded equipment presentation only, never permission to fire or fabricate.
    pub fn may_present_triple_crown(&self, person: &str, item: &str) -> bool {
        self.crowns
            .get(item)
            .is_some_and(|c| c.custodian.as_deref() == Some(person))
            && self.has_bodily_mastery(person, BodilyForm::Chimera)
            && self.expressed_form(person) == Some(BodilyForm::Chimera)
    }
    fn crown_proofs(
        &mut self,
        inspection: &Attestation,
        consents: &[(&Attestation, &str)],
        cmd: &Command,
    ) -> Result<(), Error> {
        self.attest(inspection, cmd)?;
        require(
            cmd.actor == inspection.signer
                && self.policy()?.execution_reviewers.contains(&cmd.actor)
                && !self.is_officer(&cmd.actor)
                && cmd.actor != self.policy()?.administrator
                && consents.iter().all(|(_, person)| *person != cmd.actor),
            "independent authenticated component/custody inspection required",
        )?;
        self.claim(
            inspection,
            EvidenceClaim::TripleCrown(cmd.id.clone(), "inspection".into()),
        )?;
        for (i, (consent, person)) in consents.iter().enumerate() {
            self.attest(consent, cmd)?;
            require(
                consent.signer == *person,
                "scoped owner and custodian consent required",
            )?;
            self.claim(
                consent,
                EvidenceClaim::TripleCrown(cmd.id.clone(), format!("consent.{i}")),
            )?;
        }
        Ok(())
    }
    pub(super) fn reduce_crown(&mut self, action: &Action, cmd: &Command) -> Result<(), Error> {
        match action {
            Action::Register { registration: r } => {
                for id in [&r.item, &r.owner, &r.custodian] {
                    stable(id)?;
                }
                require(
                    r.specification == TRIPLE_CROWN_ID && !self.crowns.contains_key(&r.item),
                    "unique Triple Crown identity and correct specification required",
                )?;
                require(
                    self.has_bodily_mastery(&r.custodian, BodilyForm::Chimera),
                    "Chimera qualification required for Triple Crown custody",
                )?;
                require(
                    r.components
                        .iter()
                        .map(|c| c.component)
                        .collect::<BTreeSet<_>>()
                        == Component::ALL.into_iter().collect(),
                    "exact bolo/crossbow/harpoon composition required",
                )?;
                require(
                    r.components
                        .iter()
                        .map(|c| &c.item)
                        .collect::<BTreeSet<_>>()
                        .len()
                        == 3,
                    "three distinct tangible components required",
                )?;
                for c in &r.components {
                    stable(&c.item)?;
                    require(
                        c.item != r.item
                            && !self.crowns.contains_key(&c.item)
                            && !self.crowns.values().any(|old| {
                                old.registration
                                    .components
                                    .iter()
                                    .any(|part| part.item == c.item || part.item == r.item)
                            }),
                        "existing item/component identities cannot be duplicated or consumed again",
                    )?;
                }
                self.crown_proofs(
                    &r.inspection,
                    &[
                        (&r.owner_consent, &r.owner),
                        (&r.custodian_consent, &r.custodian),
                    ],
                    cmd,
                )?;
                self.crowns.insert(
                    r.item.clone(),
                    Custody {
                        registration: r.clone(),
                        custodian: Some(r.custodian.clone()),
                    },
                );
            }
            Action::Transfer {
                item,
                from,
                to,
                inspection,
                owner_consent,
                outgoing_consent,
                incoming_consent,
            } => {
                stable(to)?;
                let c = self
                    .crowns
                    .get(item)
                    .ok_or(Error::Invalid("unknown Triple Crown"))?;
                require(
                    c.custodian.as_deref() == Some(from) && from != to,
                    "current distinct custodian required",
                )?;
                require(
                    self.has_bodily_mastery(to, BodilyForm::Chimera),
                    "incoming custodian must independently qualify as Chimera",
                )?;
                let owner = c.registration.owner.clone();
                self.crown_proofs(
                    inspection,
                    &[
                        (owner_consent, &owner),
                        (outgoing_consent, from),
                        (incoming_consent, to),
                    ],
                    cmd,
                )?;
                self.crowns.get_mut(item).expect("validated item").custodian = Some(to.clone());
            }
            Action::Release { item } => {
                let c = self
                    .crowns
                    .get_mut(item)
                    .ok_or(Error::Invalid("unknown Triple Crown"))?;
                require(
                    c.custodian.is_some()
                        && (c.custodian.as_deref() == Some(&cmd.actor)
                            || c.registration.owner == cmd.actor),
                    "owner or current custodian required",
                )?;
                c.custodian = None;
            }
        }
        Ok(())
    }
}
