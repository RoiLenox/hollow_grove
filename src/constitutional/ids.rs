use std::fmt;

fn is_stable_id(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-')
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstitutionalIdError {
    Invalid(String),
}

impl fmt::Display for ConstitutionalIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(value) => write!(formatter, "invalid constitutional identifier: {value}"),
        }
    }
}

impl std::error::Error for ConstitutionalIdError {}

macro_rules! stable_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ConstitutionalIdError> {
                let value = value.into();
                if is_stable_id(&value) {
                    Ok(Self(value))
                } else {
                    Err(ConstitutionalIdError::Invalid(value))
                }
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

stable_id!(WaveId);
stable_id!(BondId);
stable_id!(ConstitutionalEventId);
stable_id!(ParticipantId);
stable_id!(RoleId);
stable_id!(ObligationId);
stable_id!(PermissionId);
stable_id!(RuleSetId);
stable_id!(UnitId);
stable_id!(HouseDecisionId);
stable_id!(AuthorityActorId);
stable_id!(CurrentTransactionId);
stable_id!(AuraObservationId);
stable_id!(EvaluationId);
stable_id!(TombstoneId);
stable_id!(TokeId);
stable_id!(ResolutionId);
stable_id!(ArtifactId);
stable_id!(ChallengeId);
stable_id!(DefaultId);
stable_id!(RegionalBeingId);
stable_id!(RegionalEventId);
stable_id!(RegionalSynthesisId);
