//! Local social influence, deliberately outside House and kernel authority.
//! Ambient state is derived from the event-sourced living clock: loading an
//! old save needs no invented allegiance, conquest, or additional event stream.

use serde::{Deserialize, Serialize};

use super::{CardinalDirection, LivingClock, TilePosition, WorkShift, WorldMapId};
use crate::hollow_grove_contract::House;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardwalkFactionId {
    Trooners,
    Gooners,
}

impl BoardwalkFactionId {
    pub const fn rival(self) -> Self {
        match self {
            Self::Trooners => Self::Gooners,
            Self::Gooners => Self::Trooners,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Trooners => "Trooners",
            Self::Gooners => "Gooners",
        }
    }

    pub const fn leader(self) -> &'static str {
        match self {
            Self::Trooners => "The Pimp",
            Self::Gooners => "The Gimp",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardwalkLevel {
    Upper,
    Lower,
}

impl BoardwalkLevel {
    pub const fn territory_id(self) -> &'static str {
        match self {
            Self::Upper => "territory.boardwalk.upper",
            Self::Lower => "territory.boardwalk.lower",
        }
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Upper => "Upper Boardwalk",
            Self::Lower => "Lower Boardwalk",
        }
    }

    pub const fn home_faction(self) -> BoardwalkFactionId {
        match self {
            Self::Upper => BoardwalkFactionId::Trooners,
            Self::Lower => BoardwalkFactionId::Gooners,
        }
    }
}

/// A replaceable influence value, not a jurisdiction or an access permission.
/// Independent scores allow a tie or no dominant faction. There is no
/// bridge-wide owner and no conquest terminal state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardwalkInfluence {
    pub trooners: u8,
    pub gooners: u8,
}

impl BoardwalkInfluence {
    pub const fn dominant(self) -> Option<BoardwalkFactionId> {
        if self.trooners > self.gooners {
            Some(BoardwalkFactionId::Trooners)
        } else if self.gooners > self.trooners {
            Some(BoardwalkFactionId::Gooners)
        } else {
            None
        }
    }

    pub const fn for_shift(level: BoardwalkLevel, shift: WorkShift) -> Self {
        let (home, rival) = match shift {
            WorkShift::Dawn => (64, 28),
            WorkShift::Day => (76, 24),
            WorkShift::Dusk => (68, 38),
            WorkShift::Night => (60, 42),
        };
        match level {
            BoardwalkLevel::Upper => Self {
                trooners: home,
                gooners: rival,
            },
            BoardwalkLevel::Lower => Self {
                trooners: rival,
                gooners: home,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardwalkFactionView {
    pub faction_id: BoardwalkFactionId,
    pub display_name: String,
    pub leader: String,
    pub leader_participant_id: String,
    pub home_territory_id: String,
    pub rival_faction: BoardwalkFactionId,
    pub sovereign_house_affiliation: Option<House>,
    pub syndicate_organization: Option<String>,
}

pub fn boardwalk_factions() -> Vec<BoardwalkFactionView> {
    [BoardwalkLevel::Upper, BoardwalkLevel::Lower]
        .into_iter()
        .map(|level| {
            let faction = level.home_faction();
            BoardwalkFactionView {
                faction_id: faction,
                display_name: faction.name().into(),
                leader: faction.leader().into(),
                leader_participant_id: match faction {
                    BoardwalkFactionId::Trooners => super::PIMP_PARTICIPANT_ID,
                    BoardwalkFactionId::Gooners => super::GIMP_PARTICIPANT_ID,
                }
                .into(),
                home_territory_id: level.territory_id().into(),
                rival_faction: faction.rival(),
                sovereign_house_affiliation: None,
                syndicate_organization: None,
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardwalkPresenceRole {
    Leader,
    Host,
    RivalCourier,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardwalkPresence {
    pub faction_id: BoardwalkFactionId,
    pub role: BoardwalkPresenceRole,
    pub display_name: String,
    pub activity: String,
    pub position: TilePosition,
}

pub fn boardwalk_presence(level: BoardwalkLevel, clock: LivingClock) -> Vec<BoardwalkPresence> {
    let home = level.home_faction();
    let evening = matches!(clock.shift, WorkShift::Dusk | WorkShift::Night);
    // All shifts use reserved side bays; no person can appear on a walking
    // tile occupied by the player, a stair, or the public central passage.
    let records = [
        (
            home,
            BoardwalkPresenceRole::Leader,
            home.leader().to_owned(),
            6,
            5,
            "Watching rival traffic; no authority to close the public passage",
        ),
        (
            home,
            BoardwalkPresenceRole::Host,
            format!("{} host", home.name()),
            13,
            if evening { 10 } else { 6 },
            match level {
                BoardwalkLevel::Upper => {
                    "Booking performances and tending commercial relationships"
                }
                BoardwalkLevel::Lower => "Trading discreetly and tending bridge machinery",
            },
        ),
        (
            home.rival(),
            BoardwalkPresenceRole::RivalCourier,
            format!("{} courier", home.rival().name()),
            6,
            11,
            "Watching the rival territory; keeping the stairs clear",
        ),
    ];
    records
        .into_iter()
        .map(
            |(faction_id, role, display_name, x, y, activity)| BoardwalkPresence {
                faction_id,
                role,
                display_name,
                activity: activity.into(),
                position: TilePosition {
                    x,
                    y,
                    facing: CardinalDirection::South,
                },
            },
        )
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardwalkTerritoryView {
    pub territory_id: String,
    pub level: BoardwalkLevel,
    pub display_name: String,
    pub route_id: String,
    pub formal_neutrality: bool,
    pub legal_status: String,
    pub sovereign_faction: Option<BoardwalkFactionId>,
    pub public_access: bool,
    pub contested: bool,
    pub influence: BoardwalkInfluence,
    pub dominant_faction: Option<BoardwalkFactionId>,
    pub rival_faction: BoardwalkFactionId,
    pub local_presence: Vec<BoardwalkPresence>,
}

impl BoardwalkTerritoryView {
    pub fn from_map(map: WorldMapId, clock: LivingClock) -> Option<Self> {
        let level = map.boardwalk_level()?;
        Some(Self::with_influence(
            level,
            clock,
            BoardwalkInfluence::for_shift(level, clock.shift),
        ))
    }

    /// Future reputation/commerce/conflict reducers can supply influence here
    /// without acquiring a path to House law, passage rights, or sovereignty.
    pub fn with_influence(
        level: BoardwalkLevel,
        clock: LivingClock,
        influence: BoardwalkInfluence,
    ) -> Self {
        Self {
            territory_id: level.territory_id().into(),
            level,
            display_name: level.display_name().into(),
            route_id: crate::world::geography::ConstitutionalRouteId::Boardwalk
                .stable_id()
                .into(),
            formal_neutrality: true,
            legal_status:
                "Neutral public infrastructure; existing Hollow Grove institutions retain authority"
                    .into(),
            sovereign_faction: None,
            public_access: true,
            contested: true,
            influence,
            dominant_faction: influence.dominant(),
            rival_faction: influence.dominant().unwrap_or(level.home_faction()).rival(),
            local_presence: boardwalk_presence(level, clock),
        }
    }
}
