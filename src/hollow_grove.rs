use crate::grove_seam::GroveSeam;
use crate::pleb_meta::{PlebMetaInput, RoutingPass};
use crate::symptom::Symptom;
use crate::triway::{Triway, Way};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KernelBond {
    linked: Way,
}

impl KernelBond {
    pub fn select(ways: [Way; 3]) -> Self {
        // Bond selection is still ordinal-only at this kernel depth.
        let [linked, _, _] = ways;
        Self { linked }
    }

    pub fn linked_way(self) -> Way {
        self.linked
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HollowGrove {
    source: Symptom,
    bond: KernelBond,
    atmosphere: [Way; 2],
}

impl HollowGrove {
    pub fn from_triway(triway: Triway) -> Self {
        let source = triway.source();
        let ways = triway.ways();
        let bond = KernelBond::select(ways);
        let [_, atmosphere_one, atmosphere_two] = ways;

        Self {
            source,
            bond,
            atmosphere: [atmosphere_one, atmosphere_two],
        }
    }

    pub fn link(&self) -> Way {
        self.bond.linked_way()
    }

    pub fn atmosphere(&self) -> [Way; 2] {
        self.atmosphere
    }

    pub fn become_grove_seam(self, routing_input: PlebMetaInput) -> GroveSeam {
        GroveSeam::from_symptom(self.source, RoutingPass::new(routing_input))
    }
}

/// Compatibility name for the kernel's ordinal link selector.
///
/// This is not the constitutional Bond aggregate. New boundary code should
/// name `KernelBond` explicitly and use `constitutional::BondAggregate` for
/// governed Current/Aura history.
pub type Bond = KernelBond;
