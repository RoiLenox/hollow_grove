use serde::{Deserialize, Serialize};

use crate::composition::{Position3, squared_distance};
use crate::hollow_grove_contract::House;

pub const CONSTITUTIONAL_HOUSE_ORDER: [House; 4] = [
    House::Stonebend,
    House::Sandmanor,
    House::Glaushouse,
    House::Flynt,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum KernelAnchor {
    House(House),
    CentralJunction,
}

impl KernelAnchor {
    #[must_use]
    pub const fn position(self) -> Position3 {
        match self {
            Self::House(House::Stonebend) => Position3::POSITIVE_Z_EXTREME,
            Self::House(House::Sandmanor) => Position3::POSITIVE_X_EXTREME,
            Self::House(House::Glaushouse) => Position3::NEGATIVE_Z_EXTREME,
            Self::House(House::Flynt) => Position3::NEGATIVE_X_EXTREME,
            Self::CentralJunction => Position3::ORIGIN,
        }
    }

    #[must_use]
    pub const fn canonical_name(self) -> &'static str {
        match self {
            Self::House(House::Stonebend) => "Stonebend",
            Self::House(House::Sandmanor) => "Sandmanor",
            Self::House(House::Glaushouse) => "Glaüshouse",
            Self::House(House::Flynt) => "Flynt",
            Self::CentralJunction => "Central Junction",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HouseDistance {
    pub house: House,
    pub squared_distance: i64,
}

#[must_use]
pub fn house_distances(position: Position3) -> Vec<HouseDistance> {
    let mut distances = CONSTITUTIONAL_HOUSE_ORDER
        .into_iter()
        .map(|house| HouseDistance {
            house,
            squared_distance: squared_distance(position, KernelAnchor::House(house).position()),
        })
        .collect::<Vec<_>>();
    distances.sort_by(|left, right| {
        left.squared_distance
            .cmp(&right.squared_distance)
            .then_with(|| house_order(left.house).cmp(&house_order(right.house)))
    });
    distances
}

const fn house_order(house: House) -> usize {
    match house {
        House::Stonebend => 0,
        House::Sandmanor => 1,
        House::Glaushouse => 2,
        House::Flynt => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constitutional_anchors_are_exact_and_central_junction_is_separate() {
        assert_eq!(
            KernelAnchor::House(House::Stonebend).position(),
            Position3::from_raw(0, 0, 1000).unwrap()
        );
        assert_eq!(
            KernelAnchor::House(House::Sandmanor).position(),
            Position3::from_raw(1000, 0, 0).unwrap()
        );
        assert_eq!(
            KernelAnchor::House(House::Glaushouse).position(),
            Position3::from_raw(0, 0, -1000).unwrap()
        );
        assert_eq!(
            KernelAnchor::House(House::Flynt).position(),
            Position3::from_raw(-1000, 0, 0).unwrap()
        );
        assert_eq!(KernelAnchor::CentralJunction.position(), Position3::ORIGIN);
        assert_eq!(
            KernelAnchor::House(House::Glaushouse).canonical_name(),
            "Glaüshouse"
        );
    }

    #[test]
    fn house_distance_returns_every_relationship_in_stable_order() {
        let distances = house_distances(Position3::ORIGIN);
        assert_eq!(
            distances
                .iter()
                .map(|entry| entry.house)
                .collect::<Vec<_>>(),
            CONSTITUTIONAL_HOUSE_ORDER
        );
        assert!(
            distances
                .iter()
                .all(|entry| entry.squared_distance == 1_000_000)
        );
    }

    #[test]
    fn overlap_does_not_collapse_to_one_house() {
        let distances = house_distances(Position3::from_raw(-250, 0, 600).unwrap());
        assert_eq!(distances.len(), 4);
        assert_eq!(distances[0].house, House::Stonebend);
        assert_eq!(distances[1].house, House::Flynt);
        assert!(distances[0].squared_distance < distances[1].squared_distance);
    }
}
