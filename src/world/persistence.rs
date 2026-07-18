//! Versioned persistence for dynamic institutional state.
//!
//! Canonical catalogs are code fixtures and are rebuilt on load. This artifact
//! persists only the mutable records layered over those fixtures.

use std::io;

use crate::institution::{
    ClearanceLevel, GroupId, InstitutionId, InstitutionalBeingId, InstitutionalEntityId,
    MembershipId, OfficeId, RoleId, SiteId, Visibility, ZoneId,
};
use crate::institution_affiliation::*;

pub const INSTITUTIONAL_STATE_ARTIFACT_PATH: &str = "artifacts/institutional_state.txt";
pub const INSTITUTIONAL_STATE_SCHEMA_VERSION: &str = "1";

#[must_use]
pub fn build_persisted_state_output(state: &InstitutionalWorldState) -> String {
    let mut output = format!("schema_version:{}\n", INSTITUTIONAL_STATE_SCHEMA_VERSION);
    for entry in &state.memberships {
        record(
            &mut output,
            "membership",
            &[
                entry.id.as_str().into(),
                entry.being.as_str().into(),
                entry.institution.as_str().into(),
                optional_role_id(&entry.role_id),
                format!("{:?}", entry.role),
                format!("{:?}", entry.affiliation_state),
                format!("{:?}", entry.lineage),
                optional_being(&entry.sponsor),
                optional_group(&entry.subgroup),
                optional_time(entry.joined_at),
                optional_time(entry.initiated_at),
                optional_time(entry.ended_at),
                format!("{:?}", entry.public_visibility),
                format!("{:?}", entry.internal_recognition),
            ],
        );
    }
    for entry in &state.sponsorships {
        record(
            &mut output,
            "sponsorship",
            &[
                entry.id.as_str().into(),
                entry.sponsor.as_str().into(),
                entry.candidate.as_str().into(),
                entry.institution.as_str().into(),
                entry.active.to_string(),
                entry.started_at.to_string(),
                optional_time(entry.ended_at),
                format!("{:?}", entry.liability),
            ],
        );
    }
    for entry in &state.obligations {
        record(
            &mut output,
            "obligation",
            &[
                entry.id.as_str().into(),
                entity_text(&entry.debtor),
                entity_text(&entry.creditor),
                entry.institution.as_str().into(),
                format!("{:?}", entry.kind),
                format!("{:?}", entry.status),
                format!("{:?}", entry.weight),
            ],
        );
    }
    for entry in &state.claims {
        record(
            &mut output,
            "claim",
            &[
                entry.id.as_str().into(),
                entry.claimant.as_str().into(),
                entry.institution.as_str().into(),
                format!("{:?}", entry.claimed_state),
                optional_role(entry.claimed_role),
                format!("{:?}", entry.truth_status),
                format!("{:?}", entry.visibility),
            ],
        );
    }
    for entry in &state.access_grants {
        record(
            &mut output,
            "access-grant",
            &[
                entry.id.as_str().into(),
                entry.grantee.as_str().into(),
                entry.institution.as_str().into(),
                optional_site(&entry.site),
                optional_zone(&entry.zone),
                optional_clearance(entry.clearance),
                entry.active.to_string(),
            ],
        );
    }
    for entry in &state.events {
        record(
            &mut output,
            "event",
            &[
                entry.id.as_str().into(),
                format!("{:?}", entry.kind),
                entry.institution.as_str().into(),
                entry.subject.as_str().into(),
                entry.at.to_string(),
            ],
        );
    }
    output
}

pub fn parse_persisted_state(
    contents: &str,
    catalog: crate::institution::InstitutionCatalog,
) -> io::Result<InstitutionalWorldState> {
    let mut lines = contents.lines();
    let version = lines
        .next()
        .and_then(|line| line.strip_prefix("schema_version:"));
    if version != Some(INSTITUTIONAL_STATE_SCHEMA_VERSION) {
        return Err(invalid("unsupported institutional state schema"));
    }
    let mut state = InstitutionalWorldState::from_catalog(catalog);
    for (line_number, line) in lines.enumerate() {
        if line.is_empty() {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        let kind = *fields
            .first()
            .ok_or_else(|| invalid("empty institutional record"))?;
        match kind {
            "membership" => state.memberships.push(parse_membership(&fields)?),
            "sponsorship" => state.sponsorships.push(parse_sponsorship(&fields)?),
            "obligation" => state.obligations.push(parse_obligation(&fields)?),
            "claim" => state.claims.push(parse_claim(&fields)?),
            "access-grant" => state.access_grants.push(parse_access_grant(&fields)?),
            "event" => state.events.push(parse_event(&fields)?),
            _ => {
                return Err(invalid(&format!(
                    "unknown institutional record at line {}",
                    line_number + 2
                )));
            }
        }
    }
    state
        .validate()
        .map_err(|error| invalid(&format!("invalid institutional state: {error:?}")))?;
    Ok(state)
}

fn record(output: &mut String, kind: &str, fields: &[String]) {
    output.push_str(kind);
    for field in fields {
        output.push('\t');
        output.push_str(field);
    }
    output.push('\n');
}
fn optional_time(value: Option<WorldTimestamp>) -> String {
    value.map_or_else(|| "-".into(), |time| time.to_string())
}
fn optional_role_id(value: &Option<RoleId>) -> String {
    value
        .as_ref()
        .map_or_else(|| "-".into(), |id| id.as_str().into())
}
fn optional_being(value: &Option<InstitutionalBeingId>) -> String {
    value
        .as_ref()
        .map_or_else(|| "-".into(), |id| id.as_str().into())
}
fn optional_group(value: &Option<GroupId>) -> String {
    value
        .as_ref()
        .map_or_else(|| "-".into(), |id| id.as_str().into())
}
fn optional_site(value: &Option<SiteId>) -> String {
    value
        .as_ref()
        .map_or_else(|| "-".into(), |id| id.as_str().into())
}
fn optional_zone(value: &Option<ZoneId>) -> String {
    value
        .as_ref()
        .map_or_else(|| "-".into(), |id| id.as_str().into())
}
fn optional_clearance(value: Option<ClearanceLevel>) -> String {
    value.map_or_else(|| "-".into(), |value| format!("{value:?}"))
}
fn optional_role(value: Option<MembershipRole>) -> String {
    value.map_or_else(|| "-".into(), |value| format!("{value:?}"))
}

fn entity_text(entity: &InstitutionalEntityId) -> String {
    match entity {
        InstitutionalEntityId::Institution(id) => format!("institution:{}", id.as_str()),
        InstitutionalEntityId::Office(id) => format!("office:{}", id.as_str()),
        InstitutionalEntityId::Group(id) => format!("group:{}", id.as_str()),
        InstitutionalEntityId::Site(id) => format!("site:{}", id.as_str()),
        InstitutionalEntityId::Being(id) => format!("being:{}", id.as_str()),
    }
}
fn parse_entity(value: &str) -> io::Result<InstitutionalEntityId> {
    let (kind, id) = value
        .split_once(':')
        .ok_or_else(|| invalid("invalid institutional entity"))?;
    Ok(match kind {
        "institution" => InstitutionalEntityId::Institution(institution(id)?),
        "office" => InstitutionalEntityId::Office(office(id)?),
        "group" => InstitutionalEntityId::Group(group(id)?),
        "site" => InstitutionalEntityId::Site(site(id)?),
        "being" => InstitutionalEntityId::Being(being(id)?),
        _ => return Err(invalid("unknown institutional entity")),
    })
}

fn parse_membership(values: &[&str]) -> io::Result<InstitutionalMembership> {
    expect(values, 14, "membership")?;
    Ok(InstitutionalMembership {
        id: membership(values[1])?,
        being: being(values[2])?,
        institution: institution(values[3])?,
        role_id: optional(values[4], role)?,
        role: membership_role(values[5])?,
        affiliation_state: affiliation(values[6])?,
        lineage: lineage(values[7])?,
        sponsor: optional(values[8], being)?,
        subgroup: optional(values[9], group)?,
        joined_at: optional_number(values[10])?,
        initiated_at: optional_number(values[11])?,
        ended_at: optional_number(values[12])?,
        public_visibility: visibility(values[13])?,
        internal_recognition: recognition(values[14])?,
    })
}
fn parse_sponsorship(values: &[&str]) -> io::Result<Sponsorship> {
    expect(values, 8, "sponsorship")?;
    Ok(Sponsorship {
        id: sponsorship_id(values[1])?,
        sponsor: being(values[2])?,
        candidate: being(values[3])?,
        institution: institution(values[4])?,
        active: boolean(values[5])?,
        started_at: number(values[6])?,
        ended_at: optional_number(values[7])?,
        liability: liability(values[8])?,
    })
}
fn parse_obligation(values: &[&str]) -> io::Result<InstitutionalObligation> {
    expect(values, 7, "obligation")?;
    Ok(InstitutionalObligation {
        id: obligation_id(values[1])?,
        debtor: parse_entity(values[2])?,
        creditor: parse_entity(values[3])?,
        institution: institution(values[4])?,
        kind: obligation_kind(values[5])?,
        status: obligation_status(values[6])?,
        weight: obligation_weight(values[7])?,
    })
}
fn parse_claim(values: &[&str]) -> io::Result<AffiliationClaim> {
    expect(values, 7, "claim")?;
    Ok(AffiliationClaim {
        id: claim_id(values[1])?,
        claimant: being(values[2])?,
        institution: institution(values[3])?,
        claimed_state: affiliation(values[4])?,
        claimed_role: optional(values[5], membership_role)?,
        truth_status: claim_truth(values[6])?,
        visibility: visibility(values[7])?,
    })
}
fn parse_access_grant(values: &[&str]) -> io::Result<AccessGrant> {
    expect(values, 7, "access-grant")?;
    Ok(AccessGrant {
        id: access_grant_id(values[1])?,
        grantee: being(values[2])?,
        institution: institution(values[3])?,
        site: optional(values[4], site)?,
        zone: optional(values[5], zone)?,
        clearance: optional(values[6], clearance)?,
        active: boolean(values[7])?,
    })
}
fn parse_event(values: &[&str]) -> io::Result<InstitutionalEvent> {
    expect(values, 5, "event")?;
    Ok(InstitutionalEvent {
        id: event_id(values[1])?,
        kind: event_kind(values[2])?,
        institution: institution(values[3])?,
        subject: being(values[4])?,
        at: number(values[5])?,
    })
}

fn expect(values: &[&str], fields: usize, kind: &str) -> io::Result<()> {
    if values.len() == fields + 1 {
        Ok(())
    } else {
        Err(invalid(&format!("invalid {kind} record")))
    }
}
fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.to_owned())
}
fn number(value: &str) -> io::Result<u64> {
    value.parse().map_err(|_| invalid("invalid number"))
}
fn optional_number(value: &str) -> io::Result<Option<u64>> {
    if value == "-" {
        Ok(None)
    } else {
        number(value).map(Some)
    }
}
fn boolean(value: &str) -> io::Result<bool> {
    value.parse().map_err(|_| invalid("invalid boolean"))
}
fn optional<T>(value: &str, parse: impl FnOnce(&str) -> io::Result<T>) -> io::Result<Option<T>> {
    if value == "-" {
        Ok(None)
    } else {
        parse(value).map(Some)
    }
}

macro_rules! enum_parser { ($name:ident, $type:ident, [$($variant:ident),+ $(,)?]) => { fn $name(value: &str) -> io::Result<$type> { match value { $(stringify!($variant) => Ok($type::$variant),)+ _ => Err(invalid(concat!("invalid ", stringify!($type)))), } } }; }
enum_parser!(
    membership_role,
    MembershipRole,
    [
        Candidate, Associate, FullMember, Officer, Sponsor, Elder, Leader
    ]
);
enum_parser!(
    affiliation,
    AffiliationState,
    [
        None,
        Candidate,
        Associate,
        RecognizedAssociate,
        Initiated,
        Senior,
        Former,
        Suspended,
        Expelled
    ]
);
enum_parser!(
    lineage,
    LineageStatus,
    [
        None,
        AffiliatedFamily,
        Inherited,
        Founding,
        Disputed,
        Disowned
    ]
);
enum_parser!(
    recognition,
    RecognitionLevel,
    [None, Provisional, Internal, Established]
);
enum_parser!(
    visibility,
    Visibility,
    [Public, Known, Restricted, Classified, Hidden]
);
enum_parser!(
    liability,
    SponsorshipLiability,
    [None, Social, Financial, Disciplinary, Blood]
);
enum_parser!(
    obligation_kind,
    ObligationKind,
    [
        Favor,
        Financial,
        Protection,
        Silence,
        Service,
        Sponsorship,
        Restitution,
        BloodDebt,
        ArtisticCommission
    ]
);
enum_parser!(
    obligation_status,
    ObligationStatus,
    [
        Open,
        Called,
        PartiallyPaid,
        Settled,
        Forgiven,
        Defaulted,
        Inherited
    ]
);
enum_parser!(
    obligation_weight,
    ObligationWeight,
    [Minor, Significant, Major, Severe]
);
enum_parser!(
    claim_truth,
    ClaimTruth,
    [Unknown, True, False, PartiallyTrue, FormerlyTrue, Disputed]
);
enum_parser!(
    clearance,
    ClearanceLevel,
    [Basic, Restricted, Classified, Black]
);
enum_parser!(
    event_kind,
    InstitutionalEventKind,
    [
        RecruitmentStarted,
        CandidateAccepted,
        SponsorshipCreated,
        SponsorshipEnded,
        InitiationApproved,
        InitiationCompleted,
        MembershipRecognized,
        AffiliationClaimed,
        AffiliationVerified,
        AffiliationDisputed,
        PrestigeChanged,
        AccessGranted,
        AccessRevoked,
        ObligationCreated,
        ObligationSettled,
        MemberSuspended,
        MemberExpelled,
        MemberRestored
    ]
);

fn institution(value: &str) -> io::Result<InstitutionId> {
    InstitutionId::new(value).map_err(|_| invalid("invalid institution ID"))
}
fn office(value: &str) -> io::Result<OfficeId> {
    OfficeId::new(value).map_err(|_| invalid("invalid office ID"))
}
fn group(value: &str) -> io::Result<GroupId> {
    GroupId::new(value).map_err(|_| invalid("invalid group ID"))
}
fn site(value: &str) -> io::Result<SiteId> {
    SiteId::new(value).map_err(|_| invalid("invalid site ID"))
}
fn zone(value: &str) -> io::Result<ZoneId> {
    ZoneId::new(value).map_err(|_| invalid("invalid zone ID"))
}
fn role(value: &str) -> io::Result<RoleId> {
    RoleId::new(value).map_err(|_| invalid("invalid role ID"))
}
fn membership(value: &str) -> io::Result<MembershipId> {
    MembershipId::new(value).map_err(|_| invalid("invalid membership ID"))
}
fn being(value: &str) -> io::Result<InstitutionalBeingId> {
    InstitutionalBeingId::new(value).map_err(|_| invalid("invalid being ID"))
}
fn sponsorship_id(value: &str) -> io::Result<SponsorshipId> {
    SponsorshipId::new(value).ok_or_else(|| invalid("invalid sponsorship ID"))
}
fn obligation_id(value: &str) -> io::Result<ObligationId> {
    ObligationId::new(value).ok_or_else(|| invalid("invalid obligation ID"))
}
fn claim_id(value: &str) -> io::Result<ClaimId> {
    ClaimId::new(value).ok_or_else(|| invalid("invalid claim ID"))
}
fn access_grant_id(value: &str) -> io::Result<AccessGrantId> {
    AccessGrantId::new(value).ok_or_else(|| invalid("invalid access grant ID"))
}
fn event_id(value: &str) -> io::Result<EventId> {
    EventId::new(value).ok_or_else(|| invalid("invalid event ID"))
}

#[cfg(test)]
mod tests {
    use super::{build_persisted_state_output, parse_persisted_state};
    use crate::institution::{
        InstitutionalBeingId, InstitutionalEntityId, MembershipId, Visibility,
    };
    use crate::institution_affiliation::{
        AffiliationState, ClaimTruth, InstitutionalEvent, InstitutionalEventKind,
        InstitutionalMembership, InstitutionalObligation, LineageStatus, MembershipRole,
        ObligationId, ObligationKind, ObligationStatus, ObligationWeight, RecognitionLevel,
        Sponsorship, SponsorshipId, SponsorshipLiability,
    };
    use crate::world::institutional_access_fixture;

    #[test]
    fn dynamic_institutional_records_round_trip_against_canonical_catalog() {
        let mut state = institutional_access_fixture();
        let member = state.memberships[0].clone();
        let sponsor = state.memberships[3].clone();
        let candidate = InstitutionalBeingId::new("being.flynt.persistence-candidate").unwrap();
        state.memberships.push(InstitutionalMembership {
            id: MembershipId::new("membership.flynt.persistence-candidate").unwrap(),
            being: candidate.clone(),
            institution: sponsor.institution.clone(),
            role_id: Some(crate::world::flynt::gallowry::noose_role_id()),
            role: MembershipRole::Candidate,
            affiliation_state: AffiliationState::Candidate,
            lineage: LineageStatus::None,
            sponsor: Some(sponsor.being.clone()),
            subgroup: None,
            joined_at: Some(8),
            initiated_at: None,
            ended_at: None,
            public_visibility: Visibility::Known,
            internal_recognition: RecognitionLevel::Provisional,
        });
        state.sponsorships.push(Sponsorship {
            id: SponsorshipId::new("sponsorship.flynt.persistence-candidate").unwrap(),
            sponsor: sponsor.being.clone(),
            candidate: candidate.clone(),
            institution: sponsor.institution.clone(),
            active: true,
            started_at: 8,
            ended_at: None,
            liability: SponsorshipLiability::Social,
        });
        state.obligations.push(InstitutionalObligation {
            id: ObligationId::new("obligation.flynt.persistence-candidate").unwrap(),
            debtor: InstitutionalEntityId::Being(candidate),
            creditor: InstitutionalEntityId::Being(sponsor.being.clone()),
            institution: sponsor.institution.clone(),
            kind: ObligationKind::Sponsorship,
            status: ObligationStatus::Open,
            weight: ObligationWeight::Significant,
        });
        state
            .claims
            .push(crate::institution_affiliation::AffiliationClaim {
                id: crate::institution_affiliation::ClaimId::new("claim.test.member").unwrap(),
                claimant: member.being.clone(),
                institution: member.institution.clone(),
                claimed_state: AffiliationState::Associate,
                claimed_role: Some(MembershipRole::Associate),
                truth_status: ClaimTruth::True,
                visibility: crate::institution::Visibility::Known,
            });
        state.events.push(InstitutionalEvent {
            id: crate::institution_affiliation::EventId::new("event.test.member").unwrap(),
            kind: InstitutionalEventKind::MembershipRecognized,
            institution: member.institution.clone(),
            subject: member.being.clone(),
            at: 4,
        });
        let output = build_persisted_state_output(&state);
        let restored =
            parse_persisted_state(&output, state.catalog.clone()).expect("state round trip");
        assert_eq!(restored.memberships, state.memberships);
        assert_eq!(restored.sponsorships, state.sponsorships);
        assert_eq!(restored.obligations, state.obligations);
        assert_eq!(restored.claims, state.claims);
        assert_eq!(restored.access_grants, state.access_grants);
        assert_eq!(restored.events, state.events);
        assert_eq!(
            restored.memberships[0].internal_recognition,
            RecognitionLevel::Internal
        );
    }
}
