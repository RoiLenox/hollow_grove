//! Versioned persistence for dynamic institutional state.
//!
//! Canonical catalogs are code fixtures and are rebuilt on load. This artifact
//! persists only the mutable records layered over those fixtures.

use std::io;

use crate::institution::{
    ClearanceLevel, GroupId, InstitutionId, InstitutionalBeingId, InstitutionalEntityId,
    MembershipId, OfficeHolder, OfficeId, RoleId, SiteId, Visibility, ZoneId,
};
use crate::institution_affiliation::*;

pub const INSTITUTIONAL_STATE_ARTIFACT_PATH: &str = "artifacts/institutional_state.txt";
pub const INSTITUTIONAL_STATE_SCHEMA_VERSION: &str = "2";
const LEGACY_INSTITUTIONAL_STATE_SCHEMA_VERSION: &str = "1";

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
    let canonical_holders = crate::world::canonical_institutional_world_state()
        .catalog
        .office_holders;
    for entry in &state.catalog.office_holders {
        if canonical_holders.contains(entry) {
            continue;
        }
        record(
            &mut output,
            "office-holder",
            &[
                entry.office.as_str().into(),
                entry.being.as_str().into(),
                entry.active.to_string(),
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
    if !matches!(
        version,
        Some(INSTITUTIONAL_STATE_SCHEMA_VERSION | LEGACY_INSTITUTIONAL_STATE_SCHEMA_VERSION)
    ) {
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
            "membership" => {
                let membership = migrate_legacy_stonebend_membership(parse_membership(&fields)?);
                let membership = migrate_legacy_glaushouse_membership(membership);
                state
                    .memberships
                    .push(migrate_legacy_sandmanor_membership(membership))
            }
            "sponsorship" => state.sponsorships.push(parse_sponsorship(&fields)?),
            "obligation" => state.obligations.push(parse_obligation(&fields)?),
            "claim" => state.claims.push(parse_claim(&fields)?),
            "access-grant" => {
                state
                    .access_grants
                    .push(migrate_legacy_glaushouse_access(parse_access_grant(
                        &fields,
                    )?))
            }
            "event" => state.events.push(parse_event(&fields)?),
            "office-holder" if version == Some(INSTITUTIONAL_STATE_SCHEMA_VERSION) => {
                let holder = parse_office_holder(&fields)?;
                if !state.catalog.office_holders.contains(&holder) {
                    state.catalog.office_holders.push(holder);
                }
            }
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

/// Legacy Stonebend membership records never manufacture constitutional
/// authority during migration. The former labor-body role becomes ordinary
/// Gerald civic standing. The former Freemason role becomes membership in the
/// ratified Freemason institution. Defunct peer-role subgroups are discarded;
/// the membership record itself and its historical timestamps remain intact.
fn migrate_legacy_stonebend_membership(
    mut membership: InstitutionalMembership,
) -> InstitutionalMembership {
    const LEGACY_LABOR_ROLE: &str = concat!("role.stonebend.prole", "tariat");
    const LEGACY_LABOR_GROUP: &str = concat!("group.stonebend.prole", "tariat");
    const LEGACY_FREEMASON_ROLE: &str = "role.stonebend.freemason";
    const LEGACY_FREEMASON_GROUP: &str = "group.stonebend.freemason";

    match membership.role_id.as_ref().map(RoleId::as_str) {
        Some(LEGACY_LABOR_ROLE) => {
            membership.role_id = Some(role("role.stonebend.gerald").expect("canonical role ID"));
            membership.role = MembershipRole::Associate;
            if membership.subgroup.as_ref().map(GroupId::as_str) == Some(LEGACY_LABOR_GROUP) {
                membership.subgroup = None;
            }
        }
        Some(LEGACY_FREEMASON_ROLE) => {
            membership.institution =
                institution("institution.stonebend.freemason").expect("canonical institution ID");
            membership.role_id =
                Some(role("role.stonebend.freemason-member").expect("canonical Freemason role ID"));
            membership.role = MembershipRole::FullMember;
            if membership.subgroup.as_ref().map(GroupId::as_str) == Some(LEGACY_FREEMASON_GROUP) {
                membership.subgroup = None;
            }
        }
        _ => {}
    }
    membership
}

/// Legacy Glaüshouse role records remain ordinary clinical membership and can
/// never manufacture a constitutional office. Nightingales move into their
/// canonical clinical institution, recovery staff into Glauspitals, and the
/// legacy Persephone label becomes recovery-staff membership rather than an
/// inferred modern Persephone rank: current law requires typed mastery of both
/// Matron and Marshal domains.
fn migrate_legacy_glaushouse_membership(
    mut membership: InstitutionalMembership,
) -> InstitutionalMembership {
    match membership.role_id.as_ref().map(RoleId::as_str) {
        Some("role.glaushouse.nightingale") => {
            membership.institution = crate::world::glaushouse::nightingales_id();
        }
        Some("role.glaushouse.recovery-staff") => {
            membership.institution = crate::world::glaushouse::glauspitals_id();
        }
        Some("role.glaushouse.persephone") => {
            membership.institution = crate::world::glaushouse::glauspitals_id();
            membership.role_id =
                Some(role("role.glaushouse.recovery-staff").expect("canonical recovery role ID"));
            membership.role = MembershipRole::Associate;
        }
        _ => {}
    }
    membership
}

/// Historical Sandman/Sandmen role strings preserve ordinary Sandmanor civic
/// participation but never manufacture the singular Sandman office. The V2
/// Constitution requires a completed Contest of Improvement, Stonebend Title,
/// Flynt recognition, a public learning statement, and a sealed accession.
fn migrate_legacy_sandmanor_membership(
    mut membership: InstitutionalMembership,
) -> InstitutionalMembership {
    if matches!(
        membership.role_id.as_ref().map(RoleId::as_str),
        Some("role.sandmanor.sandman" | "role.sandmanor.sandmen")
    ) {
        membership.institution = crate::world::sandmanor::proof_civilization_id();
        membership.role_id = None;
        membership.role = MembershipRole::Associate;
        membership.affiliation_state = AffiliationState::Associate;
    }
    membership
}

fn migrate_legacy_glaushouse_access(mut grant: AccessGrant) -> AccessGrant {
    if grant.institution.as_str() == "institution.glaushouse.medical-civilization"
        && grant
            .site
            .as_ref()
            .is_some_and(|site| site.as_str() == "site.glaushouse.central-medical-district")
    {
        grant.institution = crate::world::glaushouse::glauspitals_id();
    }
    grant
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

fn parse_office_holder(values: &[&str]) -> io::Result<OfficeHolder> {
    expect(values, 3, "office-holder")?;
    Ok(OfficeHolder {
        office: office(values[1])?,
        being: being(values[2])?,
        active: boolean(values[3])?,
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
    use crate::world::{canonical_institutional_world_state, institutional_access_fixture};

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
            role_id: Some(crate::world::flynt::gallows_member_role_id()),
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

    #[test]
    fn legacy_stonebend_labor_membership_migrates_to_gerald_standing() {
        let role = concat!("role.stonebend.prole", "tariat");
        let group = concat!("group.stonebend.prole", "tariat");
        let input = format!(
            "schema_version:1\nmembership\tmembership.stonebend.legacy-labor\tbeing.stonebend.legacy-labor\tinstitution.stonebend.constitution\t{role}\tAssociate\tAssociate\tNone\t-\t{group}\t1\t-\t-\tPublic\tInternal\n"
        );
        let state = canonical_institutional_world_state();
        let restored = parse_persisted_state(&input, state.catalog).unwrap();
        let membership = &restored.memberships[0];
        assert_eq!(
            membership.role_id.as_ref().unwrap().as_str(),
            "role.stonebend.gerald"
        );
        assert_eq!(
            membership.institution.as_str(),
            "institution.stonebend.constitution"
        );
        assert!(membership.subgroup.is_none());
    }

    #[test]
    fn legacy_freemason_membership_migrates_without_granting_high_office() {
        let input = "schema_version:1\n\
            membership\tmembership.stonebend.legacy-freemason\tbeing.stonebend.legacy-freemason\tinstitution.stonebend.constitution\trole.stonebend.freemason\tAssociate\tAssociate\tNone\t-\tgroup.stonebend.freemason\t1\t-\t-\tPublic\tInternal\n";
        let state = canonical_institutional_world_state();
        let restored = parse_persisted_state(input, state.catalog).unwrap();
        let membership = &restored.memberships[0];
        assert_eq!(
            membership.role_id.as_ref().unwrap().as_str(),
            "role.stonebend.freemason-member"
        );
        assert_eq!(
            membership.institution.as_str(),
            "institution.stonebend.freemason"
        );
        assert!(membership.subgroup.is_none());
        assert_ne!(membership.role, MembershipRole::Leader);
    }

    #[test]
    fn legacy_nightingale_membership_moves_to_the_clinical_institution() {
        let input = "schema_version:1\n\
            membership\tmembership.glaushouse.legacy-nightingale\tbeing.glaushouse.legacy-nightingale\tinstitution.glaushouse.medical-civilization\trole.glaushouse.nightingale\tFullMember\tInitiated\tNone\t-\t-\t1\t1\t-\tKnown\tInternal\n";
        let state = canonical_institutional_world_state();
        let restored = parse_persisted_state(input, state.catalog).unwrap();
        let membership = &restored.memberships[0];
        assert_eq!(
            membership.institution.as_str(),
            "institution.glaushouse.nightingales"
        );
        assert_eq!(
            membership.role_id.as_ref().unwrap().as_str(),
            "role.glaushouse.nightingale"
        );
    }

    #[test]
    fn legacy_persephone_role_preserves_service_but_never_infers_modern_rank() {
        let input = "schema_version:1\n\
            membership\tmembership.glaushouse.legacy-persephone\tbeing.glaushouse.legacy-persephone\tinstitution.glaushouse.medical-civilization\trole.glaushouse.persephone\tLeader\tSenior\tNone\t-\t-\t1\t1\t-\tKnown\tInternal\n";
        let state = canonical_institutional_world_state();
        let restored = parse_persisted_state(input, state.catalog).unwrap();
        let membership = &restored.memberships[0];
        assert_eq!(
            membership.institution.as_str(),
            "institution.glaushouse.glauspitals"
        );
        assert_eq!(
            membership.role_id.as_ref().unwrap().as_str(),
            "role.glaushouse.recovery-staff"
        );
        assert_eq!(membership.role, MembershipRole::Associate);
        assert_ne!(
            membership.role_id.as_ref().unwrap(),
            &crate::world::glaushouse::persephone_rank_id()
        );
    }

    #[test]
    fn legacy_medical_district_access_moves_to_glauspitals() {
        let input = "schema_version:1\n\
            access-grant\taccess-grant.glaushouse.legacy\tbeing.glaushouse.legacy\tinstitution.glaushouse.medical-civilization\tsite.glaushouse.central-medical-district\tzone.glaushouse.medical-district.recovery-chambers\t-\ttrue\n";
        let state = canonical_institutional_world_state();
        let restored = parse_persisted_state(input, state.catalog).unwrap();
        assert_eq!(
            restored.access_grants[0].institution.as_str(),
            "institution.glaushouse.glauspitals"
        );
    }

    #[test]
    fn legacy_sandman_role_preserves_membership_but_never_infers_office() {
        let input = "schema_version:1\n\
            membership\tmembership.sandmanor.legacy-sandman\tbeing.sandmanor.legacy-sandman\tinstitution.sandmanor.sandmen\trole.sandmanor.sandman\tLeader\tSenior\tNone\t-\t-\t1\t1\t-\tPublic\tInternal\n";
        let state = canonical_institutional_world_state();
        let restored = parse_persisted_state(input, state.catalog).unwrap();
        let membership = &restored.memberships[0];
        assert_eq!(
            membership.institution,
            crate::world::sandmanor::proof_civilization_id()
        );
        assert!(membership.role_id.is_none());
        assert_eq!(membership.role, MembershipRole::Associate);
        assert_eq!(membership.affiliation_state, AffiliationState::Associate);
        assert!(!restored.catalog.office_holders.iter().any(|holder| {
            holder.office == crate::world::sandmanor::sandman_office_id()
                && holder.being.as_str() == "being.sandmanor.legacy-sandman"
        }));
        assert!(
            crate::world::hueman_faculties::migrate_legacy_faculty_manifestations(
                &restored.memberships,
            )
            .is_empty(),
            "legacy membership must not infer faculty, mastery, proof, credential, or office"
        );
    }

    #[test]
    fn schema_two_persists_live_office_holders_without_duplicating_canonical_tross() {
        let input = "schema_version:2\n\
            office-holder\toffice.stonebend.hypergiant\tbeing.stonebend.current-hypergiant\ttrue\n\
            office-holder\toffice.sandmanor.sandman\tbeing.sandmanor.current-sandman\ttrue\n\
            office-holder\toffice.glaushouse.prima-donna\tbeing.glaushouse.current-prima-donna\ttrue\n";
        let canonical = canonical_institutional_world_state();
        let restored = parse_persisted_state(input, canonical.catalog).unwrap();
        assert_eq!(restored.catalog.office_holders.len(), 4);
        let output = build_persisted_state_output(&restored);
        assert!(output.starts_with("schema_version:2\n"));
        assert_eq!(output.matches("office-holder\t").count(), 3);
        assert!(!output.contains("being.flynt.tross"));

        let replayed =
            parse_persisted_state(&output, canonical_institutional_world_state().catalog).unwrap();
        assert_eq!(
            replayed.catalog.office_holders,
            restored.catalog.office_holders
        );
    }

    #[test]
    fn schema_one_migration_never_infers_an_office_holder() {
        let input = "schema_version:1\n\
            membership\tmembership.sandmanor.legacy-sandman-two\tbeing.sandmanor.legacy-sandman-two\tinstitution.sandmanor.sandmen\trole.sandmanor.sandman\tLeader\tSenior\tNone\t-\t-\t1\t1\t-\tKnown\tInternal\n";
        let restored =
            parse_persisted_state(input, canonical_institutional_world_state().catalog).unwrap();
        assert_eq!(restored.catalog.office_holders.len(), 1);
        assert_eq!(
            restored.catalog.office_holders[0].office,
            crate::world::flynt::tross_office_id()
        );
    }
}
