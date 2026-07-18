//! The canonical Hueman Fourway roster.
//!
//! This is world geometry only.  It provides a typed source for traversal and
//! presentation without feeding authority or decision logic into the kernel.

use crate::hollow_grove_contract::House;

/// A cardinal position in Hueman's Fourway map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FourwayDirection {
    North,
    East,
    South,
    West,
}

/// The current canonical House at a Fourway position.
#[must_use]
pub const fn house_at(direction: FourwayDirection) -> House {
    match direction {
        FourwayDirection::North => House::Stonebend,
        FourwayDirection::East => House::Glaushouse,
        FourwayDirection::South => House::Sandmanor,
        FourwayDirection::West => House::Flynt,
    }
}

/// The current canonical Fourway position for a House.
#[must_use]
pub const fn direction_of(house: House) -> FourwayDirection {
    match house {
        House::Stonebend => FourwayDirection::North,
        House::Glaushouse => FourwayDirection::East,
        House::Sandmanor => FourwayDirection::South,
        House::Flynt => FourwayDirection::West,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_roster_is_bidirectional() {
        for (direction, house) in [
            (FourwayDirection::North, House::Stonebend),
            (FourwayDirection::East, House::Glaushouse),
            (FourwayDirection::South, House::Sandmanor),
            (FourwayDirection::West, House::Flynt),
        ] {
            assert_eq!(house_at(direction), house);
            assert_eq!(direction_of(house), direction);
        }
    }
}
