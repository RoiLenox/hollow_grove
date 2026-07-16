use crate::hollow_grove_contract::{AlignmentDiagnostic, AlignmentDiagnosticCode};
use crate::pleb_meta::{ExteriorShape, Operator, PlebMetaGrammar, PlebMetaInput, Sequence};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Manager {
    Hal,
    Clouseau,
    Cleopatra,
}

impl Manager {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hal => "HAL",
            Self::Clouseau => "Clouseau",
            Self::Cleopatra => "Cleopatra",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManagerDomain {
    Meta,
    Pleb,
    Blep,
}

impl ManagerDomain {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Meta => "META",
            Self::Pleb => "PLEB",
            Self::Blep => "BLEP",
        }
    }

    #[must_use]
    pub const fn manager_language(self) -> &'static str {
        match self {
            Self::Pleb => "Proxy",
            Self::Meta => "Moxy",
            Self::Blep => "Foxy",
        }
    }
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

impl ManagerGeometry {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Curved => "Curved",
            Self::Straight => "Straight",
            Self::Inverted => "Inverted",
        }
    }

    #[must_use]
    pub const fn spatial_geometry(self) -> &'static str {
        match self {
            Self::Straight => "Flat",
            Self::Curved => "Round",
            Self::Inverted => "Inverted",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManagerFunction {
    Locate,
    Connect,
    Reflect,
}

impl ManagerFunction {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Locate => "Locates",
            Self::Connect => "Connects",
            Self::Reflect => "Reflects",
        }
    }
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
        matches!(self.function, ManagerFunction::Connect)
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
            ManagerFunction::Connect,
        ),
        Manager::Clouseau => ManagerDomainLock::new(
            Manager::Clouseau,
            ManagerDomain::Pleb,
            ManagerRelation::PlebPleb,
            ManagerGeometry::Straight,
            ManagerFunction::Locate,
        ),
        Manager::Cleopatra => ManagerDomainLock::new(
            Manager::Cleopatra,
            ManagerDomain::Blep,
            ManagerRelation::PlebBlep,
            ManagerGeometry::Inverted,
            ManagerFunction::Reflect,
        ),
    }
}

pub fn compact_manager_domain_law() -> [&'static str; 3] {
    ["Clouseau locates.", "HAL connects.", "Cleopatra reflects."]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManagerLanguageContractInput {
    pub pleb_layer: ManagerDomain,
    pub meta_layer: ManagerDomain,
    pub blep_layer: ManagerDomain,
    pub clouseau_layer: ManagerDomain,
    pub hal_layer: ManagerDomain,
    pub cleopatra_layer: ManagerDomain,
    pub clouseau_function: ManagerFunction,
    pub hal_function: ManagerFunction,
    pub cleopatra_function: ManagerFunction,
    pub proxy_flat_only: bool,
    pub round_proxy_forbidden: bool,
    pub foxy_automatically_evil: bool,
}

impl Default for ManagerLanguageContractInput {
    fn default() -> Self {
        Self {
            pleb_layer: ManagerDomain::Pleb,
            meta_layer: ManagerDomain::Meta,
            blep_layer: ManagerDomain::Blep,
            clouseau_layer: ManagerDomain::Pleb,
            hal_layer: ManagerDomain::Meta,
            cleopatra_layer: ManagerDomain::Blep,
            clouseau_function: ManagerFunction::Locate,
            hal_function: ManagerFunction::Connect,
            cleopatra_function: ManagerFunction::Reflect,
            proxy_flat_only: false,
            round_proxy_forbidden: false,
            foxy_automatically_evil: false,
        }
    }
}

pub fn canonical_manager_language_contract_fixture() -> ManagerLanguageContractInput {
    ManagerLanguageContractInput::default()
}

pub fn validate_manager_language_contract(
    input: &ManagerLanguageContractInput,
) -> Vec<AlignmentDiagnostic> {
    let mut diagnostics = Vec::new();

    if input.pleb_layer != ManagerDomain::Pleb {
        diagnostics.push(manager_language_error("PLEB must map to Proxy."));
    }
    if input.meta_layer != ManagerDomain::Meta {
        diagnostics.push(manager_language_error("META must map to Moxy."));
    }
    if input.blep_layer != ManagerDomain::Blep {
        diagnostics.push(manager_language_error("BLEP must map to Foxy."));
    }
    if input.clouseau_layer != ManagerDomain::Pleb {
        diagnostics.push(manager_language_error("Clouseau must map to Proxy / PLEB."));
    }
    if input.hal_layer != ManagerDomain::Meta {
        diagnostics.push(manager_language_error("HAL must map to Moxy / META."));
    }
    if input.cleopatra_layer != ManagerDomain::Blep {
        diagnostics.push(manager_language_error("Cleopatra must map to Foxy / BLEP."));
    }
    if input.clouseau_function != ManagerFunction::Locate {
        diagnostics.push(manager_language_error("Clouseau must locate."));
    }
    if input.hal_function != ManagerFunction::Connect {
        diagnostics.push(manager_language_error("HAL must connect."));
    }
    if input.cleopatra_function != ManagerFunction::Reflect {
        diagnostics.push(manager_language_error("Cleopatra must reflect."));
    }
    if input.proxy_flat_only || input.round_proxy_forbidden {
        diagnostics.push(manager_language_error(
            "Proxy cannot be restricted to Flat-only geometry; Round locations may still have Proxy.",
        ));
    }
    if input.foxy_automatically_evil {
        diagnostics.push(manager_language_error(
            "Foxy cannot automatically mean evil; its root meaning is reflection and inversion.",
        ));
    }

    diagnostics
}

pub fn build_manager_language_witness() -> String {
    String::from(
        "HOLLOW GROVE MANAGER LANGUAGE\n\n\
         Proxy\n\
         Domain: PLEB\n\
         Manager: Clouseau\n\
         Function: Places / locates\n\
         Question: Where is the player?\n\n\
         Moxy\n\
         Domain: META\n\
         Manager: HAL\n\
         Function: Bonds / connects\n\
         Question: What does this place connect toward?\n\n\
         Foxy\n\
         Domain: BLEP\n\
         Manager: Cleopatra\n\
         Function: Inverts / reflects\n\
         Question: What is the reflected expression?\n\n\
         Canonical doctrine:\n\
         Proxy places.\n\
         Moxy bonds.\n\
         Foxy inverts.\n",
    )
}

pub fn build_manager_language_validation_report() -> String {
    let diagnostics =
        validate_manager_language_contract(&canonical_manager_language_contract_fixture());
    if diagnostics.is_empty() {
        String::from(
            "# Hollow Grove Manager Language Validation\n\n\
             - status: pass\n\
             - PLEB / Proxy mapping: pass\n\
             - META / Moxy mapping: pass\n\
             - BLEP / Foxy mapping: pass\n\
             - Clouseau / Proxy mapping: pass\n\
             - HAL / Moxy mapping: pass\n\
             - Cleopatra / Foxy mapping: pass\n\
             - Proxy geometry flexibility: pass\n\
             - Foxy reflection semantics: pass\n",
        )
    } else {
        let mut output =
            String::from("# Hollow Grove Manager Language Validation\n\n- status: fail\n");
        for diagnostic in diagnostics {
            output.push_str(&format!(
                "- {:?}: {}\n",
                diagnostic.code, diagnostic.message
            ));
        }
        output
    }
}

fn manager_language_error(message: impl Into<String>) -> AlignmentDiagnostic {
    AlignmentDiagnostic {
        code: AlignmentDiagnosticCode::ManagerLanguageMismatch,
        message: message.into(),
    }
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
        Manager, ManagerDomain, ManagerFunction, ManagerGeometry, ManagerLanguageContractInput,
        ManagerRelation, build_manager_language_validation_report, build_manager_language_witness,
        canonical_manager_language_contract_fixture, compact_manager_domain_law,
        manager_domain_lock, routing_respects_manager_domain_lock,
        validate_manager_language_contract,
    };

    #[test]
    fn hal_domain_lock_is_meta_curved_connect() {
        let hal = manager_domain_lock(Manager::Hal);

        assert_eq!(hal.domain(), ManagerDomain::Meta);
        assert_eq!(hal.relation(), ManagerRelation::PlebMeta);
        assert_eq!(hal.geometry(), ManagerGeometry::Curved);
        assert_eq!(hal.function(), ManagerFunction::Connect);
        assert!(hal.governs_true_bond());
    }

    #[test]
    fn clouseau_domain_lock_is_pleb_straight_locate() {
        let clouseau = manager_domain_lock(Manager::Clouseau);

        assert_eq!(clouseau.domain(), ManagerDomain::Pleb);
        assert_eq!(clouseau.relation(), ManagerRelation::PlebPleb);
        assert_eq!(clouseau.geometry(), ManagerGeometry::Straight);
        assert_eq!(clouseau.function(), ManagerFunction::Locate);
        assert!(!clouseau.governs_true_bond());
    }

    #[test]
    fn cleopatra_domain_lock_is_blep_inverted_reflect() {
        let cleopatra = manager_domain_lock(Manager::Cleopatra);

        assert_eq!(cleopatra.domain(), ManagerDomain::Blep);
        assert_eq!(cleopatra.relation(), ManagerRelation::PlebBlep);
        assert_eq!(cleopatra.geometry(), ManagerGeometry::Inverted);
        assert_eq!(cleopatra.function(), ManagerFunction::Reflect);
        assert!(!cleopatra.governs_true_bond());
    }

    #[test]
    fn compact_law_remains_canonical() {
        assert_eq!(
            compact_manager_domain_law(),
            ["Clouseau locates.", "HAL connects.", "Cleopatra reflects.",]
        );
    }

    #[test]
    fn obsolete_manager_mappings_are_not_canonical() {
        let hal = manager_domain_lock(Manager::Hal);
        let clouseau = manager_domain_lock(Manager::Clouseau);
        let cleopatra = manager_domain_lock(Manager::Cleopatra);

        assert_ne!(hal.function(), ManagerFunction::Locate);
        assert_ne!(clouseau.function(), ManagerFunction::Connect);
        assert_ne!(cleopatra.relation(), ManagerRelation::PlebMeta);
        assert_ne!(cleopatra.relation(), ManagerRelation::PlebPleb);
    }

    #[test]
    fn canonical_manager_language_fixture_passes() {
        assert!(
            validate_manager_language_contract(&canonical_manager_language_contract_fixture())
                .is_empty()
        );
    }

    #[test]
    fn contradictory_manager_language_claims_fail() {
        let diagnostics = validate_manager_language_contract(&ManagerLanguageContractInput {
            hal_layer: ManagerDomain::Blep,
            proxy_flat_only: true,
            foxy_automatically_evil: true,
            ..ManagerLanguageContractInput::default()
        });

        assert_eq!(diagnostics.len(), 3);
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains("HAL must map to Moxy"))
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains("Flat-only geometry"))
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains("automatically mean evil"))
        );
    }

    #[test]
    fn manager_language_witness_and_validation_render() {
        assert!(build_manager_language_witness().contains("Proxy places."));
        assert!(build_manager_language_validation_report().contains("status: pass"));
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
