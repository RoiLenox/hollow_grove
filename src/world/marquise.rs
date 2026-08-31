//! Canonical Flynt lore for the maritime Mystery Men tradition.
//!
//! This module deliberately models The Marquise as an extension whose parent
//! is Mystery Men. It is not a sibling institution, a Navy, or an independent
//! military. `The Marquise` is also the exact title of its singular leader.

use std::fmt::Write;

pub const MARQUISE_NAME: &str = "The Marquise";
pub const MARQUISE_HOUSE: &str = "Flynt";
pub const MARQUISE_SOVEREIGN: &str = "The Tross";
pub const MARQUISE_PARENT_TRADITION: &str = "Mystery Men";
pub const MARQUISE_RELATION_TO_PARENT: &str = "Maritime extension";
pub const MARQUISE_PRACTICE: &str = "The Push";
pub const MARQUISE_PRIMARY_SYNTHESIS: &str = "Merman";
pub const MARQUISE_COMPLEMENTARY_SYNTHESIS: &str = "Mermaid";
pub const MARQUISE_CITIZEN_ORIGIN: &str = "Warden";
pub const MARQUISE_MEMBER_TITLE: &str = "Privateer";
pub const MARQUISE_APEX_RANK: &str = "Corsair";
pub const MARQUISE_ENVIRONMENT: &str = "Maritime";
pub const MARQUISE_AUTHORITY_MODEL: &str = "Sovereign-authorized independent operations";
pub const CORSAIR_DESIGN_NOTE: &str = "A Corsair pilots systems.";
pub const MARQUISE_PUSH_PHRASE: &str = "At sea, The Push becomes navigation.";
pub const FLYNT_CIVIC_IDENTITY: &str = "Wardens";
pub const STONEBEND_CIVIC_IDENTITY: &str = "Geralds";
pub const SANDMANOR_CIVIC_IDENTITY: &str = "Sandmen";

pub const MARQUISE_OVERVIEW: &str = "The Marquise are the maritime extension of the Mystery Men of Flynt. Operating under the authority of The Tross, they carry the Mystery Men tradition onto lakes, rivers, coasts, ports, vessels, and open water. Flynt Wardens who enter this tradition become Privateers, while Corsair represents its highest attained rank. The organization and the individual who leads it are both called The Marquise. Their tradition is closely associated with Merman and Mermaid synthesis and with the maritime expression of The Push: navigation, propulsion, current, force, communication, and control of complex systems.";

pub const MARQUISE_CANON_SUMMARY: &str = "The Marquise are the maritime extension of the Mystery Men of Flynt. They answer ultimately to The Tross. Flynt Wardens may become Privateers and advance to Corsair, the highest attained rank. The organization and its leader are both called The Marquise. Their tradition centers on Merman and Mermaid synthesis and applies The Push to navigation, maritime operations, technology, communications, and complex systems.";

const OPERATIONAL_ENVIRONMENTS: &[&str] = &[
    "lakes",
    "rivers",
    "waterways",
    "harbors",
    "ports",
    "coastlines",
    "open water",
    "vessels",
    "maritime infrastructure",
];

const OPERATIONS: &[&str] = &[
    "Navigation",
    "Search and rescue",
    "Water patrol",
    "Vessel operation",
    "Exploration",
    "Salvage",
    "Recovery",
    "Transport",
    "Escort",
    "Maritime security",
    "Port operations",
    "Communications",
    "Reconnaissance",
    "Disaster response",
    "Technical operations",
    "Remote systems",
    "Maritime logistics",
];

const PUSH_MANIFESTATIONS: &[&str] = &[
    "propulsion",
    "direction",
    "momentum",
    "force",
    "pressure",
    "current",
    "steering",
    "displacement",
    "navigation",
    "remote control",
];

const CORSAIR_MARITIME_DOMAINS: &[&str] = &[
    "navigation",
    "vessels",
    "command at sea",
    "water routes",
    "currents",
    "long-distance operations",
    "expeditionary decision-making",
    "maritime systems",
];

const CORSAIR_COMPUTATIONAL_DOMAINS: &[&str] = &[
    "computers",
    "networks",
    "digital systems",
    "navigation computers",
    "remote devices",
    "communications",
    "autonomous systems",
    "distributed systems",
    "system control",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarquiseMeaning {
    Organization,
    LeaderTitle,
}

impl MarquiseMeaning {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Organization => "Organization",
            Self::LeaderTitle => "Leadership title",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarquiseStatusCategory {
    Citizen,
    PractitionerOperationalMember,
    ApexRank,
    LeadershipOffice,
}

impl MarquiseStatusCategory {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Citizen => "Citizen",
            Self::PractitionerOperationalMember => "Practitioner / Operational Member",
            Self::ApexRank => "Apex Rank",
            Self::LeadershipOffice => "Leadership Office",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarquiseStatusRecord {
    pub name: String,
    pub category: MarquiseStatusCategory,
    pub house: Option<String>,
    pub organization: Option<String>,
    pub parent_tradition: Option<String>,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub reports_to: Option<String>,
    pub commands: Vec<String>,
    pub description: String,
    pub design_note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorsairDomainRecord {
    pub maritime: Vec<String>,
    pub computational: Vec<String>,
    pub design_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseCivicIdentityRecord {
    pub people: String,
    pub house: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarquiseInstitutionRecord {
    pub name: String,
    pub house: String,
    pub sovereign: String,
    pub parent_tradition: String,
    pub relation_to_parent: String,
    pub leader_title: String,
    pub citizen_origin: String,
    pub member_title: String,
    pub apex_rank: String,
    pub practice: String,
    pub primary_synthesis: String,
    pub complementary_synthesis: String,
    pub environment: String,
    pub authority_model: String,
    pub meanings: Vec<MarquiseMeaning>,
    pub house_civic_identities: Vec<HouseCivicIdentityRecord>,
    pub command_chain: Vec<String>,
    pub progression: Vec<MarquiseStatusRecord>,
    pub operational_environments: Vec<String>,
    pub operations: Vec<String>,
    pub push_manifestations: Vec<String>,
    pub corsair_domains: CorsairDomainRecord,
    pub overview: String,
    pub canon_summary: String,
}

#[must_use]
pub fn canonical_marquise_lore() -> MarquiseInstitutionRecord {
    let name = MARQUISE_NAME.to_owned();
    let parent = MARQUISE_PARENT_TRADITION.to_owned();
    MarquiseInstitutionRecord {
        name: name.clone(),
        house: MARQUISE_HOUSE.to_owned(),
        sovereign: MARQUISE_SOVEREIGN.to_owned(),
        parent_tradition: parent.clone(),
        relation_to_parent: MARQUISE_RELATION_TO_PARENT.to_owned(),
        leader_title: name.clone(),
        citizen_origin: MARQUISE_CITIZEN_ORIGIN.to_owned(),
        member_title: MARQUISE_MEMBER_TITLE.to_owned(),
        apex_rank: MARQUISE_APEX_RANK.to_owned(),
        practice: MARQUISE_PRACTICE.to_owned(),
        primary_synthesis: MARQUISE_PRIMARY_SYNTHESIS.to_owned(),
        complementary_synthesis: MARQUISE_COMPLEMENTARY_SYNTHESIS.to_owned(),
        environment: MARQUISE_ENVIRONMENT.to_owned(),
        authority_model: MARQUISE_AUTHORITY_MODEL.to_owned(),
        meanings: vec![MarquiseMeaning::Organization, MarquiseMeaning::LeaderTitle],
        house_civic_identities: vec![
            HouseCivicIdentityRecord {
                people: FLYNT_CIVIC_IDENTITY.to_owned(),
                house: MARQUISE_HOUSE.to_owned(),
            },
            HouseCivicIdentityRecord {
                people: STONEBEND_CIVIC_IDENTITY.to_owned(),
                house: "Stonebend".to_owned(),
            },
            HouseCivicIdentityRecord {
                people: SANDMANOR_CIVIC_IDENTITY.to_owned(),
                house: "Sandmanor".to_owned(),
            },
        ],
        command_chain: vec![
            MARQUISE_SOVEREIGN.to_owned(),
            parent.clone(),
            name.clone(),
        ],
        progression: vec![
            MarquiseStatusRecord {
                name: MARQUISE_CITIZEN_ORIGIN.to_owned(),
                category: MarquiseStatusCategory::Citizen,
                house: Some(MARQUISE_HOUSE.to_owned()),
                organization: None,
                parent_tradition: None,
                previous: None,
                next: Some(MARQUISE_MEMBER_TITLE.to_owned()),
                reports_to: None,
                commands: vec![],
                description: "Primary civic identity of Flynt.".to_owned(),
                design_note: None,
            },
            MarquiseStatusRecord {
                name: MARQUISE_MEMBER_TITLE.to_owned(),
                category: MarquiseStatusCategory::PractitionerOperationalMember,
                house: None,
                organization: Some(name.clone()),
                parent_tradition: Some(parent.clone()),
                previous: Some(MARQUISE_CITIZEN_ORIGIN.to_owned()),
                next: Some(MARQUISE_APEX_RANK.to_owned()),
                reports_to: None,
                commands: vec![],
                description: "A Warden operating as part of the maritime extension of the Mystery Men.".to_owned(),
                design_note: Some("Licensed independence: an independent sailor operating legitimately under sovereign authority.".to_owned()),
            },
            MarquiseStatusRecord {
                name: MARQUISE_APEX_RANK.to_owned(),
                category: MarquiseStatusCategory::ApexRank,
                house: None,
                organization: Some(name.clone()),
                parent_tradition: Some(parent.clone()),
                previous: Some(MARQUISE_MEMBER_TITLE.to_owned()),
                next: Some(name.clone()),
                reports_to: None,
                commands: vec![],
                description: "Highest attained Privateer status, associated with mastery of navigation, independent operations, vessels, computers, networks, and complex systems.".to_owned(),
                design_note: Some(CORSAIR_DESIGN_NOTE.to_owned()),
            },
            MarquiseStatusRecord {
                name: name.clone(),
                category: MarquiseStatusCategory::LeadershipOffice,
                house: None,
                organization: Some(name.clone()),
                parent_tradition: Some(parent),
                previous: Some(MARQUISE_APEX_RANK.to_owned()),
                next: None,
                reports_to: Some(MARQUISE_SOVEREIGN.to_owned()),
                commands: vec!["Privateers".to_owned(), "Corsairs".to_owned()],
                description: "Leader of the maritime extension of the Mystery Men.".to_owned(),
                design_note: Some("An occupied leadership office, not another earned rank.".to_owned()),
            },
        ],
        operational_environments: strings(OPERATIONAL_ENVIRONMENTS),
        operations: strings(OPERATIONS),
        push_manifestations: strings(PUSH_MANIFESTATIONS),
        corsair_domains: CorsairDomainRecord {
            maritime: strings(CORSAIR_MARITIME_DOMAINS),
            computational: strings(CORSAIR_COMPUTATIONAL_DOMAINS),
            design_note: CORSAIR_DESIGN_NOTE.to_owned(),
        },
        overview: MARQUISE_OVERVIEW.to_owned(),
        canon_summary: MARQUISE_CANON_SUMMARY.to_owned(),
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarquiseLoreDiagnosticCode {
    Identity,
    ParentRelationship,
    Authority,
    Progression,
    LeadershipOffice,
    Synthesis,
    Push,
    CorsairDomain,
    Operations,
    CanonText,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarquiseLoreDiagnostic {
    pub code: MarquiseLoreDiagnosticCode,
    pub message: String,
}

#[must_use]
pub fn validate_marquise_lore(input: &MarquiseInstitutionRecord) -> Vec<MarquiseLoreDiagnostic> {
    let canonical = canonical_marquise_lore();
    let mut diagnostics = Vec::new();

    check(
        input.name == MARQUISE_NAME
            && input.house == MARQUISE_HOUSE
            && input.leader_title == MARQUISE_NAME
            && input.meanings == canonical.meanings
            && input.house_civic_identities == canonical.house_civic_identities,
        MarquiseLoreDiagnosticCode::Identity,
        "The Marquise must retain both meanings and the civic identities Wardens = Flynt, Geralds = Stonebend, and Sandmen = Sandmanor.",
        &mut diagnostics,
    );
    check(
        input.parent_tradition == MARQUISE_PARENT_TRADITION
            && input.relation_to_parent == MARQUISE_RELATION_TO_PARENT,
        MarquiseLoreDiagnosticCode::ParentRelationship,
        "The Marquise must remain the maritime extension of Mystery Men, never its sibling.",
        &mut diagnostics,
    );
    check(
        input.sovereign == MARQUISE_SOVEREIGN
            && input.authority_model == MARQUISE_AUTHORITY_MODEL
            && input.command_chain == canonical.command_chain,
        MarquiseLoreDiagnosticCode::Authority,
        "The command chain must be The Tross -> Mystery Men -> The Marquise.",
        &mut diagnostics,
    );
    check(
        input.citizen_origin == MARQUISE_CITIZEN_ORIGIN
            && input.member_title == MARQUISE_MEMBER_TITLE
            && input.apex_rank == MARQUISE_APEX_RANK
            && input.progression == canonical.progression,
        MarquiseLoreDiagnosticCode::Progression,
        "The exact progression is Warden -> Privateer -> Corsair -> The Marquise.",
        &mut diagnostics,
    );
    check(
        input.progression.last().is_some_and(|status| {
            status.name == MARQUISE_NAME
                && status.category == MarquiseStatusCategory::LeadershipOffice
                && status.reports_to.as_deref() == Some(MARQUISE_SOVEREIGN)
        }),
        MarquiseLoreDiagnosticCode::LeadershipOffice,
        "The Marquise must be a leadership office after Corsair, not a fourth earned rank.",
        &mut diagnostics,
    );
    check(
        input.primary_synthesis == MARQUISE_PRIMARY_SYNTHESIS
            && input.complementary_synthesis == MARQUISE_COMPLEMENTARY_SYNTHESIS,
        MarquiseLoreDiagnosticCode::Synthesis,
        "Merman and Mermaid synthesis must remain complementary, not adversarial.",
        &mut diagnostics,
    );
    check(
        input.practice == MARQUISE_PRACTICE
            && input.push_manifestations == canonical.push_manifestations,
        MarquiseLoreDiagnosticCode::Push,
        "The Push must retain its maritime expressions of propulsion, navigation, force, current, and remote control.",
        &mut diagnostics,
    );
    check(
        input.corsair_domains == canonical.corsair_domains,
        MarquiseLoreDiagnosticCode::CorsairDomain,
        "Corsair must preserve both maritime and computational command of complex systems.",
        &mut diagnostics,
    );
    check(
        input.environment == MARQUISE_ENVIRONMENT
            && input.operational_environments == canonical.operational_environments
            && input.operations == canonical.operations,
        MarquiseLoreDiagnosticCode::Operations,
        "The Marquise operational record must remain maritime and retain its public-service and independent capabilities.",
        &mut diagnostics,
    );
    check(
        input.overview == MARQUISE_OVERVIEW && input.canon_summary == MARQUISE_CANON_SUMMARY,
        MarquiseLoreDiagnosticCode::CanonText,
        "The canonical overview and summary must remain exact.",
        &mut diagnostics,
    );

    diagnostics
}

fn check(
    condition: bool,
    code: MarquiseLoreDiagnosticCode,
    message: &str,
    diagnostics: &mut Vec<MarquiseLoreDiagnostic>,
) {
    if !condition {
        diagnostics.push(MarquiseLoreDiagnostic {
            code,
            message: message.to_owned(),
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarquisePanel {
    Overview,
    Hierarchy,
    Warden,
    Privateer,
    Corsair,
    TheMarquise,
    MysteryMen,
    MermanMermaid,
    ThePush,
    Operations,
}

impl MarquisePanel {
    #[must_use]
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "overview" => Some(Self::Overview),
            "hierarchy" | "command" => Some(Self::Hierarchy),
            "warden" => Some(Self::Warden),
            "privateer" => Some(Self::Privateer),
            "corsair" => Some(Self::Corsair),
            "the-marquise" | "leader" => Some(Self::TheMarquise),
            "mystery-men" => Some(Self::MysteryMen),
            "merman-mermaid" | "synthesis" => Some(Self::MermanMermaid),
            "the-push" | "push" => Some(Self::ThePush),
            "operations" => Some(Self::Operations),
            _ => None,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Overview => "OVERVIEW",
            Self::Hierarchy => "HIERARCHY",
            Self::Warden => "WARDEN",
            Self::Privateer => "PRIVATEER",
            Self::Corsair => "CORSAIR",
            Self::TheMarquise => "THE MARQUISE",
            Self::MysteryMen => "MYSTERY MEN",
            Self::MermanMermaid => "MERMAN / MERMAID",
            Self::ThePush => "THE PUSH",
            Self::Operations => "OPERATIONS",
        }
    }
}

#[must_use]
pub fn build_marquise_panel_output(panel: MarquisePanel) -> String {
    let lore = canonical_marquise_lore();
    match panel {
        MarquisePanel::Overview => render_overview(&lore),
        MarquisePanel::Hierarchy => render_hierarchy(),
        MarquisePanel::Warden => render_status(&lore.progression[0]),
        MarquisePanel::Privateer => render_privateer(&lore),
        MarquisePanel::Corsair => render_corsair(&lore),
        MarquisePanel::TheMarquise => render_status(&lore.progression[3]),
        MarquisePanel::MysteryMen => render_mystery_men(),
        MarquisePanel::MermanMermaid => render_synthesis(),
        MarquisePanel::ThePush => render_push(&lore),
        MarquisePanel::Operations => render_operations(&lore),
    }
}

#[must_use]
pub fn build_marquise_validation_report() -> String {
    let diagnostics = validate_marquise_lore(&canonical_marquise_lore());
    let mut output = String::from("THE MARQUISE CANON VALIDATION\n\n");
    if diagnostics.is_empty() {
        output.push_str("status: pass\n");
        output.push_str("parent: Mystery Men\n");
        output.push_str("extension_type: Maritime\n");
        output.push_str("command: The Tross → Mystery Men → The Marquise\n");
        output.push_str("progression: Warden → Privateer → Corsair → The Marquise\n");
        output.push_str("office distinction: pass\n");
        output.push_str("complementary synthesis: Merman ↔ Mermaid\n");
        output.push_str("Corsair maritime/system dual domain: pass\n");
    } else {
        output.push_str("status: fail\n");
        for diagnostic in diagnostics {
            let _ = writeln!(output, "- {:?}: {}", diagnostic.code, diagnostic.message);
        }
    }
    output
}

fn render_overview(lore: &MarquiseInstitutionRecord) -> String {
    format!(
        "----------------------------------------\n\
         THE MARQUISE\n\
         ----------------------------------------\n\n\
         HOUSE\n{}\n\n\
         SOVEREIGN\n{}\n\n\
         PARENT TRADITION\n{}\n\n\
         FUNCTION\n{}\n\n\
         PRACTICE\n{}\n\n\
         SYNTHESIS\n{} ↔ {}\n\n\
         PROGRESSION\n\n\
         Warden                    [Civic identity]\n\
           ↓\n\
         Privateer                 [Practitioner / operational identity]\n\
           ↓\n\
         Corsair                   [Apex attained rank]\n\
           ↓\n\
         === LEADERSHIP OFFICE ===\n\
         The Marquise\n\n\
         COMMAND\n\n\
         The Tross\n\
           ↓\n\
         Mystery Men\n\
           ↓\n\
         The Marquise\n\n\
         DOMAIN\n\n\
         Navigation\n\
         Rescue\n\
         Patrol\n\
         Salvage\n\
         Exploration\n\
         Transport\n\
         Communications\n\
         Maritime Operations\n\n\
         LORE\n{}\n\n\
         ----------------------------------------",
        lore.house,
        lore.sovereign,
        lore.parent_tradition,
        lore.relation_to_parent,
        lore.practice,
        lore.primary_synthesis,
        lore.complementary_synthesis,
        lore.overview,
    )
}

fn render_hierarchy() -> String {
    String::from(
        "HIERARCHY\n\n\
         FLYNT\n\
         |\n\
         |-- Sovereign\n\
         |   `-- THE TROSS\n\
         |\n\
         |-- Citizens\n\
         |   `-- WARDENS\n\
         |\n\
         `-- MYSTERY MEN\n\
             `-- THE MARQUISE  [Maritime extension]\n\
                 |-- Leader: THE MARQUISE  [Leadership office]\n\
                 |-- CORSAIRS               [Apex attained rank]\n\
                 `-- PRIVATEERS             [Operational members]\n\n\
         COMMAND\n\
         THE TROSS → MYSTERY MEN → THE MARQUISE\n\n\
         Wardens are the broader Flynt citizenry and sit outside the active service hierarchy.",
    )
}

fn render_status(status: &MarquiseStatusRecord) -> String {
    let mut output = format!(
        "{}\n\nCATEGORY\n{}\n",
        status.name.to_uppercase(),
        status.category.label()
    );
    if let Some(house) = &status.house {
        let _ = write!(output, "\nHOUSE\n{house}\n");
    }
    if let Some(organization) = &status.organization {
        let _ = write!(output, "\nORGANIZATION\n{organization}\n");
    }
    if let Some(parent) = &status.parent_tradition {
        let _ = write!(output, "\nPARENT TRADITION\n{parent}\n");
    }
    if let Some(previous) = &status.previous {
        let _ = write!(output, "\nPREVIOUS\n{previous}\n");
    }
    if let Some(next) = &status.next {
        let _ = write!(output, "\nNEXT\n{next}\n");
    }
    if let Some(reports_to) = &status.reports_to {
        let _ = write!(output, "\nREPORTS TO\n{reports_to}\n");
    }
    if !status.commands.is_empty() {
        let _ = write!(output, "\nCOMMANDS\n{}\n", status.commands.join(" and "));
    }
    let _ = write!(output, "\nDESCRIPTION\n{}\n", status.description);
    if let Some(note) = &status.design_note {
        let _ = write!(output, "\nDESIGN NOTE\n{note}\n");
    }
    if status.name == MARQUISE_CITIZEN_ORIGIN {
        output.push_str(
            "\nHOUSE CIVIC IDENTITIES\n\
             Wardens = Flynt\n\
             Geralds = Stonebend\n\
             Sandmen = Sandmanor\n",
        );
    }
    output
}

fn render_privateer(lore: &MarquiseInstitutionRecord) -> String {
    let mut output = render_status(&lore.progression[1]);
    output.push_str(
        "\nCHARACTER\n\
         Capable / Practical / Rough around the edges / Autonomous / Mobile / Resourceful / Technically competent\n\n\
         AUTHORITY\n\
         Licensed independence under The Tross, through Mystery Men and The Marquise. Privateers may resemble buccaneers without becoming ordinary pirates.\n",
    );
    output
}

fn render_corsair(lore: &MarquiseInstitutionRecord) -> String {
    let mut output = render_status(&lore.progression[2]);
    output.push_str("\nMARITIME DOMAIN\n");
    render_list(&mut output, &lore.corsair_domains.maritime);
    output.push_str("\nCOMPUTATIONAL DOMAIN\n");
    render_list(&mut output, &lore.corsair_domains.computational);
    output.push_str(
        "\nA Corsair is neither merely a senior sailor nor merely a hacker. Corsair means navigation, command, and control of complex systems. A vessel is one system; a computer or network may be another.\n",
    );
    output
}

fn render_mystery_men() -> String {
    String::from(
        "MYSTERY MEN\n\n\
         HOUSE\nFlynt\n\n\
         AUTHORITY\nThe Tross\n\n\
         CHILD / EXTENSION\nThe Marquise\n\n\
         EXTENSION TYPE\nMaritime\n\n\
         The Mystery Men are the parent tradition. The Marquise inherit their philosophical and cultural identity from Mystery Men; they are not a sibling, rival, or separate House institution.\n",
    )
}

fn render_synthesis() -> String {
    String::from(
        "MERMAN / MERMAID\n\n\
         Merman ↔ Mermaid\n\n\
         RELATIONSHIP\nComplementary, not inherently adversarial\n\n\
         The Marquise may contain both Mermen and Mermaids. Both belong to the same maritime synthesis ecology. Merman synthesis is primary and Mermaid synthesis is complementary; neither is encoded as the other's enemy.\n",
    )
}

fn render_push(lore: &MarquiseInstitutionRecord) -> String {
    let mut output =
        String::from("THE PUSH\n\nAt sea, The Push becomes navigation.\n\nMARITIME EXPRESSIONS\n");
    render_list(&mut output, &lore.push_manifestations);
    output.push_str(
        "\nThe Push is Flynt's broader philosophical principle, not merely water magic. This maritime expression is descriptive lore rather than a closed mathematical rule.\n",
    );
    output
}

fn render_operations(lore: &MarquiseInstitutionRecord) -> String {
    let mut output = String::from("OPERATIONS\n\nENVIRONMENT\nMaritime\n\nOPERATING RANGE\n");
    render_list(&mut output, &lore.operational_environments);
    output.push_str("\nCAPABILITIES\n");
    render_list(&mut output, &lore.operations);
    output.push_str(
        "\nThe Marquise combine formal public service with irregular independent operations: privateers, rescue crews, merchant sailors, expeditionary crews, and technically skilled sailors acting under sovereign authority.\n",
    );
    output
}

fn render_list(output: &mut String, values: &[String]) {
    for value in values {
        let _ = writeln!(output, "- {value}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_record_enforces_parent_command_and_exact_progression() {
        let lore = canonical_marquise_lore();
        assert!(validate_marquise_lore(&lore).is_empty());
        assert_eq!(lore.parent_tradition, "Mystery Men");
        assert_eq!(lore.relation_to_parent, "Maritime extension");
        assert_eq!(
            lore.house_civic_identities
                .iter()
                .map(|identity| (identity.people.as_str(), identity.house.as_str()))
                .collect::<Vec<_>>(),
            [
                ("Wardens", "Flynt"),
                ("Geralds", "Stonebend"),
                ("Sandmen", "Sandmanor"),
            ]
        );
        assert_eq!(
            lore.command_chain,
            ["The Tross", "Mystery Men", "The Marquise"]
        );
        assert_eq!(
            lore.progression
                .iter()
                .map(|status| status.name.as_str())
                .collect::<Vec<_>>(),
            ["Warden", "Privateer", "Corsair", "The Marquise"]
        );
    }

    #[test]
    fn leadership_office_is_not_an_earned_rank() {
        let lore = canonical_marquise_lore();
        assert_eq!(
            lore.progression[2].category,
            MarquiseStatusCategory::ApexRank
        );
        assert_eq!(
            lore.progression[3].category,
            MarquiseStatusCategory::LeadershipOffice
        );
        assert_eq!(lore.name, lore.leader_title);
        assert_eq!(
            lore.meanings,
            [MarquiseMeaning::Organization, MarquiseMeaning::LeaderTitle]
        );
    }

    #[test]
    fn corsair_preserves_maritime_and_computational_mastery() {
        let lore = canonical_marquise_lore();
        assert!(lore.corsair_domains.maritime.contains(&"vessels".into()));
        assert!(
            lore.corsair_domains
                .computational
                .contains(&"networks".into())
        );
        assert_eq!(lore.corsair_domains.design_note, CORSAIR_DESIGN_NOTE);
    }

    #[test]
    fn renderer_keeps_wardens_civic_and_syntheses_complementary() {
        let overview = build_marquise_panel_output(MarquisePanel::Overview);
        let hierarchy = build_marquise_panel_output(MarquisePanel::Hierarchy);
        let synthesis = build_marquise_panel_output(MarquisePanel::MermanMermaid);
        assert!(overview.contains("Warden                    [Civic identity]"));
        assert!(overview.contains("=== LEADERSHIP OFFICE ==="));
        assert!(hierarchy.contains("`-- MYSTERY MEN"));
        assert!(hierarchy.contains("`-- THE MARQUISE  [Maritime extension]"));
        assert!(synthesis.contains("Merman ↔ Mermaid"));
        assert!(!synthesis.contains(concat!("Merman", " vs ", "Mermaid")));
    }

    #[test]
    fn validator_rejects_a_sibling_reinterpretation() {
        let mut lore = canonical_marquise_lore();
        lore.relation_to_parent = "Sibling organization".into();
        let diagnostics = validate_marquise_lore(&lore);
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == MarquiseLoreDiagnosticCode::ParentRelationship
        }));
    }
}
