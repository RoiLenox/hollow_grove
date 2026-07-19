//! Presentation-neutral event contract for future terminal clients.

use std::collections::BTreeMap;
use std::fmt;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuiCommand {
    Catalog,
    RunScenario { scenario: String },
    ReplayScenario { scenario: String },
    PersistScenario { scenario: String },
    MigrateScenario { scenario: String },
    InspectTrace { scenario: String },
    InspectAuthority { scenario: String },
    InspectEvidence { scenario: String },
    InspectPolarity { scenario: String },
    InspectLineage { scenario: String },
    InspectRegion { scenario: String },
    InspectStewardship { scenario: String },
    InspectGuardianship { scenario: String },
    Audit,
    Cancel { request_id: String },
}

impl TuiCommand {
    #[must_use]
    pub const fn is_read_only(&self) -> bool {
        !matches!(self, Self::RunScenario { .. } | Self::Cancel { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TuiEventKind {
    SessionStarted,
    CatalogEntry,
    CatalogCompleted,
    ScenarioStarted,
    TransitionProposed,
    AuthorityChecked,
    EvidenceChecked,
    PolarityObserved,
    TransitionAccepted,
    TransitionRejected,
    StateChanged,
    Persisted,
    ReplayStarted,
    ReplayCompleted,
    MigrationStarted,
    MigrationCompleted,
    WaveCreated,
    RegionEntered,
    SynthesisProposed,
    SynthesisAccepted,
    SynthesisRejected,
    LineagePreserved,
    StewardshipGranted,
    GuardianshipGranted,
    AuraFieldsAssigned,
    AuraBeachAssigned,
    AuraSeaGuardianshipAssigned,
    ScenarioCompleted,
    AuditCompleted,
    CancellationAccepted,
    RequestCancelled,
}

impl TuiEventKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionStarted => "SessionStarted",
            Self::CatalogEntry => "CatalogEntry",
            Self::CatalogCompleted => "CatalogCompleted",
            Self::ScenarioStarted => "ScenarioStarted",
            Self::TransitionProposed => "TransitionProposed",
            Self::AuthorityChecked => "AuthorityChecked",
            Self::EvidenceChecked => "EvidenceChecked",
            Self::PolarityObserved => "PolarityObserved",
            Self::TransitionAccepted => "TransitionAccepted",
            Self::TransitionRejected => "TransitionRejected",
            Self::StateChanged => "StateChanged",
            Self::Persisted => "Persisted",
            Self::ReplayStarted => "ReplayStarted",
            Self::ReplayCompleted => "ReplayCompleted",
            Self::MigrationStarted => "MigrationStarted",
            Self::MigrationCompleted => "MigrationCompleted",
            Self::WaveCreated => "WaveCreated",
            Self::RegionEntered => "RegionEntered",
            Self::SynthesisProposed => "SynthesisProposed",
            Self::SynthesisAccepted => "SynthesisAccepted",
            Self::SynthesisRejected => "SynthesisRejected",
            Self::LineagePreserved => "LineagePreserved",
            Self::StewardshipGranted => "StewardshipGranted",
            Self::GuardianshipGranted => "GuardianshipGranted",
            Self::AuraFieldsAssigned => "AuraFieldsAssigned",
            Self::AuraBeachAssigned => "AuraBeachAssigned",
            Self::AuraSeaGuardianshipAssigned => "AuraSeaGuardianshipAssigned",
            Self::ScenarioCompleted => "ScenarioCompleted",
            Self::AuditCompleted => "AuditCompleted",
            Self::CancellationAccepted => "CancellationAccepted",
            Self::RequestCancelled => "RequestCancelled",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "SessionStarted" => Self::SessionStarted,
            "CatalogEntry" => Self::CatalogEntry,
            "CatalogCompleted" => Self::CatalogCompleted,
            "ScenarioStarted" => Self::ScenarioStarted,
            "TransitionProposed" => Self::TransitionProposed,
            "AuthorityChecked" => Self::AuthorityChecked,
            "EvidenceChecked" => Self::EvidenceChecked,
            "PolarityObserved" => Self::PolarityObserved,
            "TransitionAccepted" => Self::TransitionAccepted,
            "TransitionRejected" => Self::TransitionRejected,
            "StateChanged" => Self::StateChanged,
            "Persisted" => Self::Persisted,
            "ReplayStarted" => Self::ReplayStarted,
            "ReplayCompleted" => Self::ReplayCompleted,
            "MigrationStarted" => Self::MigrationStarted,
            "MigrationCompleted" => Self::MigrationCompleted,
            "WaveCreated" => Self::WaveCreated,
            "RegionEntered" => Self::RegionEntered,
            "SynthesisProposed" => Self::SynthesisProposed,
            "SynthesisAccepted" => Self::SynthesisAccepted,
            "SynthesisRejected" => Self::SynthesisRejected,
            "LineagePreserved" => Self::LineagePreserved,
            "StewardshipGranted" => Self::StewardshipGranted,
            "GuardianshipGranted" => Self::GuardianshipGranted,
            "AuraFieldsAssigned" => Self::AuraFieldsAssigned,
            "AuraBeachAssigned" => Self::AuraBeachAssigned,
            "AuraSeaGuardianshipAssigned" => Self::AuraSeaGuardianshipAssigned,
            "ScenarioCompleted" => Self::ScenarioCompleted,
            "AuditCompleted" => Self::AuditCompleted,
            "CancellationAccepted" => Self::CancellationAccepted,
            "RequestCancelled" => Self::RequestCancelled,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuiEvent {
    pub sequence: u64,
    pub scenario: String,
    pub kind: TuiEventKind,
    pub fields: BTreeMap<String, String>,
}

impl TuiEvent {
    /// Stable tab-separated wire record with escaped keys and values.
    #[must_use]
    pub fn encode_line(&self) -> String {
        let mut values = vec![
            self.sequence.to_string(),
            escape(&self.scenario),
            self.kind.as_str().to_owned(),
        ];
        values.extend(
            self.fields
                .iter()
                .map(|(key, value)| format!("{}={}", escape(key), escape(value))),
        );
        values.join("\t")
    }

    pub fn decode_line(line: &str) -> Result<Self, TuiWireError> {
        let mut parts = line.split('\t');
        let sequence = parts
            .next()
            .ok_or(TuiWireError::MissingField("sequence"))?
            .parse()
            .map_err(|_| TuiWireError::InvalidSequence)?;
        let scenario = unescape(parts.next().ok_or(TuiWireError::MissingField("scenario"))?)?;
        let kind_value = parts.next().ok_or(TuiWireError::MissingField("kind"))?;
        let kind = TuiEventKind::parse(kind_value)
            .ok_or_else(|| TuiWireError::UnknownKind(kind_value.to_owned()))?;
        let mut fields = BTreeMap::new();
        for part in parts {
            let (key, value) = part
                .split_once('=')
                .ok_or_else(|| TuiWireError::InvalidField(part.to_owned()))?;
            let key = unescape(key)?;
            if fields.insert(key.clone(), unescape(value)?).is_some() {
                return Err(TuiWireError::DuplicateField(key));
            }
        }
        Ok(Self {
            sequence,
            scenario,
            kind,
            fields,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuiWireError {
    MissingField(&'static str),
    InvalidSequence,
    UnknownKind(String),
    InvalidField(String),
    DuplicateField(String),
    InvalidEscape,
}

impl fmt::Display for TuiWireError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "TUI event wire error: {self:?}")
    }
}

impl std::error::Error for TuiWireError {}

fn escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
        .replace('=', "\\e")
}

fn unescape(value: &str) -> Result<String, TuiWireError> {
    let mut output = String::new();
    let mut chars = value.chars();
    while let Some(character) = chars.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }
        output.push(match chars.next().ok_or(TuiWireError::InvalidEscape)? {
            '\\' => '\\',
            't' => '\t',
            'n' => '\n',
            'e' => '=',
            _ => return Err(TuiWireError::InvalidEscape),
        });
    }
    Ok(output)
}

/// Projects trace output into deterministic presentation-neutral events.
#[must_use]
pub fn tui_events_from_trace(trace: &ConstitutionalTrace) -> Vec<TuiEvent> {
    let mut events = vec![TuiEvent {
        sequence: 0,
        scenario: trace.scenario.clone(),
        kind: TuiEventKind::ScenarioStarted,
        fields: BTreeMap::new(),
    }];
    for transition in &trace.transitions {
        let mut fields = BTreeMap::from([
            ("command".into(), transition.command.into()),
            ("event_id".into(), transition.event_id.clone()),
            ("prior_state".into(), transition.prior_state.clone()),
            ("proposed_state".into(), transition.proposed_state.clone()),
        ]);
        if let Some(region) = transition.region {
            fields.insert("region".into(), region.as_str().into());
        }
        if let Some(predecessor) = &transition.predecessor_being {
            fields.insert("predecessor".into(), predecessor.clone());
        }
        if let Some(result) = &transition.resulting_being {
            fields.insert("result".into(), result.clone());
        }
        if let Some(rule) = transition.synthesis_rule {
            fields.insert("synthesis_rule".into(), rule.into());
        }
        if let Some(function) = transition.regional_function {
            fields.insert("regional_function".into(), function.as_str().into());
        }
        if !transition.stewardship.is_empty()
            && transition.region == Some(ConstitutionalRegion::AuraFields)
        {
            fields.insert("stewardship".into(), transition.stewardship.join("; "));
        }
        if !transition.guardianship.is_empty() {
            fields.insert("guardianship".into(), transition.guardianship.join("; "));
        }
        if let Some(failure) = &transition.failure_code {
            fields.insert("failure".into(), failure.clone());
        }
        if let Some(house) = transition.acting_house {
            fields.insert("house".into(), format!("{house:?}"));
        }
        if let Some(institution) = &transition.acting_institution {
            fields.insert("institution".into(), institution.clone());
        }
        if !transition.authority.is_empty() {
            fields.insert("authority".into(), transition.authority.join("; "));
        }
        if !transition.evidence.is_empty() {
            fields.insert("evidence".into(), transition.evidence.join("; "));
        }
        let synthesis =
            transition.synthesis_rule.is_some() || transition.command == "SYNTHESIZE_BEING";
        events.push(TuiEvent {
            sequence: u64::try_from(events.len()).unwrap_or(u64::MAX),
            scenario: trace.scenario.clone(),
            kind: if synthesis {
                TuiEventKind::SynthesisProposed
            } else {
                TuiEventKind::TransitionProposed
            },
            fields: fields.clone(),
        });
        if transition.acting_house.is_some()
            || transition.acting_institution.is_some()
            || !transition.authority.is_empty()
        {
            events.push(TuiEvent {
                sequence: u64::try_from(events.len()).unwrap_or(u64::MAX),
                scenario: trace.scenario.clone(),
                kind: TuiEventKind::AuthorityChecked,
                fields: fields.clone(),
            });
        }
        if !transition.evidence.is_empty() {
            events.push(TuiEvent {
                sequence: u64::try_from(events.len()).unwrap_or(u64::MAX),
                scenario: trace.scenario.clone(),
                kind: TuiEventKind::EvidenceChecked,
                fields: fields.clone(),
            });
        }
        events.push(TuiEvent {
            sequence: u64::try_from(events.len()).unwrap_or(u64::MAX),
            scenario: trace.scenario.clone(),
            kind: if transition.command == "CREATE_WAVE"
                && transition.disposition == TraceDisposition::Accepted
            {
                TuiEventKind::WaveCreated
            } else if synthesis {
                if transition.disposition == TraceDisposition::Accepted {
                    TuiEventKind::SynthesisAccepted
                } else {
                    TuiEventKind::SynthesisRejected
                }
            } else if transition.disposition == TraceDisposition::Accepted {
                TuiEventKind::TransitionAccepted
            } else {
                TuiEventKind::TransitionRejected
            },
            fields,
        });
        if transition.disposition == TraceDisposition::Accepted && transition.state_changed {
            events.push(TuiEvent {
                sequence: u64::try_from(events.len()).unwrap_or(u64::MAX),
                scenario: trace.scenario.clone(),
                kind: TuiEventKind::StateChanged,
                fields: BTreeMap::from([
                    ("prior_state".into(), transition.prior_state.clone()),
                    ("next_state".into(), transition.proposed_state.clone()),
                ]),
            });
        }
        if transition.disposition == TraceDisposition::Accepted
            && transition.predecessor_being.is_some()
            && transition.resulting_being.is_some()
        {
            let mut lineage = BTreeMap::new();
            lineage.insert(
                "predecessor".into(),
                transition.predecessor_being.clone().unwrap_or_default(),
            );
            lineage.insert(
                "result".into(),
                transition.resulting_being.clone().unwrap_or_default(),
            );
            events.push(TuiEvent {
                sequence: u64::try_from(events.len()).unwrap_or(u64::MAX),
                scenario: trace.scenario.clone(),
                kind: TuiEventKind::LineagePreserved,
                fields: lineage,
            });
        }
        if !transition.stewardship.is_empty()
            && transition.region == Some(ConstitutionalRegion::AuraFields)
        {
            events.push(TuiEvent {
                sequence: u64::try_from(events.len()).unwrap_or(u64::MAX),
                scenario: trace.scenario.clone(),
                kind: TuiEventKind::AuraFieldsAssigned,
                fields: BTreeMap::from([("region".into(), "Aura Fields".into())]),
            });
            events.push(TuiEvent {
                sequence: u64::try_from(events.len()).unwrap_or(u64::MAX),
                scenario: trace.scenario.clone(),
                kind: TuiEventKind::StewardshipGranted,
                fields: BTreeMap::from([("duties".into(), transition.stewardship.join("; "))]),
            });
        }
        if !transition.stewardship.is_empty()
            && transition.region == Some(ConstitutionalRegion::AuraBeach)
        {
            events.push(TuiEvent {
                sequence: u64::try_from(events.len()).unwrap_or(u64::MAX),
                scenario: trace.scenario.clone(),
                kind: TuiEventKind::AuraBeachAssigned,
                fields: BTreeMap::from([
                    ("region".into(), "Aura Beach".into()),
                    ("duties".into(), transition.stewardship.join("; ")),
                ]),
            });
        }
        if !transition.guardianship.is_empty() {
            events.push(TuiEvent {
                sequence: u64::try_from(events.len()).unwrap_or(u64::MAX),
                scenario: trace.scenario.clone(),
                kind: TuiEventKind::GuardianshipGranted,
                fields: BTreeMap::from([("region".into(), "Aura Sea".into())]),
            });
            events.push(TuiEvent {
                sequence: u64::try_from(events.len()).unwrap_or(u64::MAX),
                scenario: trace.scenario.clone(),
                kind: TuiEventKind::AuraSeaGuardianshipAssigned,
                fields: BTreeMap::from([("duties".into(), transition.guardianship.join("; "))]),
            });
        }
    }
    events.push(TuiEvent {
        sequence: u64::try_from(events.len()).unwrap_or(u64::MAX),
        scenario: trace.scenario.clone(),
        kind: TuiEventKind::ScenarioCompleted,
        fields: BTreeMap::from([
            (
                "replay".into(),
                if trace.live_replay_equivalent {
                    "Equivalent"
                } else {
                    "Divergent"
                }
                .into(),
            ),
            (
                "persistence".into(),
                if trace.canonical_persistence {
                    "Canonical"
                } else {
                    "Unavailable"
                }
                .into(),
            ),
        ]),
    });
    events
}
