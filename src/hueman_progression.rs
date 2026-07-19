use std::{fmt, io, path::Path};

use crate::current_synthesis_engine::{
    CURRENT_SYNTHESIS_TUI_STATE_ARTIFACT_PATH, parse_persisted_state,
};
use crate::hueman_slice::{
    SliceResolutionPath, VerticalSliceSpec, primary_vertical_slice, vertical_slice_by_id,
    vertical_slice_for_current_synthesis_scenario,
};
use crate::{read_text_artifact, write_text_artifact};

pub const HUEMAN_SLICE_STATE_ARTIFACT_PATH: &str = "artifacts/hueman_slice_state.txt";
pub const HUEMAN_SLICE_STATUS_ARTIFACT_PATH: &str = "artifacts/hueman_slice_status.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlicePhase {
    NeedObserved,
    SeamSurveyed,
    InputsGathered,
    OpalOilRefined,
    ToolNamed,
    ToolProven,
    ToolCleared,
    ToolDeployed,
    RecognitionEarned,
    CurrentFormUnlocked,
}

impl SlicePhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NeedObserved => "NeedObserved",
            Self::SeamSurveyed => "SeamSurveyed",
            Self::InputsGathered => "InputsGathered",
            Self::OpalOilRefined => "OpalOilRefined",
            Self::ToolNamed => "ToolNamed",
            Self::ToolProven => "ToolProven",
            Self::ToolCleared => "ToolCleared",
            Self::ToolDeployed => "ToolDeployed",
            Self::RecognitionEarned => "RecognitionEarned",
            Self::CurrentFormUnlocked => "CurrentFormUnlocked",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "NeedObserved" => Some(Self::NeedObserved),
            "SeamSurveyed" => Some(Self::SeamSurveyed),
            "InputsGathered" => Some(Self::InputsGathered),
            "OpalOilRefined" => Some(Self::OpalOilRefined),
            "ToolNamed" => Some(Self::ToolNamed),
            "ToolProven" => Some(Self::ToolProven),
            "ToolCleared" => Some(Self::ToolCleared),
            "ToolDeployed" => Some(Self::ToolDeployed),
            "RecognitionEarned" => Some(Self::RecognitionEarned),
            "CurrentFormUnlocked" | "GremlinUnlocked" => Some(Self::CurrentFormUnlocked),
            _ => None,
        }
    }

    fn requires_named_tool(self) -> bool {
        matches!(
            self,
            Self::ToolNamed
                | Self::ToolProven
                | Self::ToolCleared
                | Self::ToolDeployed
                | Self::RecognitionEarned
                | Self::CurrentFormUnlocked
        )
    }

    fn requires_resolution_path(self) -> bool {
        matches!(
            self,
            Self::ToolProven
                | Self::ToolCleared
                | Self::ToolDeployed
                | Self::RecognitionEarned
                | Self::CurrentFormUnlocked
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SliceResourceLedger {
    pub regular_current_units: u8,
    pub holographic_aura_units: u8,
    pub opal_oil_units: u8,
    pub branch_output_units: u8,
}

impl SliceResourceLedger {
    pub fn empty() -> Self {
        Self {
            regular_current_units: 0,
            holographic_aura_units: 0,
            opal_oil_units: 0,
            branch_output_units: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SliceUnlockState {
    pub current_form: &'static str,
    pub node_name: &'static str,
    pub unlocked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlyntAscensionState {
    pub gargoyle_mastered: bool,
    pub werewolf_mastered: bool,
    pub merman_mastered: bool,
    pub chimera_synthesized: bool,
    pub chimera_refined: bool,
    pub executive_mastery: bool,
    pub constitutionally_recognized: bool,
    pub lawfully_acceded: bool,
}

impl FlyntAscensionState {
    pub fn locked() -> Self {
        Self {
            gargoyle_mastered: false,
            werewolf_mastered: false,
            merman_mastered: false,
            chimera_synthesized: false,
            chimera_refined: false,
            executive_mastery: false,
            constitutionally_recognized: false,
            lawfully_acceded: false,
        }
    }

    pub fn holds_tross_office(self) -> bool {
        self.lawfully_acceded
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FollowUpPhase {
    Locked,
    Ready,
    InProgress,
    Completed,
}

impl FollowUpPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Locked => "Locked",
            Self::Ready => "Ready",
            Self::InProgress => "InProgress",
            Self::Completed => "Completed",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "Locked" => Some(Self::Locked),
            "Ready" => Some(Self::Ready),
            "InProgress" => Some(Self::InProgress),
            "Completed" => Some(Self::Completed),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerticalSliceState {
    spec: &'static VerticalSliceSpec,
    phase: SlicePhase,
    resources: SliceResourceLedger,
    named_tool: Option<String>,
    resolution_path: Option<SliceResolutionPath>,
    unlock: SliceUnlockState,
    follow_up_phase: FollowUpPhase,
    flynt_ascension: FlyntAscensionState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SliceProgressError {
    WrongPhase {
        expected: SlicePhase,
        actual: SlicePhase,
    },
    ToolNameMismatch {
        expected: &'static str,
        actual: String,
    },
    ResolutionPathMismatch {
        expected: SliceResolutionPath,
        actual: SliceResolutionPath,
    },
    MissingInputs {
        required_regular_current: u8,
        available_regular_current: u8,
        required_holographic_aura: u8,
        available_holographic_aura: u8,
    },
    MissingOpalOil {
        required_units: u8,
        available_units: u8,
    },
    FollowUpUnavailable {
        phase: FollowUpPhase,
    },
    FlyntRecipeGate {
        required: &'static str,
    },
}

impl fmt::Display for SliceProgressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongPhase { expected, actual } => write!(
                f,
                "wrong slice phase: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::ToolNameMismatch { expected, actual } => {
                write!(f, "expected tool name `{expected}`, got `{actual}`")
            }
            Self::ResolutionPathMismatch { expected, actual } => write!(
                f,
                "expected resolution path `{}`, got `{}`",
                expected.as_str(),
                actual.as_str()
            ),
            Self::MissingInputs {
                required_regular_current,
                available_regular_current,
                required_holographic_aura,
                available_holographic_aura,
            } => write!(
                f,
                "missing inputs: regular current {available_regular_current}/{required_regular_current}, holographic aura {available_holographic_aura}/{required_holographic_aura}"
            ),
            Self::MissingOpalOil {
                required_units,
                available_units,
            } => write!(
                f,
                "missing opal oil: {available_units}/{required_units} refined units available"
            ),
            Self::FollowUpUnavailable { phase } => write!(
                f,
                "follow-up task is not available from state {}",
                phase.as_str()
            ),
            Self::FlyntRecipeGate { required } => {
                write!(f, "Flynt ascension requires {required} first")
            }
        }
    }
}

impl std::error::Error for SliceProgressError {}

impl VerticalSliceState {
    pub fn new(spec: &'static VerticalSliceSpec) -> Self {
        Self {
            spec,
            phase: SlicePhase::NeedObserved,
            resources: SliceResourceLedger::empty(),
            named_tool: None,
            resolution_path: None,
            unlock: SliceUnlockState {
                current_form: spec.current_form,
                node_name: spec.unlock_node,
                unlocked: false,
            },
            follow_up_phase: FollowUpPhase::Locked,
            flynt_ascension: FlyntAscensionState::locked(),
        }
    }

    pub fn primary() -> Self {
        Self::new(primary_vertical_slice())
    }

    pub fn for_slice_id(slice_id: &str) -> io::Result<Self> {
        let spec = vertical_slice_by_id(slice_id).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unsupported slice_id: {slice_id}"),
            )
        })?;
        Ok(Self::new(spec))
    }

    pub fn for_current_synthesis_scenario(scenario_id: &str) -> io::Result<Self> {
        let spec = vertical_slice_for_current_synthesis_scenario(scenario_id).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unsupported scenario_id for Hueman slice mapping: {scenario_id}"),
            )
        })?;
        Ok(Self::new(spec))
    }

    pub fn spec(&self) -> &'static VerticalSliceSpec {
        self.spec
    }

    pub fn phase(&self) -> SlicePhase {
        self.phase
    }

    pub fn resources(&self) -> SliceResourceLedger {
        self.resources
    }

    pub fn named_tool(&self) -> Option<&str> {
        self.named_tool.as_deref()
    }

    pub fn unlock(&self) -> SliceUnlockState {
        self.unlock
    }

    pub fn resolution_path(&self) -> Option<SliceResolutionPath> {
        self.resolution_path
    }

    pub fn follow_up_phase(&self) -> FollowUpPhase {
        self.follow_up_phase
    }

    pub fn flynt_ascension(&self) -> FlyntAscensionState {
        self.flynt_ascension
    }

    pub fn survey_safe_seam(&mut self) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::NeedObserved)?;
        self.phase = SlicePhase::SeamSurveyed;
        Ok(())
    }

    pub fn gather_inputs(
        &mut self,
        regular_current_units: u8,
        holographic_aura_units: u8,
    ) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::SeamSurveyed)?;
        self.resources.regular_current_units = self
            .resources
            .regular_current_units
            .saturating_add(regular_current_units);
        self.resources.holographic_aura_units = self
            .resources
            .holographic_aura_units
            .saturating_add(holographic_aura_units);
        self.phase = SlicePhase::InputsGathered;
        Ok(())
    }

    pub fn refine_opal_oil(&mut self) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::InputsGathered)?;
        if self.resources.regular_current_units < self.spec.required_regular_current_units
            || self.resources.holographic_aura_units < self.spec.required_holographic_aura_units
        {
            return Err(SliceProgressError::MissingInputs {
                required_regular_current: self.spec.required_regular_current_units,
                available_regular_current: self.resources.regular_current_units,
                required_holographic_aura: self.spec.required_holographic_aura_units,
                available_holographic_aura: self.resources.holographic_aura_units,
            });
        }

        self.resources.regular_current_units -= self.spec.required_regular_current_units;
        self.resources.holographic_aura_units -= self.spec.required_holographic_aura_units;
        self.resources.opal_oil_units = self
            .resources
            .opal_oil_units
            .saturating_add(self.spec.required_opal_oil_units);
        self.phase = SlicePhase::OpalOilRefined;
        Ok(())
    }

    pub fn name_tool(&mut self, name: &str) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::OpalOilRefined)?;
        if self.resources.opal_oil_units < self.spec.required_opal_oil_units {
            return Err(SliceProgressError::MissingOpalOil {
                required_units: self.spec.required_opal_oil_units,
                available_units: self.resources.opal_oil_units,
            });
        }
        if name != self.spec.crafted_object {
            return Err(SliceProgressError::ToolNameMismatch {
                expected: self.spec.crafted_object,
                actual: name.to_string(),
            });
        }

        self.named_tool = Some(name.to_string());
        self.phase = SlicePhase::ToolNamed;
        Ok(())
    }

    pub fn prove_tool(&mut self) -> Result<(), SliceProgressError> {
        self.prove_tool_for(self.spec.default_resolution_path)
    }

    pub fn prove_tool_for(&mut self, path: SliceResolutionPath) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::ToolNamed)?;
        self.resolution_path = Some(path);
        self.phase = SlicePhase::ToolProven;
        Ok(())
    }

    pub fn clear_tool(&mut self) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::ToolProven)?;
        self.phase = SlicePhase::ToolCleared;
        Ok(())
    }

    pub fn deploy_tool(&mut self) -> Result<(), SliceProgressError> {
        self.deploy_tool_for(self.spec.default_resolution_path)
    }

    pub fn deploy_tool_for(&mut self, path: SliceResolutionPath) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::ToolCleared)?;
        if let Some(expected) = self.resolution_path
            && expected != path
        {
            return Err(SliceProgressError::ResolutionPathMismatch {
                expected,
                actual: path,
            });
        }
        self.resolution_path = Some(path);
        self.phase = SlicePhase::ToolDeployed;
        Ok(())
    }

    pub fn recognize_result(&mut self) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::ToolDeployed)?;
        let selected_path = self.resolution_path.ok_or(SliceProgressError::WrongPhase {
            expected: SlicePhase::ToolDeployed,
            actual: self.phase,
        })?;
        if let Some(option) = self.spec.resolution_option(selected_path) {
            self.resources.branch_output_units = option.produced_resource_units;
        }
        self.phase = SlicePhase::RecognitionEarned;
        Ok(())
    }

    pub fn unlock_first_current_form_node(&mut self) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::RecognitionEarned)?;
        self.unlock.unlocked = true;
        self.phase = SlicePhase::CurrentFormUnlocked;
        self.follow_up_phase = FollowUpPhase::Ready;
        Ok(())
    }

    pub fn start_follow_up_task(&mut self) -> Result<(), SliceProgressError> {
        if self.follow_up_phase != FollowUpPhase::Ready {
            return Err(SliceProgressError::FollowUpUnavailable {
                phase: self.follow_up_phase,
            });
        }
        self.follow_up_phase = FollowUpPhase::InProgress;
        Ok(())
    }

    pub fn complete_follow_up_task(&mut self) -> Result<(), SliceProgressError> {
        if self.follow_up_phase != FollowUpPhase::InProgress {
            return Err(SliceProgressError::FollowUpUnavailable {
                phase: self.follow_up_phase,
            });
        }
        self.follow_up_phase = FollowUpPhase::Completed;
        Ok(())
    }

    pub fn embody_gargoyle_form(&mut self) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::CurrentFormUnlocked)?;
        self.flynt_ascension.gargoyle_mastered = true;
        Ok(())
    }

    pub fn master_werewolf_branch(&mut self) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::CurrentFormUnlocked)?;
        if !self.flynt_ascension.gargoyle_mastered {
            return Err(SliceProgressError::FlyntRecipeGate {
                required: "Gargoyle mastery",
            });
        }
        self.flynt_ascension.werewolf_mastered = true;
        Ok(())
    }

    pub fn master_merman_branch(&mut self) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::CurrentFormUnlocked)?;
        if !self.flynt_ascension.gargoyle_mastered {
            return Err(SliceProgressError::FlyntRecipeGate {
                required: "Gargoyle mastery",
            });
        }
        self.flynt_ascension.merman_mastered = true;
        Ok(())
    }

    pub fn synthesize_chimera_form(&mut self) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::CurrentFormUnlocked)?;
        if !self.flynt_ascension.gargoyle_mastered {
            return Err(SliceProgressError::FlyntRecipeGate {
                required: "Gargoyle mastery",
            });
        }
        if !self.flynt_ascension.werewolf_mastered {
            return Err(SliceProgressError::FlyntRecipeGate {
                required: "Werewolf mastery",
            });
        }
        if !self.flynt_ascension.merman_mastered {
            return Err(SliceProgressError::FlyntRecipeGate {
                required: "Merman mastery",
            });
        }
        self.flynt_ascension.chimera_synthesized = true;
        Ok(())
    }

    pub fn refine_chimera_form(&mut self) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::CurrentFormUnlocked)?;
        if !self.flynt_ascension.chimera_synthesized {
            return Err(SliceProgressError::FlyntRecipeGate {
                required: "Chimera synthesis",
            });
        }
        self.flynt_ascension.chimera_refined = true;
        Ok(())
    }

    pub fn master_manticorp_form(&mut self) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::CurrentFormUnlocked)?;
        if !self.flynt_ascension.chimera_refined {
            return Err(SliceProgressError::FlyntRecipeGate {
                required: "candidate-specific Chimera refinement",
            });
        }
        self.flynt_ascension.executive_mastery = true;
        Ok(())
    }

    pub fn receive_constitutional_recognition(&mut self) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::CurrentFormUnlocked)?;
        if !self.flynt_ascension.executive_mastery {
            return Err(SliceProgressError::FlyntRecipeGate {
                required: "ExecutiveMastery of Manticorp Form",
            });
        }
        self.flynt_ascension.constitutionally_recognized = true;
        Ok(())
    }

    pub fn complete_lawful_accession(&mut self) -> Result<(), SliceProgressError> {
        self.expect_phase(SlicePhase::CurrentFormUnlocked)?;
        if !self.flynt_ascension.constitutionally_recognized {
            return Err(SliceProgressError::FlyntRecipeGate {
                required: "matching ConstitutionalRecognition",
            });
        }
        self.flynt_ascension.lawfully_acceded = true;
        Ok(())
    }

    fn expect_phase(&self, expected: SlicePhase) -> Result<(), SliceProgressError> {
        if self.phase == expected {
            Ok(())
        } else {
            Err(SliceProgressError::WrongPhase {
                expected,
                actual: self.phase,
            })
        }
    }
}

pub fn build_vertical_slice_follow_up_report(state: &VerticalSliceState) -> String {
    let Some(option) = state
        .resolution_path
        .and_then(|path| state.spec.resolution_option(path))
    else {
        return String::from(
            "# Hueman Slice Follow-Up\n\n\
             ## Status\n\n\
             - no branch has been selected yet\n\
             - complete proof and recognition to unlock a concrete next task\n",
        );
    };

    let unlocked = matches!(
        state.phase,
        SlicePhase::RecognitionEarned | SlicePhase::CurrentFormUnlocked
    );

    if !unlocked {
        return format!(
            "# Hueman Slice Follow-Up\n\n\
             ## Status\n\n\
             - branch selected: {}\n\
             - follow-up task is not unlocked yet\n\
             - complete recognition to unlock `{}`\n",
            option.label, option.follow_up_task_title
        );
    }

    format!(
        "# Hueman Slice Follow-Up\n\n\
         ## Task Status\n\n\
         - branch: {}\n\
         - task: {}\n\
         - phase: {}\n\
         - uses reward: {} x{}\n\
         - trust credential: {}\n\
         - focus: {}\n\
         - summary: {}\n\
         - start action: {}\n\
         - completion result: {}\n",
        option.label,
        option.follow_up_task_title,
        state.follow_up_phase.as_str(),
        option.produced_resource,
        option.produced_resource_units,
        option.recognition_credential,
        option.follow_up_focus,
        option.follow_up_task_summary,
        option.follow_up_task_start,
        option.follow_up_task_completion
    )
}

pub fn build_vertical_slice_progress_report(state: &VerticalSliceState) -> String {
    let selected_option = state
        .resolution_path
        .and_then(|path| state.spec.resolution_option(path));
    let selected_resolution = selected_option
        .map(|option| option.label)
        .unwrap_or("unselected");
    let selected_branch_resource = selected_option
        .map(|option| option.produced_resource)
        .unwrap_or("unselected");
    let selected_credential = selected_option
        .map(|option| {
            if matches!(
                state.phase,
                SlicePhase::RecognitionEarned | SlicePhase::CurrentFormUnlocked
            ) {
                option.recognition_credential
            } else {
                "pending"
            }
        })
        .unwrap_or("pending");
    let resolved_result = selected_option
        .map(|option| option.recognition_result)
        .unwrap_or(state.spec.deployment_result);
    let follow_up_task = selected_option
        .map(|option| {
            if matches!(
                state.phase,
                SlicePhase::RecognitionEarned | SlicePhase::CurrentFormUnlocked
            ) {
                option.follow_up_task_title
            } else {
                "locked until recognition"
            }
        })
        .unwrap_or("locked until recognition");
    let follow_up_phase = if state.resolution_path.is_some() {
        state.follow_up_phase.as_str()
    } else {
        "Locked"
    };
    let flynt_next_gate = if !state.flynt_ascension.gargoyle_mastered {
        "embody Gargoyle"
    } else if !state.flynt_ascension.werewolf_mastered {
        "master Werewolf branch"
    } else if !state.flynt_ascension.merman_mastered {
        "master Merman branch"
    } else if !state.flynt_ascension.chimera_synthesized {
        "synthesize Chimera"
    } else if !state.flynt_ascension.chimera_refined {
        "refine Chimera"
    } else if !state.flynt_ascension.executive_mastery {
        "master Manticorp Form"
    } else if !state.flynt_ascension.constitutionally_recognized {
        "receive ConstitutionalRecognition"
    } else if !state.flynt_ascension.lawfully_acceded {
        "complete LawfulAccession"
    } else {
        "serve in Tross office"
    };
    let active_resolution = selected_option.map_or_else(
        || String::from("- proof gate: unselected\n- clearance gate: unselected\n- field output: unselected\n- credential: pending\n- follow-up focus: unresolved until a branch is selected\n- failure risk: unresolved until a branch is selected\n\n"),
        |option| {
            format!(
                "- proof gate: {}\n- clearance gate: {}\n- field output: {} x{}\n- credential: {}\n- follow-up task: {}\n- follow-up focus: {}\n- failure risk: {}\n\n",
                option.proof_condition,
                option.clearance_condition,
                option.produced_resource,
                option.produced_resource_units,
                if matches!(
                    state.phase,
                    SlicePhase::RecognitionEarned | SlicePhase::CurrentFormUnlocked
                ) {
                    option.recognition_credential
                } else {
                    "pending"
                },
                if matches!(
                    state.phase,
                    SlicePhase::RecognitionEarned | SlicePhase::CurrentFormUnlocked
                ) {
                    option.follow_up_task_title
                } else {
                    "locked until recognition"
                },
                option.follow_up_focus,
                option.failure_condition
            )
        },
    );
    let mut resolution_options = String::new();
    for option in state.spec.resolution_options {
        let marker = if state.resolution_path == Some(option.path) {
            "selected"
        } else {
            "available"
        };
        let _ = fmt::Write::write_fmt(
            &mut resolution_options,
            format_args!(
                "- {} (`{}`): proof={} clearance={} output={} x{} credential={} [{}]\n",
                option.label,
                option.path.as_str(),
                option.proof_condition,
                option.clearance_condition,
                option.produced_resource,
                option.produced_resource_units,
                option.recognition_credential,
                marker
            ),
        );
    }

    format!(
        "# Vertical Slice Progress\n\n\
         ## Slice\n\n\
         - id: `{}`\n\
         - title: {}\n\n\
         ## Current State\n\n\
         - phase: {}\n\
         - tool: {}\n\
         - resolution path: {}\n\
         - branch field resource: {}\n\
         - branch credential: {}\n\
         - follow-up task: {}\n\
         - follow-up phase: {}\n\
         - regular current units: {}\n\
         - holographic aura units: {}\n\
         - signature resource units ({}): {}\n\n\
         - branch output units: {}\n\n\
         ## Active Resolution\n\n\
         {}\
         ## Resolution Paths\n\n\
         {}\
         ## Unlock\n\n\
         - form path: {}\n\
         - node: {}\n\
         - unlocked: {}\n\n\
         ## Flynt Ascension\n\n\
         - Gargoyle embodied: {}\n\
         - Werewolf branch mastered: {}\n\
         - Merman branch mastered: {}\n\
         - Chimera synthesized: {}\n\
         - Chimera refined: {}\n\
         - ExecutiveMastery / Manticorp Form: {}\n\
         - ConstitutionalRecognition: {}\n\
         - LawfulAccession: {}\n\
         - active Tross holder: {}\n\
         - next gate: {}\n\n\
         ## Deployment Goal\n\n\
         - {}\n",
        state.spec.id,
        state.spec.title,
        state.phase.as_str(),
        state.named_tool().unwrap_or("unnamed"),
        selected_resolution,
        selected_branch_resource,
        selected_credential,
        follow_up_task,
        follow_up_phase,
        state.resources.regular_current_units,
        state.resources.holographic_aura_units,
        state.spec.signature_resource,
        state.resources.opal_oil_units,
        state.resources.branch_output_units,
        active_resolution,
        resolution_options,
        state.unlock.current_form,
        state.unlock.node_name,
        state.unlock.unlocked,
        state.flynt_ascension.gargoyle_mastered,
        state.flynt_ascension.werewolf_mastered,
        state.flynt_ascension.merman_mastered,
        state.flynt_ascension.chimera_synthesized,
        state.flynt_ascension.chimera_refined,
        state.flynt_ascension.executive_mastery,
        state.flynt_ascension.constitutionally_recognized,
        state.flynt_ascension.lawfully_acceded,
        state.flynt_ascension.holds_tross_office(),
        flynt_next_gate,
        resolved_result
    )
}

pub fn build_vertical_slice_state_output(state: &VerticalSliceState) -> String {
    let named_tool = state
        .named_tool()
        .map(escape_vertical_slice_value)
        .unwrap_or_else(|| String::from("(unnamed)"));
    let resolution_path = state
        .resolution_path
        .map(|path| path.as_str().to_string())
        .unwrap_or_else(|| String::from("(none)"));

    format!(
        "# Hueman Vertical Slice State\n\
         slice_id: {}\n\
         phase: {}\n\
         follow_up_phase: {}\n\
         regular_current_units: {}\n\
         holographic_aura_units: {}\n\
         opal_oil_units: {}\n\
         branch_output_units: {}\n\
         named_tool: {}\n\
         resolution_path: {}\n\
         unlock_unlocked: {}\n\
         flynt_gargoyle_mastered: {}\n\
         flynt_werewolf_mastered: {}\n\
         flynt_merman_mastered: {}\n\
         flynt_chimera_synthesized: {}\n\
         flynt_chimera_refined: {}\n\
         flynt_executive_mastery: {}\n\
         flynt_constitutionally_recognized: {}\n\
         flynt_lawfully_acceded: {}\n",
        state.spec.id,
        state.phase.as_str(),
        state.follow_up_phase.as_str(),
        state.resources.regular_current_units,
        state.resources.holographic_aura_units,
        state.resources.opal_oil_units,
        state.resources.branch_output_units,
        named_tool,
        resolution_path,
        state.unlock.unlocked,
        state.flynt_ascension.gargoyle_mastered,
        state.flynt_ascension.werewolf_mastered,
        state.flynt_ascension.merman_mastered,
        state.flynt_ascension.chimera_synthesized,
        state.flynt_ascension.chimera_refined,
        state.flynt_ascension.executive_mastery,
        state.flynt_ascension.constitutionally_recognized,
        state.flynt_ascension.lawfully_acceded
    )
}

pub fn write_vertical_slice_artifacts_at(
    root: &std::path::Path,
    state: &VerticalSliceState,
) -> io::Result<(std::path::PathBuf, std::path::PathBuf)> {
    let state_path = root.join(HUEMAN_SLICE_STATE_ARTIFACT_PATH);
    write_text_artifact(&state_path, &build_vertical_slice_state_output(state))?;

    let status_path = root.join(HUEMAN_SLICE_STATUS_ARTIFACT_PATH);
    write_text_artifact(&status_path, &build_vertical_slice_progress_report(state))?;

    Ok((state_path, status_path))
}

pub fn resolve_active_vertical_slice_at(root: &Path) -> io::Result<&'static VerticalSliceSpec> {
    match read_text_artifact(&root.join(HUEMAN_SLICE_STATE_ARTIFACT_PATH)) {
        Ok(contents) => return Ok(parse_vertical_slice_state(&contents)?.spec()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }

    match read_text_artifact(&root.join(CURRENT_SYNTHESIS_TUI_STATE_ARTIFACT_PATH)) {
        Ok(contents) => {
            let persisted = parse_persisted_state(&contents)?;
            return vertical_slice_for_current_synthesis_scenario(&persisted.scenario_id)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "unsupported scenario_id for Hueman slice mapping: {}",
                            persisted.scenario_id
                        ),
                    )
                });
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }

    Ok(primary_vertical_slice())
}

pub fn parse_vertical_slice_state(contents: &str) -> io::Result<VerticalSliceState> {
    let mut slice_id = None;
    let mut phase = None;
    let mut follow_up_phase = None;
    let mut regular_current_units = None;
    let mut holographic_aura_units = None;
    let mut opal_oil_units = None;
    let mut branch_output_units = None;
    let mut named_tool = None;
    let mut resolution_path = None;
    let mut unlock_unlocked = None;
    let mut flynt_gargoyle_mastered = None;
    let mut flynt_werewolf_mastered = None;
    let mut flynt_merman_mastered = None;
    let mut flynt_chimera_synthesized = None;
    let mut flynt_chimera_refined = None;
    let mut flynt_executive_mastery = None;
    let mut flynt_constitutionally_recognized = None;
    let mut flynt_lawfully_acceded = None;
    let mut seen_unknown = Vec::new();

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = line.split_once(':').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("vertical slice state line is missing ':' separator: {line}"),
            )
        })?;
        let key = key.trim();
        let value = value.trim();

        match key {
            "slice_id" => {
                if slice_id.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate slice_id",
                    ));
                }
                slice_id = Some(value.to_string());
            }
            "phase" => {
                if phase.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate phase",
                    ));
                }
                phase = Some(SlicePhase::from_str(value).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid slice phase: {value}"),
                    )
                })?);
            }
            "follow_up_phase" => {
                if follow_up_phase.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate follow_up_phase",
                    ));
                }
                follow_up_phase = Some(FollowUpPhase::from_str(value).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid follow_up_phase: {value}"),
                    )
                })?);
            }
            "regular_current_units" => {
                if regular_current_units.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate regular_current_units",
                    ));
                }
                regular_current_units = Some(parse_slice_u8(value, key)?);
            }
            "holographic_aura_units" => {
                if holographic_aura_units.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate holographic_aura_units",
                    ));
                }
                holographic_aura_units = Some(parse_slice_u8(value, key)?);
            }
            "opal_oil_units" => {
                if opal_oil_units.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate opal_oil_units",
                    ));
                }
                opal_oil_units = Some(parse_slice_u8(value, key)?);
            }
            "branch_output_units" => {
                if branch_output_units.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate branch_output_units",
                    ));
                }
                branch_output_units = Some(parse_slice_u8(value, key)?);
            }
            "named_tool" => {
                if named_tool.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate named_tool",
                    ));
                }
                named_tool = Some(if value == "(unnamed)" {
                    None
                } else {
                    Some(unescape_vertical_slice_value(value))
                });
            }
            "resolution_path" => {
                if resolution_path.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate resolution_path",
                    ));
                }
                resolution_path = Some(if value == "(none)" {
                    None
                } else {
                    Some(SliceResolutionPath::from_str(value).ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("invalid resolution_path: {value}"),
                        )
                    })?)
                });
            }
            "unlock_unlocked" => {
                if unlock_unlocked.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate unlock_unlocked",
                    ));
                }
                unlock_unlocked = Some(parse_slice_bool(value, key)?);
            }
            "flynt_gargoyle_mastered" => {
                if flynt_gargoyle_mastered.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate flynt_gargoyle_mastered",
                    ));
                }
                flynt_gargoyle_mastered = Some(parse_slice_bool(value, key)?);
            }
            "flynt_werewolf_mastered" => {
                if flynt_werewolf_mastered.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate flynt_werewolf_mastered",
                    ));
                }
                flynt_werewolf_mastered = Some(parse_slice_bool(value, key)?);
            }
            "flynt_merman_mastered" => {
                if flynt_merman_mastered.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate flynt_merman_mastered",
                    ));
                }
                flynt_merman_mastered = Some(parse_slice_bool(value, key)?);
            }
            "flynt_chimera_synthesized" => {
                if flynt_chimera_synthesized.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate flynt_chimera_synthesized",
                    ));
                }
                flynt_chimera_synthesized = Some(parse_slice_bool(value, key)?);
            }
            "flynt_chimera_refined" => {
                if flynt_chimera_refined.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate flynt_chimera_refined",
                    ));
                }
                flynt_chimera_refined = Some(parse_slice_bool(value, key)?);
            }
            "flynt_executive_mastery" => {
                if flynt_executive_mastery.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate flynt_executive_mastery",
                    ));
                }
                flynt_executive_mastery = Some(parse_slice_bool(value, key)?);
            }
            "flynt_constitutionally_recognized" => {
                if flynt_constitutionally_recognized.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate flynt_constitutionally_recognized",
                    ));
                }
                flynt_constitutionally_recognized = Some(parse_slice_bool(value, key)?);
            }
            "flynt_lawfully_acceded" => {
                if flynt_lawfully_acceded.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "vertical slice state contains duplicate flynt_lawfully_acceded",
                    ));
                }
                flynt_lawfully_acceded = Some(parse_slice_bool(value, key)?);
            }
            other => seen_unknown.push(other.to_owned()),
        }
    }

    if let Some(unknown_key) = seen_unknown.first() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("vertical slice state contains unknown key: {unknown_key}"),
        ));
    }

    let slice_id = slice_id.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "vertical slice state missing slice_id",
        )
    })?;
    let spec = vertical_slice_by_id(&slice_id).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unsupported slice_id: {slice_id}"),
        )
    })?;

    let phase = phase.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "vertical slice state missing phase",
        )
    })?;
    let follow_up_phase = follow_up_phase.unwrap_or_else(|| {
        if matches!(phase, SlicePhase::CurrentFormUnlocked) {
            FollowUpPhase::Ready
        } else {
            FollowUpPhase::Locked
        }
    });
    let named_tool = named_tool.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "vertical slice state missing named_tool",
        )
    })?;
    let resolution_path = resolution_path.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "vertical slice state missing resolution_path",
        )
    })?;
    let branch_output_units = branch_output_units.unwrap_or_else(|| {
        if matches!(
            phase,
            SlicePhase::RecognitionEarned | SlicePhase::CurrentFormUnlocked
        ) {
            resolution_path
                .and_then(|path| spec.resolution_option(path))
                .map(|option| option.produced_resource_units)
                .unwrap_or(0)
        } else {
            0
        }
    });
    let unlock_unlocked = unlock_unlocked.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "vertical slice state missing unlock_unlocked",
        )
    })?;
    let flynt_ascension = FlyntAscensionState {
        gargoyle_mastered: flynt_gargoyle_mastered.unwrap_or(false),
        werewolf_mastered: flynt_werewolf_mastered.unwrap_or(false),
        merman_mastered: flynt_merman_mastered.unwrap_or(false),
        chimera_synthesized: flynt_chimera_synthesized.unwrap_or(false),
        chimera_refined: flynt_chimera_refined.unwrap_or(false),
        executive_mastery: flynt_executive_mastery.unwrap_or(false),
        constitutionally_recognized: flynt_constitutionally_recognized.unwrap_or(false),
        lawfully_acceded: flynt_lawfully_acceded.unwrap_or(false),
    };

    if phase.requires_named_tool() && named_tool.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("phase {} requires named_tool", phase.as_str()),
        ));
    }
    if !phase.requires_named_tool() && named_tool.is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("phase {} must not carry a named_tool yet", phase.as_str()),
        ));
    }
    if phase.requires_resolution_path() && resolution_path.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("phase {} requires resolution_path", phase.as_str()),
        ));
    }
    if !phase.requires_resolution_path() && resolution_path.is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "phase {} must not carry a resolution_path yet",
                phase.as_str()
            ),
        ));
    }
    if matches!(
        phase,
        SlicePhase::NeedObserved
            | SlicePhase::SeamSurveyed
            | SlicePhase::InputsGathered
            | SlicePhase::OpalOilRefined
            | SlicePhase::ToolNamed
            | SlicePhase::ToolProven
            | SlicePhase::ToolCleared
            | SlicePhase::ToolDeployed
    ) && branch_output_units != 0
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("phase {} must not carry branch output yet", phase.as_str()),
        ));
    }
    if matches!(
        phase,
        SlicePhase::RecognitionEarned | SlicePhase::CurrentFormUnlocked
    ) && resolution_path
        .and_then(|path| spec.resolution_option(path))
        .is_some_and(|option| branch_output_units != option.produced_resource_units)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "branch_output_units must match the selected resolution reward",
        ));
    }
    if unlock_unlocked != matches!(phase, SlicePhase::CurrentFormUnlocked) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unlock_unlocked must match the CurrentFormUnlocked phase",
        ));
    }
    if !matches!(phase, SlicePhase::CurrentFormUnlocked) && follow_up_phase != FollowUpPhase::Locked
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "follow_up_phase must stay Locked until the first current-form unlock",
        ));
    }
    if phase != SlicePhase::CurrentFormUnlocked
        && (flynt_ascension.gargoyle_mastered
            || flynt_ascension.werewolf_mastered
            || flynt_ascension.merman_mastered
            || flynt_ascension.chimera_synthesized
            || flynt_ascension.chimera_refined
            || flynt_ascension.executive_mastery
            || flynt_ascension.constitutionally_recognized
            || flynt_ascension.lawfully_acceded)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Flynt ascension state requires the CurrentFormUnlocked phase",
        ));
    }
    if !flynt_ascension.gargoyle_mastered
        && (flynt_ascension.werewolf_mastered
            || flynt_ascension.merman_mastered
            || flynt_ascension.chimera_synthesized
            || flynt_ascension.chimera_refined
            || flynt_ascension.executive_mastery
            || flynt_ascension.constitutionally_recognized
            || flynt_ascension.lawfully_acceded)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Flynt ascension requires Gargoyle mastery before later forms",
        ));
    }
    if flynt_ascension.chimera_synthesized
        && !(flynt_ascension.gargoyle_mastered
            && flynt_ascension.werewolf_mastered
            && flynt_ascension.merman_mastered)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Chimera synthesis requires Gargoyle, Werewolf, and Merman mastery",
        ));
    }
    if flynt_ascension.chimera_refined && !flynt_ascension.chimera_synthesized {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Chimera refinement requires Chimera synthesis first",
        ));
    }
    if flynt_ascension.executive_mastery && !flynt_ascension.chimera_refined {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "ExecutiveMastery requires candidate-specific Chimera refinement first",
        ));
    }
    if flynt_ascension.constitutionally_recognized && !flynt_ascension.executive_mastery {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "ConstitutionalRecognition requires existing ExecutiveMastery",
        ));
    }
    if flynt_ascension.lawfully_acceded && !flynt_ascension.constitutionally_recognized {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "LawfulAccession requires matching ConstitutionalRecognition",
        ));
    }

    Ok(VerticalSliceState {
        spec,
        phase,
        resources: SliceResourceLedger {
            regular_current_units: regular_current_units.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "vertical slice state missing regular_current_units",
                )
            })?,
            holographic_aura_units: holographic_aura_units.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "vertical slice state missing holographic_aura_units",
                )
            })?,
            opal_oil_units: opal_oil_units.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "vertical slice state missing opal_oil_units",
                )
            })?,
            branch_output_units,
        },
        named_tool,
        resolution_path,
        unlock: SliceUnlockState {
            current_form: spec.current_form,
            node_name: spec.unlock_node,
            unlocked: unlock_unlocked,
        },
        follow_up_phase,
        flynt_ascension,
    })
}

fn escape_vertical_slice_value(value: &str) -> String {
    value.replace('\\', "\\\\").replace('\n', "\\n")
}

fn unescape_vertical_slice_value(value: &str) -> String {
    let mut output = String::new();
    let mut chars = value.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => output.push('\n'),
                Some('\\') => output.push('\\'),
                Some(other) => {
                    output.push('\\');
                    output.push(other);
                }
                None => output.push('\\'),
            }
        } else {
            output.push(ch);
        }
    }

    output
}

fn parse_slice_bool(value: &str, field: &str) -> io::Result<bool> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid bool for {field}: {value}"),
        )),
    }
}

fn parse_slice_u8(value: &str, field: &str) -> io::Result<u8> {
    value.parse::<u8>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid u8 for {field}: {value}"),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{
        SlicePhase, SliceProgressError, VerticalSliceState, build_vertical_slice_progress_report,
        build_vertical_slice_state_output, parse_vertical_slice_state,
        resolve_active_vertical_slice_at,
    };
    use crate::hueman_slice::SliceResolutionPath;
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nonce}"))
    }

    #[test]
    fn primary_slice_progress_requires_ordered_transitions() {
        let mut state = VerticalSliceState::primary();

        assert_eq!(state.phase(), SlicePhase::NeedObserved);
        assert!(matches!(
            state.refine_opal_oil(),
            Err(SliceProgressError::WrongPhase {
                expected: SlicePhase::InputsGathered,
                actual: SlicePhase::NeedObserved
            })
        ));

        state
            .survey_safe_seam()
            .expect("survey should advance the slice");
        state
            .gather_inputs(2, 2)
            .expect("input gathering should advance the slice");
        state
            .refine_opal_oil()
            .expect("refinement should advance the slice");

        assert_eq!(state.phase(), SlicePhase::OpalOilRefined);
        assert_eq!(state.resources().opal_oil_units, 1);
        assert_eq!(state.resources().branch_output_units, 0);
    }

    #[test]
    fn primary_slice_rejects_the_wrong_tool_name() {
        let mut state = VerticalSliceState::primary();
        state.survey_safe_seam().expect("survey should succeed");
        state
            .gather_inputs(2, 2)
            .expect("input gathering should succeed");
        state.refine_opal_oil().expect("refinement should succeed");

        let error = state
            .name_tool("Wrong Drill")
            .expect_err("tool naming should reject a non-canonical name");

        assert!(matches!(
            error,
            SliceProgressError::ToolNameMismatch {
                expected: "Ridge Lantern Drill",
                ..
            }
        ));
    }

    #[test]
    fn primary_slice_unlocks_gremlin_only_after_recognition() {
        let mut state = VerticalSliceState::primary();
        state.survey_safe_seam().expect("survey should succeed");
        state
            .gather_inputs(2, 2)
            .expect("input gathering should succeed");
        state.refine_opal_oil().expect("refinement should succeed");
        state
            .name_tool("Ridge Lantern Drill")
            .expect("tool naming should succeed");
        state
            .prove_tool_for(SliceResolutionPath::FlockDefense)
            .expect("proof should succeed");
        state.clear_tool().expect("clearance should succeed");
        state
            .deploy_tool_for(SliceResolutionPath::FlockDefense)
            .expect("deployment should succeed");
        state
            .recognize_result()
            .expect("recognition should succeed");
        state
            .unlock_first_current_form_node()
            .expect("unlock should succeed after recognition");

        assert_eq!(state.phase(), SlicePhase::CurrentFormUnlocked);
        assert!(state.unlock().unlocked);
        assert_eq!(state.unlock().node_name, "Load-Bearing Grip");
        assert_eq!(
            state.resolution_path(),
            Some(SliceResolutionPath::FlockDefense)
        );
        assert_eq!(state.resources().branch_output_units, 1);
    }

    #[test]
    fn flynt_ascension_requires_gargoyle_before_branch_mastery() {
        let mut state = VerticalSliceState::primary();
        state.survey_safe_seam().expect("survey should succeed");
        state
            .gather_inputs(2, 2)
            .expect("input gathering should succeed");
        state.refine_opal_oil().expect("refinement should succeed");
        state
            .name_tool("Ridge Lantern Drill")
            .expect("tool naming should succeed");
        state.prove_tool().expect("proof should succeed");
        state.clear_tool().expect("clearance should succeed");
        state.deploy_tool().expect("deployment should succeed");
        state
            .recognize_result()
            .expect("recognition should succeed");
        state
            .unlock_first_current_form_node()
            .expect("unlock should succeed after recognition");

        let error = state
            .master_werewolf_branch()
            .expect_err("werewolf branch should require Gargoyle first");

        assert_eq!(
            error.to_string(),
            "Flynt ascension requires Gargoyle mastery first"
        );
    }

    #[test]
    fn flynt_ascension_tracks_the_full_constitutional_sequence() {
        let mut state = VerticalSliceState::primary();
        state.survey_safe_seam().expect("survey should succeed");
        state
            .gather_inputs(2, 2)
            .expect("input gathering should succeed");
        state.refine_opal_oil().expect("refinement should succeed");
        state
            .name_tool("Ridge Lantern Drill")
            .expect("tool naming should succeed");
        state.prove_tool().expect("proof should succeed");
        state.clear_tool().expect("clearance should succeed");
        state.deploy_tool().expect("deployment should succeed");
        state
            .recognize_result()
            .expect("recognition should succeed");
        state
            .unlock_first_current_form_node()
            .expect("unlock should succeed after recognition");

        state
            .embody_gargoyle_form()
            .expect("Gargoyle should unlock after Gremlin");
        state
            .master_werewolf_branch()
            .expect("Werewolf should unlock after Gargoyle");
        state
            .master_merman_branch()
            .expect("Merman should unlock after Gargoyle");
        state
            .synthesize_chimera_form()
            .expect("Chimera should require both branch masteries");
        state
            .refine_chimera_form()
            .expect("refinement should require Chimera first");
        state
            .master_manticorp_form()
            .expect("Manticorp Form should require refinement first");

        assert!(!state.flynt_ascension().holds_tross_office());
        state
            .receive_constitutional_recognition()
            .expect("recognition should acknowledge existing mastery");
        assert!(!state.flynt_ascension().holds_tross_office());
        state
            .complete_lawful_accession()
            .expect("accession should require matching recognition");

        let ascension = state.flynt_ascension();
        assert!(ascension.gargoyle_mastered);
        assert!(ascension.werewolf_mastered);
        assert!(ascension.merman_mastered);
        assert!(ascension.chimera_synthesized);
        assert!(ascension.chimera_refined);
        assert!(ascension.executive_mastery);
        assert!(ascension.constitutionally_recognized);
        assert!(ascension.lawfully_acceded);
        assert!(ascension.holds_tross_office());
    }

    #[test]
    fn progress_report_reflects_current_state() {
        let state = VerticalSliceState::primary();
        let report = build_vertical_slice_progress_report(&state);

        assert!(report.starts_with("# Vertical Slice Progress"));
        assert!(report.contains("phase: NeedObserved"));
        assert!(report.contains("node: Load-Bearing Grip"));
        assert!(report.contains("Aura Ridge Opal Oil Starter Loop"));
        assert!(report.contains("proof gate: unselected"));
        assert!(report.contains("branch field resource: unselected"));
        assert!(report.contains("Route Stabilization (`route`)"));
        assert!(report.contains("Flock Defense (`defense`)"));
        assert!(report.contains("## Flynt Ascension"));
        assert!(report.contains("next gate: embody Gargoyle"));
    }

    #[test]
    fn state_output_and_parser_remain_compatible() {
        let mut state = VerticalSliceState::primary();
        state.survey_safe_seam().expect("survey should succeed");
        state
            .gather_inputs(2, 2)
            .expect("input gathering should succeed");
        state.refine_opal_oil().expect("refinement should succeed");
        state
            .name_tool("Ridge Lantern Drill")
            .expect("tool naming should succeed");
        state
            .prove_tool_for(SliceResolutionPath::FlockDefense)
            .expect("proof should succeed");
        state.clear_tool().expect("clearance should succeed");
        state
            .deploy_tool_for(SliceResolutionPath::FlockDefense)
            .expect("deployment should succeed");
        state
            .recognize_result()
            .expect("recognition should succeed");

        let output = build_vertical_slice_state_output(&state);
        let parsed = parse_vertical_slice_state(&output).expect("state output should parse");

        assert_eq!(parsed, state);
    }

    #[test]
    fn state_parser_rejects_unlock_phase_mismatch() {
        let error = parse_vertical_slice_state(&format!(
            "# Hueman Vertical Slice State\n\
             slice_id: {}\n\
             phase: CurrentFormUnlocked\n\
             regular_current_units: 0\n\
             holographic_aura_units: 0\n\
             opal_oil_units: 0\n\
             branch_output_units: 1\n\
             named_tool: Ridge Lantern Drill\n\
             resolution_path: defense\n\
             unlock_unlocked: false\n",
            VerticalSliceState::primary().spec().id
        ))
        .expect_err("unlock mismatch should fail");

        assert_eq!(
            error.to_string(),
            "unlock_unlocked must match the CurrentFormUnlocked phase"
        );
    }

    #[test]
    fn state_parser_rejects_resolution_path_before_deployment() {
        let error = parse_vertical_slice_state(&format!(
            "# Hueman Vertical Slice State\n\
             slice_id: {}\n\
             phase: ToolNamed\n\
             regular_current_units: 0\n\
             holographic_aura_units: 0\n\
             opal_oil_units: 1\n\
             branch_output_units: 0\n\
             named_tool: Ridge Lantern Drill\n\
             resolution_path: route\n\
             unlock_unlocked: false\n",
            VerticalSliceState::primary().spec().id
        ))
        .expect_err("premature resolution path should fail");

        assert_eq!(
            error.to_string(),
            "phase ToolNamed must not carry a resolution_path yet"
        );
    }

    #[test]
    fn state_parser_rejects_branch_output_before_recognition() {
        let error = parse_vertical_slice_state(&format!(
            "# Hueman Vertical Slice State\n\
             slice_id: {}\n\
             phase: ToolCleared\n\
             regular_current_units: 0\n\
             holographic_aura_units: 0\n\
             opal_oil_units: 1\n\
             branch_output_units: 1\n\
             named_tool: Ridge Lantern Drill\n\
             resolution_path: defense\n\
             unlock_unlocked: false\n",
            VerticalSliceState::primary().spec().id
        ))
        .expect_err("premature branch output should fail");

        assert_eq!(
            error.to_string(),
            "phase ToolCleared must not carry branch output yet"
        );
    }

    #[test]
    fn state_parser_backfills_missing_branch_output_units_for_older_completed_states() {
        let parsed = parse_vertical_slice_state(&format!(
            "# Hueman Vertical Slice State\n\
             slice_id: {}\n\
             phase: GremlinUnlocked\n\
             regular_current_units: 0\n\
             holographic_aura_units: 0\n\
             opal_oil_units: 1\n\
             named_tool: Ridge Lantern Drill\n\
             resolution_path: defense\n\
             unlock_unlocked: true\n",
            VerticalSliceState::primary().spec().id
        ))
        .expect("older state contract should migrate");

        assert_eq!(parsed.resources().branch_output_units, 1);
        assert_eq!(
            parsed.resolution_path(),
            Some(SliceResolutionPath::FlockDefense)
        );
        assert!(!parsed.flynt_ascension().gargoyle_mastered);
    }

    #[test]
    fn state_parser_rejects_flynt_branch_mastery_before_gargoyle() {
        let error = parse_vertical_slice_state(&format!(
            "# Hueman Vertical Slice State\n\
             slice_id: {}\n\
             phase: GremlinUnlocked\n\
             follow_up_phase: Ready\n\
             regular_current_units: 0\n\
             holographic_aura_units: 0\n\
             opal_oil_units: 1\n\
             branch_output_units: 1\n\
             named_tool: Ridge Lantern Drill\n\
             resolution_path: defense\n\
             unlock_unlocked: true\n\
             flynt_gargoyle_mastered: false\n\
             flynt_werewolf_mastered: true\n\
             flynt_merman_mastered: false\n\
             flynt_chimera_synthesized: false\n\
             flynt_chimera_refined: false\n\
             flynt_executive_mastery: false\n\
             flynt_constitutionally_recognized: false\n\
             flynt_lawfully_acceded: false\n",
            VerticalSliceState::primary().spec().id
        ))
        .expect_err("later Flynt branch mastery should require Gargoyle first");

        assert_eq!(
            error.to_string(),
            "Flynt ascension requires Gargoyle mastery before later forms"
        );
    }

    #[test]
    fn deployment_rejects_switching_branches_after_proof() {
        let mut state = VerticalSliceState::primary();
        state.survey_safe_seam().expect("survey should succeed");
        state
            .gather_inputs(2, 2)
            .expect("input gathering should succeed");
        state.refine_opal_oil().expect("refinement should succeed");
        state
            .name_tool("Ridge Lantern Drill")
            .expect("tool naming should succeed");
        state
            .prove_tool_for(SliceResolutionPath::FlockDefense)
            .expect("defense proof should succeed");
        state.clear_tool().expect("clearance should succeed");

        let error = state
            .deploy_tool_for(SliceResolutionPath::RouteStabilization)
            .expect_err("deploy should reject branch mismatch");

        assert!(matches!(
            error,
            SliceProgressError::ResolutionPathMismatch {
                expected: SliceResolutionPath::FlockDefense,
                actual: SliceResolutionPath::RouteStabilization
            }
        ));
    }

    #[test]
    fn active_slice_resolver_follows_current_synthesis_when_slice_state_is_missing() {
        let root = unique_temp_dir("hollow-grove-hueman-progression-active-slice");
        fs::create_dir_all(root.join("artifacts")).expect("artifact dir should create");
        fs::write(
            root.join("artifacts/current_synthesis_tui_state.txt"),
            "# Current Synthesis TUI State\nscenario_id: flooded_quarry_night_watch\nseed: 7\ncompleted_ticks: 1\nfocused_npc_id: quarry_foreman_01\n",
        )
        .expect("current synthesis state should write");

        let slice = resolve_active_vertical_slice_at(&root).expect("slice should resolve");

        assert_eq!(slice.id, "flooded_quarry_spillrail_latch");
        fs::remove_dir_all(&root).expect("cleanup should succeed");
    }
}
