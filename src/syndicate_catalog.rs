//! Nonexecuting title and source catalog for the approved Syndicate design.
//!
//! This module records names and provenance only. It grants no membership,
//! credential, Agency, Form, office, authority, or Incantation capability.

/// Version of the creator-approved source represented by this catalog.
pub const SYNDICATE_CATALOG_VERSION: &str = "revc-syndicate-titles-v1";

/// Stable catalog identity and provenance for one rank title.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RankTitle {
    pub id: &'static str,
    pub discipline_id: &'static str,
    pub title: &'static str,
    /// Display/order position only; this is not a credential or authority level.
    pub rung: u8,
    pub source_ref: &'static str,
    pub catalog_version: &'static str,
}

const REV_C_SOURCE: &str =
    "HOLLOW_GROVE_REV_C.md §4.4; HOLLOW_GROVE_MASTER_CANON_WORKING_REV100_TUI_HANDOFF.md §6.5";

/// The nine approved titles, in discipline and presentation order.
pub const RANK_TITLES: [RankTitle; 9] = [
    rank(
        "syndicate.word.rung_1.warlock",
        "syndicate.word",
        "Warlock",
        1,
    ),
    rank(
        "syndicate.word.rung_2.wizard",
        "syndicate.word",
        "Wizard",
        2,
    ),
    rank(
        "syndicate.word.rung_3.sorcerer",
        "syndicate.word",
        "Sorcerer",
        3,
    ),
    rank(
        "syndicate.work.rung_1.wiccan",
        "syndicate.work",
        "Wiccan",
        1,
    ),
    rank("syndicate.work.rung_2.witch", "syndicate.work", "Witch", 2),
    rank(
        "syndicate.work.rung_3.sorceress",
        "syndicate.work",
        "Sorceress",
        3,
    ),
    rank(
        "syndicate.performance.rung_1.mage",
        "syndicate.performance",
        "Mage",
        1,
    ),
    rank(
        "syndicate.performance.rung_2.magician",
        "syndicate.performance",
        "Magician",
        2,
    ),
    rank(
        "syndicate.performance.rung_3.mime",
        "syndicate.performance",
        "Mime",
        3,
    ),
];

const fn rank(
    id: &'static str,
    discipline_id: &'static str,
    title: &'static str,
    rung: u8,
) -> RankTitle {
    RankTitle {
        id,
        discipline_id,
        title,
        rung,
        source_ref: REV_C_SOURCE,
        catalog_version: SYNDICATE_CATALOG_VERSION,
    }
}
