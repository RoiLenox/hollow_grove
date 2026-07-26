//! Canonical visual identity for Hollow Grove.
//!
//! This constitutional domain is deliberately above and outside the recursion
//! kernel. Renderers consume the machine-readable palette through these
//! accessors; they do not own or restate canonical color values.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::OnceLock;

use serde::Deserialize;

use crate::hollow_grove_contract::House;

pub const VISUAL_COLOR_CONSTITUTION_ID: &str = "hollow-grove.visual-color-constitution";
pub const VISUAL_COLOR_PALETTE_SCHEMA_VERSION: &str = "1.0.0";
pub const CANONICAL_VISUAL_COLOR_PALETTE_PATH: &str =
    "src/constitutional/hollow_grove_visual_color_palette.json";
pub const CANONICAL_VISUAL_COLOR_PALETTE_JSON: &str =
    include_str!("hollow_grove_visual_color_palette.json");

const REQUIRED_RENDER_ORDER: [RenderLayer; 6] = [
    RenderLayer::UniversalOutline,
    RenderLayer::UniversalShadow,
    RenderLayer::HouseDark,
    RenderLayer::HousePrimary,
    RenderLayer::HouseHighlight,
    RenderLayer::Overlays,
];

const REQUIRED_OVERLAY_FAMILIES: [OverlayFamily; 3] = [
    OverlayFamily::Current,
    OverlayFamily::Aura,
    OverlayFamily::Material,
];

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VisualColorConstitution {
    pub schema_version: String,
    pub constitution_id: String,
    pub colors: Vec<CanonicalColor>,
    pub universal_foundation: UniversalFoundation,
    pub houses: Vec<HouseVisualPalette>,
    pub rendering_law: RenderingLaw,
    pub pure_black_policy: PureBlackPolicy,
}

impl VisualColorConstitution {
    #[must_use]
    pub fn color(&self, canonical_id: &str) -> Option<&CanonicalColor> {
        self.colors.iter().find(|color| color.id == canonical_id)
    }

    #[must_use]
    pub fn semantic_color(&self, semantic_identity: &str) -> Option<&CanonicalColor> {
        self.colors
            .iter()
            .find(|color| color.semantic_identity == semantic_identity)
    }

    #[must_use]
    pub fn house_palette(&self, house: House) -> Option<&HouseVisualPalette> {
        let canonical_id = match house {
            House::Stonebend => "stonebend",
            House::Sandmanor => "sandmanor",
            House::Glaushouse => "glaushouse",
            House::Flynt => "flynt",
        };
        self.houses
            .iter()
            .find(|palette| palette.id == canonical_id)
    }

    pub fn ordinary_sprite_defaults(&self) -> Result<Vec<&CanonicalColor>, VisualColorAccessError> {
        self.universal_foundation
            .ordinary_sprite_order
            .iter()
            .map(|id| self.require_color(id))
            .collect()
    }

    pub fn renderer_palette(
        &self,
        house: House,
    ) -> Result<RendererHousePalette<'_>, VisualColorAccessError> {
        let house_palette = self
            .house_palette(house)
            .ok_or(VisualColorAccessError::MissingHouse(house))?;
        let universal_outline = self.require_color(&self.universal_foundation.outline)?;
        let universal_shadow = self
            .universal_foundation
            .shadow
            .iter()
            .map(|id| self.require_color(id))
            .collect::<Result<Vec<_>, _>>()?;
        let house_dark = self.require_color(&house_palette.dark)?;
        let house_primary = self.require_color(&house_palette.primary)?;
        let house_highlight = self.require_color(&house_palette.highlight)?;
        let semantic_overlays = house_palette
            .semantic_overlays
            .iter()
            .map(|binding| {
                Ok(RendererSemanticOverlay {
                    identity: binding.identity.as_str(),
                    color: self.require_color(&binding.color)?,
                })
            })
            .collect::<Result<Vec<_>, VisualColorAccessError>>()?;

        Ok(RendererHousePalette {
            constitution: self,
            house,
            universal_outline,
            universal_shadow,
            house_dark,
            house_primary,
            house_highlight,
            semantic_overlays,
        })
    }

    /// Pure black is available only through an explicitly reserved
    /// constitutional state. It is never part of ordinary renderer defaults.
    #[must_use]
    pub fn reserved_pure_black(&self, constitutional_state: &str) -> Option<RgbColor> {
        self.pure_black_policy
            .reserved_states
            .iter()
            .any(|state| state == constitutional_state)
            .then(|| RgbColor::from_hex(&self.pure_black_policy.hex).ok())
            .flatten()
    }

    fn require_color(&self, id: &str) -> Result<&CanonicalColor, VisualColorAccessError> {
        self.color(id)
            .ok_or_else(|| VisualColorAccessError::MissingColor(id.to_owned()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalColor {
    pub id: String,
    pub name: String,
    pub hex: String,
    pub semantic_identity: String,
}

impl CanonicalColor {
    pub fn rgb(&self) -> Result<RgbColor, HexColorError> {
        RgbColor::from_hex(&self.hex)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UniversalFoundation {
    pub ordinary_sprite_order: Vec<String>,
    pub outline: String,
    pub shadow: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HouseVisualPalette {
    pub id: String,
    pub name: String,
    pub dark: String,
    pub primary: String,
    pub highlight: String,
    pub semantic_overlays: Vec<SemanticColorBinding>,
    pub meanings: Vec<String>,
}

impl HouseVisualPalette {
    #[must_use]
    pub fn canonical_color_ids(&self) -> Vec<&str> {
        let mut ids = vec![
            self.dark.as_str(),
            self.primary.as_str(),
            self.highlight.as_str(),
        ];
        ids.extend(
            self.semantic_overlays
                .iter()
                .map(|binding| binding.color.as_str()),
        );
        ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticColorBinding {
    pub identity: String,
    pub color: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenderingLaw {
    pub order: Vec<RenderLayer>,
    pub overlay_families: Vec<OverlayFamily>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderLayer {
    UniversalOutline,
    UniversalShadow,
    HouseDark,
    HousePrimary,
    HouseHighlight,
    Overlays,
}

impl RenderLayer {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UniversalOutline => "universal_outline",
            Self::UniversalShadow => "universal_shadow",
            Self::HouseDark => "house_dark",
            Self::HousePrimary => "house_primary",
            Self::HouseHighlight => "house_highlight",
            Self::Overlays => "overlays",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverlayFamily {
    Current,
    Aura,
    Material,
}

impl OverlayFamily {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Aura => "aura",
            Self::Material => "material",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PureBlackPolicy {
    pub id: String,
    pub hex: String,
    pub ordinary_sprite_default: bool,
    pub reserved_states: Vec<String>,
    pub future_states_require_explicit_constitutional_definition: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RgbColor {
    pub fn from_hex(hex: &str) -> Result<Self, HexColorError> {
        let digits = hex.strip_prefix('#').ok_or(HexColorError::MissingHash)?;
        if digits.len() != 6 || !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(HexColorError::InvalidDigits);
        }
        let red =
            u8::from_str_radix(&digits[0..2], 16).map_err(|_| HexColorError::InvalidDigits)?;
        let green =
            u8::from_str_radix(&digits[2..4], 16).map_err(|_| HexColorError::InvalidDigits)?;
        let blue =
            u8::from_str_radix(&digits[4..6], 16).map_err(|_| HexColorError::InvalidDigits)?;
        Ok(Self { red, green, blue })
    }

    #[must_use]
    pub const fn is_pure_black(self) -> bool {
        self.red == 0 && self.green == 0 && self.blue == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HexColorError {
    MissingHash,
    InvalidDigits,
}

impl fmt::Display for HexColorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid six-digit RGB color: {self:?}")
    }
}

impl std::error::Error for HexColorError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualColorAccessError {
    MissingHouse(House),
    MissingColor(String),
    DisallowedOverlayFamily(OverlayFamily),
}

impl fmt::Display for VisualColorAccessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "visual color access failed: {self:?}")
    }
}

impl std::error::Error for VisualColorAccessError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RendererHousePalette<'a> {
    constitution: &'a VisualColorConstitution,
    pub house: House,
    pub universal_outline: &'a CanonicalColor,
    pub universal_shadow: Vec<&'a CanonicalColor>,
    pub house_dark: &'a CanonicalColor,
    pub house_primary: &'a CanonicalColor,
    pub house_highlight: &'a CanonicalColor,
    pub semantic_overlays: Vec<RendererSemanticOverlay<'a>>,
}

impl<'a> RendererHousePalette<'a> {
    /// Returns the mandatory base construction order. Both universal shadow
    /// values occupy the one universal-shadow layer and precede House color.
    #[must_use]
    pub fn ordered_base_layers(&self) -> Vec<RendererColorLayer<'a>> {
        let mut layers = vec![RendererColorLayer {
            layer: RenderLayer::UniversalOutline,
            color: self.universal_outline,
        }];
        layers.extend(
            self.universal_shadow
                .iter()
                .copied()
                .map(|color| RendererColorLayer {
                    layer: RenderLayer::UniversalShadow,
                    color,
                }),
        );
        layers.extend([
            RendererColorLayer {
                layer: RenderLayer::HouseDark,
                color: self.house_dark,
            },
            RendererColorLayer {
                layer: RenderLayer::HousePrimary,
                color: self.house_primary,
            },
            RendererColorLayer {
                layer: RenderLayer::HouseHighlight,
                color: self.house_highlight,
            },
        ]);
        layers
    }

    pub fn resolve_overlay(
        &self,
        family: OverlayFamily,
        canonical_color_id: &str,
    ) -> Result<RendererOverlay<'a>, VisualColorAccessError> {
        if !self
            .constitution
            .rendering_law
            .overlay_families
            .contains(&family)
        {
            return Err(VisualColorAccessError::DisallowedOverlayFamily(family));
        }
        Ok(RendererOverlay {
            family,
            color: self.constitution.require_color(canonical_color_id)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RendererColorLayer<'a> {
    pub layer: RenderLayer,
    pub color: &'a CanonicalColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RendererSemanticOverlay<'a> {
    pub identity: &'a str,
    pub color: &'a CanonicalColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RendererOverlay<'a> {
    pub family: OverlayFamily,
    pub color: &'a CanonicalColor,
}

#[derive(Debug)]
pub enum VisualColorLoadError {
    InvalidJson(serde_json::Error),
    InvalidConstitution(Vec<VisualColorDiagnostic>),
}

impl fmt::Display for VisualColorLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJson(error) => write!(formatter, "invalid visual palette JSON: {error}"),
            Self::InvalidConstitution(diagnostics) => write!(
                formatter,
                "visual color constitution has {} validation error(s)",
                diagnostics.len()
            ),
        }
    }
}

impl std::error::Error for VisualColorLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidJson(error) => Some(error),
            Self::InvalidConstitution(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VisualColorDiagnosticCode {
    WrongSchemaVersion,
    WrongConstitutionIdentity,
    DuplicateCanonicalIdentifier,
    DuplicateSemanticIdentity,
    InvalidCanonicalColor,
    PureBlackCanonicalColor,
    MissingHousePalette,
    DuplicateHousePalette,
    InvalidHousePalette,
    MissingColorReference,
    DuplicatePaletteAssignment,
    UnassignedCanonicalColor,
    InvalidUniversalFoundation,
    InvalidRenderingOrder,
    InvalidOverlayFamilies,
    InvalidPureBlackPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualColorDiagnostic {
    pub code: VisualColorDiagnosticCode,
    pub message: String,
}

fn diagnostic(
    diagnostics: &mut Vec<VisualColorDiagnostic>,
    code: VisualColorDiagnosticCode,
    message: impl Into<String>,
) {
    diagnostics.push(VisualColorDiagnostic {
        code,
        message: message.into(),
    });
}

pub fn load_visual_color_constitution(
    source: &str,
) -> Result<VisualColorConstitution, VisualColorLoadError> {
    let constitution = serde_json::from_str(source).map_err(VisualColorLoadError::InvalidJson)?;
    validate_visual_color_constitution(&constitution)
        .map_err(VisualColorLoadError::InvalidConstitution)?;
    Ok(constitution)
}

pub fn validate_visual_color_constitution(
    constitution: &VisualColorConstitution,
) -> Result<(), Vec<VisualColorDiagnostic>> {
    let mut diagnostics = Vec::new();

    if constitution.schema_version != VISUAL_COLOR_PALETTE_SCHEMA_VERSION {
        diagnostic(
            &mut diagnostics,
            VisualColorDiagnosticCode::WrongSchemaVersion,
            format!(
                "schema version must be {VISUAL_COLOR_PALETTE_SCHEMA_VERSION}, found {}",
                constitution.schema_version
            ),
        );
    }
    if constitution.constitution_id != VISUAL_COLOR_CONSTITUTION_ID {
        diagnostic(
            &mut diagnostics,
            VisualColorDiagnosticCode::WrongConstitutionIdentity,
            format!(
                "constitution id must be {VISUAL_COLOR_CONSTITUTION_ID}, found {}",
                constitution.constitution_id
            ),
        );
    }

    let mut colors_by_id = BTreeMap::new();
    let mut semantic_identities = BTreeSet::new();
    for color in &constitution.colors {
        if colors_by_id.insert(color.id.as_str(), color).is_some() {
            diagnostic(
                &mut diagnostics,
                VisualColorDiagnosticCode::DuplicateCanonicalIdentifier,
                format!("canonical color identifier {} is duplicated", color.id),
            );
        }
        if color.semantic_identity.is_empty()
            || !semantic_identities.insert(color.semantic_identity.as_str())
        {
            diagnostic(
                &mut diagnostics,
                VisualColorDiagnosticCode::DuplicateSemanticIdentity,
                format!(
                    "canonical color {} does not have a unique semantic identity",
                    color.id
                ),
            );
        }
        match color.rgb() {
            Ok(rgb) if rgb.is_pure_black() => diagnostic(
                &mut diagnostics,
                VisualColorDiagnosticCode::PureBlackCanonicalColor,
                format!(
                    "ordinary canonical color {} resolves to constitutionally reserved pure black",
                    color.id
                ),
            ),
            Ok(_) => {}
            Err(error) => diagnostic(
                &mut diagnostics,
                VisualColorDiagnosticCode::InvalidCanonicalColor,
                format!(
                    "canonical color {} has invalid RGB value: {error}",
                    color.id
                ),
            ),
        }
    }
    if colors_by_id.contains_key(constitution.pure_black_policy.id.as_str()) {
        diagnostic(
            &mut diagnostics,
            VisualColorDiagnosticCode::DuplicateCanonicalIdentifier,
            format!(
                "reserved pure-black identifier {} collides with an ordinary canonical color",
                constitution.pure_black_policy.id
            ),
        );
    }

    validate_foundation(constitution, &colors_by_id, &mut diagnostics);
    validate_houses(constitution, &colors_by_id, &mut diagnostics);

    if constitution.rendering_law.order != REQUIRED_RENDER_ORDER {
        diagnostic(
            &mut diagnostics,
            VisualColorDiagnosticCode::InvalidRenderingOrder,
            "rendering order must be universal outline, universal shadow, House dark, House primary, House highlight, then overlays",
        );
    }
    if constitution.rendering_law.overlay_families != REQUIRED_OVERLAY_FAMILIES {
        diagnostic(
            &mut diagnostics,
            VisualColorDiagnosticCode::InvalidOverlayFamilies,
            "overlay families must be Current, Aura, and material in canonical order",
        );
    }

    validate_pure_black_policy(constitution, &mut diagnostics);

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn validate_foundation<'a>(
    constitution: &'a VisualColorConstitution,
    colors_by_id: &BTreeMap<&'a str, &'a CanonicalColor>,
    diagnostics: &mut Vec<VisualColorDiagnostic>,
) {
    let foundation = &constitution.universal_foundation;
    let expected_order = std::iter::once(&foundation.outline)
        .chain(foundation.shadow.iter())
        .collect::<Vec<_>>();
    if foundation.ordinary_sprite_order.iter().collect::<Vec<_>>() != expected_order
        || expected_order.len() != 3
    {
        diagnostic(
            diagnostics,
            VisualColorDiagnosticCode::InvalidUniversalFoundation,
            "ordinary sprites must begin with one universal outline followed by both universal shadow colors",
        );
    }

    let mut foundation_ids = BTreeSet::new();
    for id in &foundation.ordinary_sprite_order {
        if !foundation_ids.insert(id.as_str()) {
            diagnostic(
                diagnostics,
                VisualColorDiagnosticCode::InvalidUniversalFoundation,
                format!("universal foundation color {id} is repeated"),
            );
        }
        let Some(color) = colors_by_id.get(id.as_str()) else {
            diagnostic(
                diagnostics,
                VisualColorDiagnosticCode::MissingColorReference,
                format!("universal foundation references missing color {id}"),
            );
            continue;
        };
        if color.rgb().is_ok_and(RgbColor::is_pure_black) {
            diagnostic(
                diagnostics,
                VisualColorDiagnosticCode::PureBlackCanonicalColor,
                format!("ordinary sprite default {id} resolves to pure black"),
            );
        }
    }
}

fn validate_houses<'a>(
    constitution: &'a VisualColorConstitution,
    colors_by_id: &BTreeMap<&'a str, &'a CanonicalColor>,
    diagnostics: &mut Vec<VisualColorDiagnostic>,
) {
    let required_houses = ["stonebend", "sandmanor", "glaushouse", "flynt"];
    let mut house_ids = BTreeSet::new();
    let mut assigned_colors = constitution
        .universal_foundation
        .ordinary_sprite_order
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();

    for house in &constitution.houses {
        if !house_ids.insert(house.id.as_str()) {
            diagnostic(
                diagnostics,
                VisualColorDiagnosticCode::DuplicateHousePalette,
                format!("House palette {} is duplicated", house.id),
            );
        }
        if !required_houses.contains(&house.id.as_str()) {
            diagnostic(
                diagnostics,
                VisualColorDiagnosticCode::InvalidHousePalette,
                format!("{} is not a canonical Hollow Grove House", house.id),
            );
        }
        if house.meanings.len() != 6
            || house.meanings.iter().any(|meaning| meaning.is_empty())
            || house.meanings.iter().collect::<BTreeSet<_>>().len() != house.meanings.len()
        {
            diagnostic(
                diagnostics,
                VisualColorDiagnosticCode::InvalidHousePalette,
                format!("House {} must have six unique canonical meanings", house.id),
            );
        }

        let palette_ids = house.canonical_color_ids();
        let expected_color_count = if house.id == "flynt" { 4 } else { 3 };
        if palette_ids.len() != expected_color_count
            || palette_ids.iter().copied().collect::<BTreeSet<_>>().len() != expected_color_count
        {
            diagnostic(
                diagnostics,
                VisualColorDiagnosticCode::InvalidHousePalette,
                format!(
                    "House {} must have {expected_color_count} distinct canonical colors",
                    house.id
                ),
            );
        }
        if house.id == "flynt" {
            if house.semantic_overlays.len() != 1
                || house.semantic_overlays[0].identity != "authority"
            {
                diagnostic(
                    diagnostics,
                    VisualColorDiagnosticCode::InvalidHousePalette,
                    "Flynt must define its fourth blue-black color as the authority semantic overlay",
                );
            }
        } else if !house.semantic_overlays.is_empty() {
            diagnostic(
                diagnostics,
                VisualColorDiagnosticCode::InvalidHousePalette,
                format!("House {} has an unratified semantic overlay", house.id),
            );
        }

        for color_id in palette_ids {
            if !colors_by_id.contains_key(color_id) {
                diagnostic(
                    diagnostics,
                    VisualColorDiagnosticCode::MissingColorReference,
                    format!("House {} references missing color {color_id}", house.id),
                );
            }
            if !assigned_colors.insert(color_id) {
                diagnostic(
                    diagnostics,
                    VisualColorDiagnosticCode::DuplicatePaletteAssignment,
                    format!("canonical color {color_id} is assigned to multiple palettes"),
                );
            }
        }
    }

    for required_house in required_houses {
        let count = constitution
            .houses
            .iter()
            .filter(|house| house.id == required_house)
            .count();
        if count == 0 {
            diagnostic(
                diagnostics,
                VisualColorDiagnosticCode::MissingHousePalette,
                format!("House {required_house} has no canonical palette"),
            );
        }
    }

    for color_id in colors_by_id.keys() {
        if !assigned_colors.contains(color_id) {
            diagnostic(
                diagnostics,
                VisualColorDiagnosticCode::UnassignedCanonicalColor,
                format!("canonical color {color_id} is not assigned to a constitutional palette"),
            );
        }
    }
}

fn validate_pure_black_policy(
    constitution: &VisualColorConstitution,
    diagnostics: &mut Vec<VisualColorDiagnostic>,
) {
    let policy = &constitution.pure_black_policy;
    let required_states = ["Void", "Absolute Absence", "Erasure"];
    let is_black = RgbColor::from_hex(&policy.hex).is_ok_and(RgbColor::is_pure_black);
    let has_required_states = required_states
        .iter()
        .all(|required| policy.reserved_states.iter().any(|state| state == required));
    let unique_states = policy.reserved_states.iter().collect::<BTreeSet<_>>().len()
        == policy.reserved_states.len();
    if policy.id.is_empty()
        || !is_black
        || policy.ordinary_sprite_default
        || !has_required_states
        || !unique_states
        || !policy.future_states_require_explicit_constitutional_definition
    {
        diagnostic(
            diagnostics,
            VisualColorDiagnosticCode::InvalidPureBlackPolicy,
            "pure black must be excluded from ordinary defaults and reserved for unique, explicit constitutional states",
        );
    }
}

static CANONICAL_VISUAL_COLOR_CONSTITUTION: OnceLock<
    Result<VisualColorConstitution, VisualColorLoadError>,
> = OnceLock::new();

pub fn try_canonical_visual_color_constitution()
-> Result<&'static VisualColorConstitution, &'static VisualColorLoadError> {
    CANONICAL_VISUAL_COLOR_CONSTITUTION
        .get_or_init(|| load_visual_color_constitution(CANONICAL_VISUAL_COLOR_PALETTE_JSON))
        .as_ref()
}

#[must_use]
pub fn canonical_visual_color_constitution() -> &'static VisualColorConstitution {
    try_canonical_visual_color_constitution()
        .expect("embedded Hollow Grove visual color constitution must remain valid")
}

#[must_use]
pub fn build_visual_color_validation_report() -> String {
    match try_canonical_visual_color_constitution() {
        Ok(constitution) => format!(
            "Hollow Grove Visual Color Constitution: pass\ncanonical colors: {}\nHouse palettes: {}\nordinary pure-black defaults: 0\nrender order: {}\nsource: {}\n",
            constitution.colors.len(),
            constitution.houses.len(),
            constitution
                .rendering_law
                .order
                .iter()
                .map(|layer| layer.as_str())
                .collect::<Vec<_>>()
                .join(" -> "),
            CANONICAL_VISUAL_COLOR_PALETTE_PATH,
        ),
        Err(error) => format!("Hollow Grove Visual Color Constitution: fail\n{error}\n"),
    }
}

#[must_use]
pub fn build_visual_color_palette_output() -> String {
    let constitution = canonical_visual_color_constitution();
    let mut output = format!(
        "# Hollow Grove Visual Color Constitution\n\nSource: `{}`\n\n## Universal Foundation\n",
        CANONICAL_VISUAL_COLOR_PALETTE_PATH
    );
    for color in constitution
        .ordinary_sprite_defaults()
        .expect("validated universal colors must resolve")
    {
        output.push_str(&format!("- {}: {} ({})\n", color.name, color.hex, color.id));
    }
    for palette in &constitution.houses {
        output.push_str(&format!("\n## {}\n", palette.name));
        for color_id in palette.canonical_color_ids() {
            let color = constitution
                .color(color_id)
                .expect("validated House colors must resolve");
            output.push_str(&format!("- {}: {} ({})\n", color.name, color.hex, color.id));
        }
        output.push_str(&format!("- Meanings: {}\n", palette.meanings.join(", ")));
    }
    output
}

/// Builds an ephemeral GIMP Palette projection accepted by Aseprite. The GPL
/// text is deliberately generated from the constitutional catalog and is not
/// checked in as a second palette authority.
#[must_use]
pub fn build_aseprite_gpl_palette() -> String {
    let constitution = canonical_visual_color_constitution();
    let mut output =
        String::from("GIMP Palette\nName: Hollow Grove Visual Color Constitution\nColumns: 4\n#\n");
    for color in &constitution.colors {
        let rgb = color.rgb().expect("validated canonical RGB must resolve");
        output.push_str(&format!(
            "{:3} {:3} {:3}\t{} [{}]\n",
            rgb.red, rgb.green, rgb.blue, color.name, color.id
        ));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn canonical_fixture() -> VisualColorConstitution {
        serde_json::from_str(CANONICAL_VISUAL_COLOR_PALETTE_JSON)
            .expect("canonical visual color fixture must parse")
    }

    fn diagnostic_codes(
        result: Result<(), Vec<VisualColorDiagnostic>>,
    ) -> BTreeSet<VisualColorDiagnosticCode> {
        result
            .expect_err("mutated constitution must fail validation")
            .into_iter()
            .map(|diagnostic| diagnostic.code)
            .collect()
    }

    #[test]
    fn canonical_machine_palette_loads_and_validates() {
        let constitution = load_visual_color_constitution(CANONICAL_VISUAL_COLOR_PALETTE_JSON)
            .expect("canonical palette must validate");
        assert_eq!(constitution.colors.len(), 16);
        assert_eq!(constitution.houses.len(), 4);
        assert_eq!(constitution.rendering_law.order, REQUIRED_RENDER_ORDER);
    }

    #[test]
    fn renderer_accessors_preserve_the_constitutional_layer_order() {
        let constitution = canonical_visual_color_constitution();
        let palette = constitution
            .renderer_palette(House::Flynt)
            .expect("Flynt renderer palette must resolve");
        let layers = palette.ordered_base_layers();
        assert_eq!(layers.len(), 6);
        assert_eq!(layers[0].layer, RenderLayer::UniversalOutline);
        assert_eq!(layers[1].layer, RenderLayer::UniversalShadow);
        assert_eq!(layers[2].layer, RenderLayer::UniversalShadow);
        assert_eq!(layers[3].layer, RenderLayer::HouseDark);
        assert_eq!(layers[4].layer, RenderLayer::HousePrimary);
        assert_eq!(layers[5].layer, RenderLayer::HouseHighlight);
        assert_eq!(palette.semantic_overlays[0].identity, "authority");
        assert_eq!(
            palette.semantic_overlays[0].color.id,
            "flynt.tropical_indigo"
        );
    }

    #[test]
    fn ordinary_sprite_defaults_never_resolve_to_pure_black() {
        let defaults = canonical_visual_color_constitution()
            .ordinary_sprite_defaults()
            .expect("ordinary defaults must resolve");
        assert_eq!(defaults.len(), 3);
        assert!(
            defaults
                .iter()
                .all(|color| !color.rgb().expect("valid color").is_pure_black())
        );
    }

    #[test]
    fn pure_black_requires_an_explicit_reserved_state() {
        let constitution = canonical_visual_color_constitution();
        assert_eq!(
            constitution.reserved_pure_black("Void"),
            Some(RgbColor {
                red: 0,
                green: 0,
                blue: 0
            })
        );
        assert_eq!(constitution.reserved_pure_black("ordinary sprite"), None);
    }

    #[test]
    fn validation_rejects_a_missing_house_palette() {
        let mut fixture = canonical_fixture();
        fixture.houses.retain(|house| house.id != "sandmanor");
        let codes = diagnostic_codes(validate_visual_color_constitution(&fixture));
        assert!(codes.contains(&VisualColorDiagnosticCode::MissingHousePalette));
    }

    #[test]
    fn validation_rejects_duplicate_identifiers_and_semantic_identities() {
        let mut fixture = canonical_fixture();
        fixture.colors[1].id = fixture.colors[0].id.clone();
        fixture.colors[2].semantic_identity = fixture.colors[0].semantic_identity.clone();
        let codes = diagnostic_codes(validate_visual_color_constitution(&fixture));
        assert!(codes.contains(&VisualColorDiagnosticCode::DuplicateCanonicalIdentifier));
        assert!(codes.contains(&VisualColorDiagnosticCode::DuplicateSemanticIdentity));
    }

    #[test]
    fn validation_rejects_pure_black_as_an_ordinary_color() {
        let mut fixture = canonical_fixture();
        fixture.colors[0].hex = String::from("#000000");
        let codes = diagnostic_codes(validate_visual_color_constitution(&fixture));
        assert!(codes.contains(&VisualColorDiagnosticCode::PureBlackCanonicalColor));
    }

    #[test]
    fn validation_rejects_a_renderer_order_that_replaces_the_foundation() {
        let mut fixture = canonical_fixture();
        fixture.rendering_law.order.swap(0, 2);
        let codes = diagnostic_codes(validate_visual_color_constitution(&fixture));
        assert!(codes.contains(&VisualColorDiagnosticCode::InvalidRenderingOrder));
    }

    #[test]
    fn aseprite_projection_is_derived_from_every_canonical_color() {
        let output = build_aseprite_gpl_palette();
        assert!(output.starts_with("GIMP Palette\n"));
        for color in &canonical_visual_color_constitution().colors {
            assert!(output.contains(&format!("[{}]", color.id)));
        }
        assert!(!output.contains("  0   0   0\t"));
    }
}
