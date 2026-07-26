use std::collections::BTreeSet;

use hollow_grove::hollow_grove_contract::House;
use hollow_grove::lineage_contract::{SandmanorForm, validate_sandmanor_transition};
use hollow_grove::world::hueman_faculties::{
    CURRENT_FORM_PRESYNCE_LADDER, canonical_regional_soul_manifestations,
    canonical_resynce_cultures, sandmanor_soul_halves_equal, validate_regional_soul_manifestation,
    validate_resynce_cultures,
};
use hollow_grove::world::{house_institutions, sandmanor};
use hollow_grove::{InstitutionalEntityId, RelationshipKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    sandmanor::validate_civic_traditions()?;
    let catalog = house_institutions::canonical_house_institutions();
    catalog
        .validate()
        .map_err(|error| format!("neutral institution catalog failed: {error:?}"))?;

    let institutions = catalog
        .institutions
        .iter()
        .filter(|entry| entry.house == Some(House::Sandmanor))
        .map(|entry| entry.id.as_str())
        .collect::<BTreeSet<_>>();
    if institutions
        != BTreeSet::from([
            "institution.sandmanor.minoan-county-courthouse",
            "institution.sandmanor.sandmen",
        ])
    {
        return Err("Sandmanor institution roster drifted".into());
    }

    let offices = catalog
        .offices
        .iter()
        .filter(|entry| entry.house == Some(House::Sandmanor))
        .collect::<Vec<_>>();
    if offices.len() != 1
        || offices[0].id != sandmanor::sandman_office_id()
        || !offices[0].singular
        || offices[0].institution.as_ref() != Some(&sandmanor::proof_civilization_id())
    {
        return Err("the singular Sandman office placement drifted".into());
    }
    for authority in [
        "WitnessedImprovement",
        "ProofDetermination",
        "ReciprocalTeaching",
        "ReproductionOrder",
        "DesignIntegrity",
        "ContestIntegrity",
        "StandardsOfEvidence",
    ] {
        if !offices[0].authority.iter().any(|entry| entry == authority) {
            return Err(format!("Sandman authority `{authority}` is missing").into());
        }
    }
    if offices[0]
        .authority
        .iter()
        .any(|entry| entry == "CrowdRecognition")
    {
        return Err("popularity was promoted into Sandman authority".into());
    }

    let roles = catalog
        .roles
        .iter()
        .filter(|entry| entry.institution == sandmanor::proof_civilization_id())
        .map(|entry| entry.id.as_str())
        .collect::<BTreeSet<_>>();
    if roles != BTreeSet::from(["role.sandmanor.minorian", "role.sandmanor.minoan"]) {
        return Err("Sandmanor civic tradition roles drifted".into());
    }

    let groups = catalog
        .groups
        .iter()
        .filter(|entry| entry.institution == sandmanor::proof_civilization_id())
        .map(|entry| entry.id.as_str())
        .collect::<BTreeSet<_>>();
    if groups != BTreeSet::from(["group.sandmanor.minorians", "group.sandmanor.minoans"]) {
        return Err("Sandmanor civic tradition groups drifted".into());
    }

    let sites = catalog
        .sites
        .iter()
        .filter(|entry| entry.house == House::Sandmanor)
        .map(|entry| entry.id.as_str())
        .collect::<BTreeSet<_>>();
    if sites
        != BTreeSet::from([
            "site.sandmanor.aura-beach",
            "site.sandmanor.aura-fields",
            "site.sandmanor.minoan-county-courthouse",
        ])
    {
        return Err("Sandmanor regional proving grounds drifted".into());
    }

    let sandman = InstitutionalEntityId::Office(sandmanor::sandman_office_id());
    let proof_body = InstitutionalEntityId::Institution(sandmanor::proof_civilization_id());
    if !catalog.relationships.iter().any(|entry| {
        entry.source == sandman
            && entry.kind == RelationshipKind::Coordinates
            && entry.target == proof_body
    }) {
        return Err("Sandman-to-proof-body placement drifted".into());
    }

    validate_sandmanor_transition(
        SandmanorForm::Gnome.frame(),
        SandmanorForm::Minotaur.frame(),
    )
    .map_err(|error| format!("Gnome-to-Minotaur lineage drifted: {error:?}"))?;
    validate_sandmanor_transition(SandmanorForm::Elf.frame(), SandmanorForm::Centaur.frame())
        .map_err(|error| format!("Elf-to-Centaur lineage drifted: {error:?}"))?;
    validate_sandmanor_transition(
        SandmanorForm::Minotaur.frame(),
        SandmanorForm::Hecaton.frame(),
    )
    .map_err(|error| format!("Minotaur-to-Hecaton lineage drifted: {error:?}"))?;
    validate_sandmanor_transition(
        SandmanorForm::Centaur.frame(),
        SandmanorForm::Pegasus.frame(),
    )
    .map_err(|error| format!("Centaur-to-Pegasus lineage drifted: {error:?}"))?;

    use sandmanor::milestone::{
        CoastalTransferId, CoastalZone, ContentFarmAssessment, ContentFarmPractice,
        CourthouseTransfer, ManticorpCurrentBreakTraining, MaritimeTrainingId,
        NORTH_TO_SOUTH_COAST, SANDMAN_COMMON_NAME, SANDMAN_HISTORICAL_NAME,
        coast_is_progressively_regulated,
    };
    if NORTH_TO_SOUTH_COAST
        != [
            CoastalZone::FreeAuraBeach,
            CoastalZone::SouthernCoast,
            CoastalZone::CurrentBreak,
            CoastalZone::MinoanCountyCourthouse,
            CoastalZone::GlaushouseBorder,
        ]
        || !coast_is_progressively_regulated()
    {
        return Err("Sandmanor coast order or graduated Southern Law drifted".into());
    }
    let healthy_content = ContentFarmAssessment {
        practices: BTreeSet::from([
            ContentFarmPractice::Educates,
            ContentFarmPractice::PreservesMemory,
            ContentFarmPractice::NourishesAttention,
        ]),
    };
    if !healthy_content.is_healthy() || healthy_content.is_exploitative() {
        return Err("healthy Content Farm was treated as inherently corrupt".into());
    }
    CourthouseTransfer {
        id: CoastalTransferId::new("transfer.audit.courthouse-glaushouse")?,
        person: hollow_grove::institution::IdentityId::new("being.audit.coastal-transfer")?,
        from: CoastalZone::MinoanCountyCourthouse,
        to: CoastalZone::GlaushouseBorder,
        lawful_transfer: true,
        medical_or_clinical_reason: true,
        courthouse_authority_retained_by: House::Sandmanor,
        receiving_care_authority: House::Glaushouse,
    }
    .validate()?;
    ManticorpCurrentBreakTraining {
        id: MaritimeTrainingId::new("training.audit.current-break")?,
        manticorp_institution: hollow_grove::world::flynt::manticorp_id(),
        flynt_authorized_unit: true,
        sandmanor_authorized_access: true,
        minoan_coastal_instruction: true,
        command_house: House::Flynt,
        territorial_house: House::Sandmanor,
        creates_second_manticorp: false,
    }
    .validate()?;
    if SANDMAN_HISTORICAL_NAME != "Aegon" || SANDMAN_COMMON_NAME != "The Sandman" {
        return Err("Sandman historical/common naming drifted".into());
    }
    let design_indexes = hollow_grove::world::central_junction::canonical_market_indexes()
        .into_iter()
        .filter(|index| index.pole == hollow_grove::world::central_junction::EconomicPole::Design)
        .collect::<Vec<_>>();
    if design_indexes.len() != 1 || design_indexes[0].owner.is_some() || design_indexes[0].currency
    {
        return Err("Sandmanor Design was split into an owned index or currency".into());
    }

    if !sandmanor_soul_halves_equal() {
        return Err("Prefog and Prefig Soul equality drifted".into());
    }
    for manifestation in canonical_regional_soul_manifestations() {
        validate_regional_soul_manifestation(manifestation)?;
    }
    validate_resynce_cultures(&canonical_resynce_cultures())?;
    if CURRENT_FORM_PRESYNCE_LADDER.len() != 8 {
        return Err("Current Form Presynce ladder drifted".into());
    }

    println!("Sandmanor Constitutional Audit: pass");
    println!("source: {}", sandmanor::SANDMANOR_CONSTITUTION_SOURCE);
    println!("governing verb: {}", sandmanor::SANDMANOR_GOVERNING_VERB);
    println!(
        "signature offense: {}",
        sandmanor::SANDMANOR_SIGNATURE_OFFENSE
    );
    println!("equal civic traditions: Minorians / Gnomes; Minoans / Elves");
    println!("singular highest office: The Sandman");
    println!("office authority origin: completed Contest of Improvement only");
    println!("stable proof-body adapter: institution.sandmanor.sandmen");
    println!("regional design: Gnome -> Minotaur / Aura Field / advanced tending and labor");
    println!("regional design: Elf -> Centaur / Aura Beach and Current Sea / roam and guard");
    println!("guardian line: Gnome -> Minotaur -> Hecaton / maintained Form and distinct mantle");
    println!("guardian line: Elf -> Centaur -> Pegasus / maintained Form and distinct mantle");
    println!(
        "coastal order: Free Aura Beach -> Southern Coast -> Current Break -> Minoan County Courthouse -> Glaushouse"
    );
    println!("Manticorp command at Current Break remains Flynt-bound: true");
    println!("historical sovereign name: Aegon; common name: The Sandman");
    println!("official Sandmanor Design indexes: 1");
    println!("proof substitutes for Title, clearance, consent, recognition, or office: false");
    println!("legacy progression creates proof, mastery, credential, or office: false");
    println!("Hueman Soul: Minorian Prefog == Minoan Prefig");
    println!("Soul cycle: Prefog -> Prefig -> Proof -> Evidence or Failure -> Revision -> Prefog");
    println!("Prefog or Prefig directly creates proof: false");
    println!("Recipe faculty manifestations independently execute Synthesis: false");
    println!("Current Form Presynce ladder stages: 8");
    println!("We Fairy Men / Aura Ridge and Gallows / Flynt civic identities merged: false");
    println!("recursion kernel dependency: none");
    Ok(())
}
