//! Compatibility facade for former Runtime Federation fixture names.

pub use crate::runtime_rampart_fixture::*;

pub const FEDERATION_KERNEL_COMPONENT_ID: &str = RAMPART_KERNEL_COMPONENT_ID;
pub const FEDERATION_CONSTITUTIONAL_COMPONENT_ID: &str = RAMPART_CONSTITUTIONAL_COMPONENT_ID;
pub const FEDERATION_REGIONAL_COMPONENT_ID: &str = RAMPART_REGIONAL_COMPONENT_ID;
pub const FEDERATION_INSTITUTIONAL_COMPONENT_ID: &str = RAMPART_INSTITUTIONAL_COMPONENT_ID;
pub const FEDERATION_WORLD_POINT_COMPONENT_ID: &str = RAMPART_WORLD_POINT_COMPONENT_ID;
pub const FEDERATION_FUNCTION_JUNCTION_COMPONENT_ID: &str = RAMPART_FUNCTION_JUNCTION_COMPONENT_ID;
pub const FEDERATION_TOURNAMENT_COMPONENT_ID: &str = RAMPART_TOURNAMENT_COMPONENT_ID;
pub const FEDERATION_ROUTE_COMPONENT_ID: &str = RAMPART_ROUTE_COMPONENT_ID;
pub const FEDERATION_PERMANENCE_COMPONENT_ID: &str = RAMPART_PERMANENCE_COMPONENT_ID;
pub const FEDERATION_GAMEPLAY_COMPONENT_ID: &str = RAMPART_GAMEPLAY_COMPONENT_ID;

pub fn canonical_runtime_federation_payload() -> Result<
    crate::runtime_rampart::RuntimeRampartArchivePayload,
    crate::runtime_rampart::RuntimeRampartError,
> {
    crate::runtime_rampart_fixture::canonical_runtime_rampart_payload()
}
