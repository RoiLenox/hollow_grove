use crate::pleb_meta::{ExteriorShape, Operator, PlebMetaGrammar, PlebMetaInput, Sequence};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Manager {
    Hal,
    Clouseau,
    Cleopatra,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManagerDomain {
    Meta,
    Pleb,
    Blep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManagerRelation {
    PlebMeta,
    PlebPleb,
    PlebBlep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManagerGeometry {
    Curved,
    Straight,
    Inverted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManagerFunction {
    InformationFromBeyond,
    Bond,
    UnderworldReflection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ManagerDomainLock {
    manager: Manager,
    domain: ManagerDomain,
    relation: ManagerRelation,
    geometry: ManagerGeometry,
    function: ManagerFunction,
}

impl ManagerDomainLock {
    pub const fn new(
        manager: Manager,
        domain: ManagerDomain,
        relation: ManagerRelation,
        geometry: ManagerGeometry,
        function: ManagerFunction,
    ) -> Self {
        Self {
            manager,
            domain,
            relation,
            geometry,
            function,
        }
    }

    pub const fn manager(self) -> Manager {
        self.manager
    }

    pub const fn domain(self) -> ManagerDomain {
        self.domain
    }

    pub const fn relation(self) -> ManagerRelation {
        self.relation
    }

    pub const fn geometry(self) -> ManagerGeometry {
        self.geometry
    }

    pub const fn function(self) -> ManagerFunction {
        self.function
    }

    pub const fn governs_true_bond(self) -> bool {
        matches!(self.function, ManagerFunction::Bond)
    }

    pub const fn operator(self) -> Operator {
        match self.manager {
            Manager::Hal => Operator::Hal,
            Manager::Clouseau => Operator::Clouseau,
            Manager::Cleopatra => Operator::Cleopatra,
        }
    }

    pub const fn domain_sequence(self) -> Sequence {
        match self.domain {
            ManagerDomain::Meta => Sequence::Meta,
            ManagerDomain::Pleb => Sequence::Pleb,
            ManagerDomain::Blep => Sequence::Blep,
        }
    }
}

pub const fn manager_domain_lock(manager: Manager) -> ManagerDomainLock {
    match manager {
        Manager::Hal => ManagerDomainLock::new(
            Manager::Hal,
            ManagerDomain::Meta,
            ManagerRelation::PlebMeta,
            ManagerGeometry::Curved,
            ManagerFunction::InformationFromBeyond,
        ),
        Manager::Clouseau => ManagerDomainLock::new(
            Manager::Clouseau,
            ManagerDomain::Pleb,
            ManagerRelation::PlebPleb,
            ManagerGeometry::Straight,
            ManagerFunction::Bond,
        ),
        Manager::Cleopatra => ManagerDomainLock::new(
            Manager::Cleopatra,
            ManagerDomain::Blep,
            ManagerRelation::PlebBlep,
            ManagerGeometry::Inverted,
            ManagerFunction::UnderworldReflection,
        ),
    }
}

pub fn compact_manager_domain_law() -> [&'static str; 3] {
    ["HAL reveals.", "Clouseau bonds.", "Cleopatra reflects."]
}

pub fn routing_respects_manager_domain_lock(input: PlebMetaInput) -> bool {
    let routing = PlebMetaGrammar::route(input);
    let exterior = routing.exterior();
    let interior = routing.interior();

    match exterior.shape() {
        ExteriorShape::Straight => {
            exterior.operator() == manager_domain_lock(Manager::Clouseau).operator()
                && exterior.foreground_sequence()
                    == manager_domain_lock(Manager::Clouseau).domain_sequence()
                && interior.operator() == manager_domain_lock(Manager::Cleopatra).operator()
        }
        ExteriorShape::Curved => {
            exterior.operator() == manager_domain_lock(Manager::Hal).operator()
                && exterior.foreground_sequence()
                    == manager_domain_lock(Manager::Hal).domain_sequence()
                && interior.operator() == manager_domain_lock(Manager::Cleopatra).operator()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pleb_meta::{ExteriorShape, Mode, PlebMetaInput};

    use super::{
        Manager, ManagerDomain, ManagerFunction, ManagerGeometry, ManagerRelation,
        compact_manager_domain_law, manager_domain_lock, routing_respects_manager_domain_lock,
    };

    #[test]
    fn hal_domain_lock_is_meta_curved_information_from_beyond() {
        let hal = manager_domain_lock(Manager::Hal);

        assert_eq!(hal.domain(), ManagerDomain::Meta);
        assert_eq!(hal.relation(), ManagerRelation::PlebMeta);
        assert_eq!(hal.geometry(), ManagerGeometry::Curved);
        assert_eq!(hal.function(), ManagerFunction::InformationFromBeyond);
        assert!(!hal.governs_true_bond());
    }

    #[test]
    fn clouseau_domain_lock_is_pleb_straight_bond() {
        let clouseau = manager_domain_lock(Manager::Clouseau);

        assert_eq!(clouseau.domain(), ManagerDomain::Pleb);
        assert_eq!(clouseau.relation(), ManagerRelation::PlebPleb);
        assert_eq!(clouseau.geometry(), ManagerGeometry::Straight);
        assert_eq!(clouseau.function(), ManagerFunction::Bond);
        assert!(clouseau.governs_true_bond());
    }

    #[test]
    fn cleopatra_domain_lock_is_blep_inverted_pleb_blep_underworld_reflection() {
        let cleopatra = manager_domain_lock(Manager::Cleopatra);

        assert_eq!(cleopatra.domain(), ManagerDomain::Blep);
        assert_eq!(cleopatra.relation(), ManagerRelation::PlebBlep);
        assert_eq!(cleopatra.geometry(), ManagerGeometry::Inverted);
        assert_eq!(cleopatra.function(), ManagerFunction::UnderworldReflection);
        assert!(!cleopatra.governs_true_bond());
    }

    #[test]
    fn compact_law_remains_canonical() {
        assert_eq!(
            compact_manager_domain_law(),
            ["HAL reveals.", "Clouseau bonds.", "Cleopatra reflects.",]
        );
    }

    #[test]
    fn obsolete_manager_mappings_are_not_canonical() {
        let hal = manager_domain_lock(Manager::Hal);
        let clouseau = manager_domain_lock(Manager::Clouseau);
        let cleopatra = manager_domain_lock(Manager::Cleopatra);

        assert_ne!(hal.function(), ManagerFunction::Bond);
        assert_ne!(clouseau.function(), ManagerFunction::InformationFromBeyond);
        assert_ne!(cleopatra.relation(), ManagerRelation::PlebMeta);
        assert_ne!(cleopatra.relation(), ManagerRelation::PlebPleb);
    }

    #[test]
    fn straight_routing_respects_clouseau_and_cleopatra_domains() {
        assert!(routing_respects_manager_domain_lock(PlebMetaInput {
            exterior_shape: ExteriorShape::Straight,
            pleb_mode: Mode::Pathos,
            meta_mode: Mode::Logos,
        }));
    }

    #[test]
    fn curved_routing_respects_hal_and_cleopatra_domains() {
        assert!(routing_respects_manager_domain_lock(PlebMetaInput {
            exterior_shape: ExteriorShape::Curved,
            pleb_mode: Mode::Pathos,
            meta_mode: Mode::Logos,
        }));
    }
}
