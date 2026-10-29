//! Minimal clinical decisions shared by House governance adapters.
//!
//! The trusted caller authenticates the mandate, credential certification,
//! clearance and consent records independently. This journal projection contains
//! only scoped decisions and evidence references, never clinical/private memory.
//! It neither issues professional credentials nor authenticates signer strings.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::governance::Attestation;
use crate::hollow_grove_contract::House;
use crate::institution::IdentityId;

pub const POLICY_VERSION: u32 = 2;

/// A governance manifestation, not permission for arbitrary clinical treatment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Procedure {
    pub house: House,
    pub form: String,
}

/// An independently certified authorization for one reviewer and subject.
/// Clearance and subject consent are separate, authenticated external records.
/// All three permissions must cover this exact procedure and the entire supplied
/// interval. Time uses the receiving adapter's in-world years, [from, until).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Authorization {
    pub record: String,
    pub subject: String,
    pub procedure: Procedure,
    pub valid_from: u64,
    pub valid_until: u64,
    pub clearance: Attestation,
    pub consent: Attestation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "update", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Update {
    /// A new lawful mandate, never an inference from an office or old events.
    AdoptPolicy {
        version: u32,
        authorization: Attestation,
        credential_authorities: BTreeSet<String>,
    },
    /// Record a verified clinical credential decision, not a self-certification.
    Register {
        authorization: Authorization,
        certification: Attestation,
    },
    /// Credential revocation, clearance withdrawal or subject consent withdrawal
    /// disables the combined authorization. Reinstatement needs fresh records.
    Revoke {
        record: String,
        certification: Attestation,
    },
}

pub(crate) struct Context<'a> {
    pub house: House,
    pub year: u64,
    pub actor: &'a str,
    pub evidence: &'a BTreeSet<String>,
    pub policy_authorization: &'a Attestation,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Authorizations {
    authorities: Option<BTreeSet<String>>,
    records: BTreeMap<String, Authorization>,
    used_evidence: BTreeSet<String>,
    revoked: BTreeSet<String>,
}

type Result<T> = std::result::Result<T, &'static str>;

fn require(condition: bool, reason: &'static str) -> Result<()> {
    if condition { Ok(()) } else { Err(reason) }
}

fn stable(id: &str) -> Result<()> {
    IdentityId::new(id)
        .map(|_| ())
        .map_err(|_| "invalid clinical record identity")
}

fn attest(proof: &Attestation, context: &Context<'_>) -> Result<()> {
    stable(&proof.record)?;
    stable(&proof.signer)?;
    require(
        context.evidence.contains(&proof.record),
        "clinical attestation missing from evidence",
    )
}

impl Authorizations {
    pub(crate) fn active(&self) -> bool {
        self.authorities.is_some()
    }

    // The receiving governance adapter validates and applies on a clone.
    pub(crate) fn apply(&mut self, update: &Update, context: Context<'_>) -> Result<()> {
        match update {
            Update::AdoptPolicy {
                version,
                authorization,
                credential_authorities,
            } => {
                require(!self.active(), "clinical policy already adopted")?;
                require(
                    *version == POLICY_VERSION,
                    "unsupported clinical policy version",
                )?;
                attest(authorization, &context)?;
                require(
                    context.actor == context.policy_authorization.signer
                        && authorization.signer == context.actor
                        && authorization.record != context.policy_authorization.record,
                    "independent clinical policy mandate required",
                )?;
                require(
                    !credential_authorities.is_empty(),
                    "clinical authorities required",
                )?;
                for authority in credential_authorities {
                    stable(authority)?;
                }
                self.used_evidence.insert(authorization.record.clone());
                self.authorities = Some(credential_authorities.clone());
            }
            Update::Register {
                authorization: a,
                certification,
            } => {
                let authorities = self
                    .authorities
                    .as_ref()
                    .ok_or("clinical policy required")?;
                attest(certification, &context)?;
                attest(&a.clearance, &context)?;
                attest(&a.consent, &context)?;
                stable(&a.record)?;
                stable(&a.subject)?;
                stable(&a.procedure.form)?;
                require(
                    authorities.contains(context.actor)
                        && certification.signer == context.actor
                        && context.actor != a.clearance.signer
                        && context.actor != a.subject,
                    "independent clinical credential authority required",
                )?;
                require(
                    a.clearance.signer != a.subject && a.consent.signer == a.subject,
                    "independent reviewer and personal consent required",
                )?;
                require(
                    a.procedure.house == context.house,
                    "wrong clinical House scope",
                )?;
                require(
                    a.valid_from < a.valid_until && context.year < a.valid_until,
                    "invalid or expired clinical authorization interval",
                )?;
                let ids = BTreeSet::from([
                    a.record.clone(),
                    certification.record.clone(),
                    a.clearance.record.clone(),
                    a.consent.record.clone(),
                ]);
                require(
                    ids.len() == 4 && ids.is_disjoint(&self.used_evidence),
                    "clinical authorization, certification, clearance and consent must be distinct and unused",
                )?;
                require(
                    context.evidence.contains(&a.record),
                    "clinical authorization missing from evidence",
                )?;
                self.used_evidence.extend(ids);
                self.records.insert(a.record.clone(), a.clone());
            }
            Update::Revoke {
                record,
                certification,
            } => {
                let authorities = self
                    .authorities
                    .as_ref()
                    .ok_or("clinical policy required")?;
                let a = self
                    .records
                    .get(record)
                    .ok_or("unknown clinical authorization")?;
                attest(certification, &context)?;
                require(
                    certification.signer == context.actor
                        && (authorities.contains(context.actor)
                            || context.actor == a.clearance.signer
                            || context.actor == a.subject),
                    "clinical revocation authority required",
                )?;
                require(
                    !self.revoked.contains(record),
                    "clinical authorization already revoked",
                )?;
                require(
                    !self.used_evidence.contains(&certification.record),
                    "clinical revocation needs fresh evidence",
                )?;
                self.used_evidence.insert(certification.record.clone());
                self.revoked.insert(record.clone());
            }
        }
        Ok(())
    }

    pub(crate) fn authorize(
        &self,
        procedure: &Procedure,
        subject: &str,
        year: u64,
        clearance: &Attestation,
        consent: &Attestation,
    ) -> Result<()> {
        require(self.active(), "clinical policy required")?;
        let a = self
            .records
            .values()
            .find(|a| a.clearance.record == clearance.record)
            .ok_or("unregistered clinical clearance")?;
        require(
            !self.revoked.contains(&a.record),
            "revoked clinical authorization",
        )?;
        require(
            a.subject == subject && a.procedure == *procedure,
            "wrong clinical subject or procedure scope",
        )?;
        require(
            a.valid_from <= year && year < a.valid_until,
            "clinical authorization outside validity interval",
        )?;
        require(
            a.clearance == *clearance && a.consent == *consent,
            "clearance or consent does not match certified records",
        )
    }
}
