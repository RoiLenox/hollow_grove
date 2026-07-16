use std::io;
use std::path::Path;

use crate::read_text_artifact;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotBoundary {
    grove_seam_route: String,
    hollow_beam_route: String,
    landing_route: String,
    landed_point: String,
    canonical_witness: String,
}

impl SnapshotBoundary {
    pub fn parse(snapshot: &str) -> io::Result<Self> {
        validate_schema_version(snapshot)?;

        let boundary = Self {
            grove_seam_route: extract_json_string(snapshot, "grove_seam_route")?,
            hollow_beam_route: extract_json_string(snapshot, "hollow_beam_route")?,
            landing_route: extract_json_string(snapshot, "landing_route")?,
            landed_point: extract_json_string(snapshot, "landed_point")?,
            canonical_witness: unescape_newlines(&extract_json_string(
                snapshot,
                "canonical_witness",
            )?),
        };

        boundary.validate()?;

        Ok(boundary)
    }

    pub fn read_from_path(path: &Path) -> io::Result<Self> {
        let snapshot = read_text_artifact(path)?;
        Self::parse(&snapshot)
    }

    pub fn grove_seam_route(&self) -> &str {
        &self.grove_seam_route
    }

    pub fn hollow_beam_route(&self) -> &str {
        &self.hollow_beam_route
    }

    pub fn landing_route(&self) -> &str {
        &self.landing_route
    }

    pub fn landed_point(&self) -> &str {
        &self.landed_point
    }

    pub fn canonical_witness(&self) -> &str {
        &self.canonical_witness
    }

    fn validate(&self) -> io::Result<()> {
        match (
            self.grove_seam_route.as_str(),
            self.hollow_beam_route.as_str(),
            self.landing_route.as_str(),
        ) {
            ("PlebExterior", "BlepReturn", "BlepArrival")
            | ("MetaExterior", "AtemReturn", "AtemArrival") => {}
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "snapshot boundary route chain is contradictory: {} -> {} -> {}",
                        self.grove_seam_route, self.hollow_beam_route, self.landing_route
                    ),
                ));
            }
        }

        if self.landed_point != "Point²" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "snapshot boundary landed_point must remain the universal `Point²`, got `{}`",
                    self.landed_point
                ),
            ));
        }

        let expected_witness = format!(
            "Point\n\
             ↓\n\
             Triway\n\
             ↓\n\
             Fourway\n\
             ↓\n\
             HollowGrove\n\
             ↓\n\
             CurrentSeam [{}]\n\
             ↓\n\
             AuraBeam [{}]\n\
             ↓\n\
             Point² (Landed Point) [{}]",
            self.grove_seam_route, self.hollow_beam_route, self.landing_route
        );
        if self.canonical_witness != expected_witness {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "snapshot canonical witness does not match the frozen route witness",
            ));
        }

        Ok(())
    }
}

fn extract_json_string(snapshot: &str, key: &str) -> io::Result<String> {
    let needle = format!("\"{key}\": \"");
    let start = unique_field_start(snapshot, key, &needle)?.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("snapshot missing string field `{key}`"),
        )
    })?;

    let rest = &snapshot[start + needle.len()..];
    let end = rest.find('"').ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("snapshot field `{key}` is not terminated"),
        )
    })?;

    Ok(rest[..end].to_string())
}

fn extract_optional_json_string(snapshot: &str, key: &str) -> io::Result<Option<String>> {
    let needle = format!("\"{key}\": \"");
    let Some(start) = unique_field_start(snapshot, key, &needle)? else {
        return Ok(None);
    };

    let rest = &snapshot[start + needle.len()..];
    let end = rest.find('"').ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("snapshot field `{key}` is not terminated"),
        )
    })?;

    Ok(Some(rest[..end].to_string()))
}

fn unique_field_start(snapshot: &str, key: &str, needle: &str) -> io::Result<Option<usize>> {
    let mut matches = snapshot.match_indices(needle).map(|(index, _)| index);
    let Some(first) = matches.next() else {
        return Ok(None);
    };

    if matches.next().is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("snapshot contains duplicate string field `{key}`"),
        ));
    }

    Ok(Some(first))
}

fn validate_schema_version(snapshot: &str) -> io::Result<()> {
    let Some(schema_version) = extract_optional_json_string(snapshot, "schema_version")? else {
        return Ok(());
    };

    if schema_version == "1" {
        return Ok(());
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("snapshot schema version `{schema_version}` is unsupported"),
    ))
}

fn unescape_newlines(value: &str) -> String {
    value.replace("\\n", "\n")
}

pub fn build_snapshot_boundary_output(boundary: &SnapshotBoundary) -> String {
    format!(
        "SnapshotBoundary\n\
         grove_seam_route: {}\n\
         hollow_beam_route: {}\n\
         landing_route: {}\n\
         landed_point: {}\n\
         canonical_witness:\n\
         {}",
        boundary.grove_seam_route(),
        boundary.hollow_beam_route(),
        boundary.landing_route(),
        boundary.landed_point(),
        boundary.canonical_witness()
    )
}

#[cfg(test)]
mod tests {
    use crate::{
        ExteriorShape, KernelInput, Mode, PlebMetaInput, Symptom, build_snapshot_output,
        run_kernel_cycle, run_kernel_cycle_with_input,
    };

    use super::{SnapshotBoundary, build_snapshot_boundary_output};

    #[test]
    fn snapshot_boundary_reads_the_frozen_straight_route_chain() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        let boundary = SnapshotBoundary::parse(&build_snapshot_output(&kernel_pass))
            .expect("snapshot boundary should parse");

        assert_eq!(boundary.grove_seam_route(), "PlebExterior");
        assert_eq!(boundary.hollow_beam_route(), "BlepReturn");
        assert_eq!(boundary.landing_route(), "BlepArrival");
        assert_eq!(boundary.landed_point(), "Point²");
        assert!(boundary.canonical_witness().contains("Fourway"));
        assert!(
            boundary
                .canonical_witness()
                .contains("CurrentSeam [PlebExterior]")
        );
        assert!(
            boundary
                .canonical_witness()
                .contains("AuraBeam [BlepReturn]")
        );
        assert!(
            boundary
                .canonical_witness()
                .contains("Point² (Landed Point) [BlepArrival]")
        );
    }

    #[test]
    fn snapshot_boundary_reads_the_frozen_curved_route_chain() {
        let kernel_pass = run_kernel_cycle_with_input(
            Symptom::origin(),
            KernelInput {
                routing: PlebMetaInput {
                    exterior_shape: ExteriorShape::Curved,
                    pleb_mode: Mode::Pathos,
                    meta_mode: Mode::Logos,
                },
            },
        );
        let boundary = SnapshotBoundary::parse(&build_snapshot_output(&kernel_pass))
            .expect("snapshot boundary should parse");

        assert_eq!(boundary.grove_seam_route(), "MetaExterior");
        assert_eq!(boundary.hollow_beam_route(), "AtemReturn");
        assert_eq!(boundary.landing_route(), "AtemArrival");
        assert_eq!(boundary.landed_point(), "Point²");
        assert!(boundary.canonical_witness().contains("Fourway"));
        assert!(
            boundary
                .canonical_witness()
                .contains("CurrentSeam [MetaExterior]")
        );
        assert!(
            boundary
                .canonical_witness()
                .contains("AuraBeam [AtemReturn]")
        );
        assert!(
            boundary
                .canonical_witness()
                .contains("Point² (Landed Point) [AtemArrival]")
        );
    }

    #[test]
    fn snapshot_boundary_output_is_deterministic() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        let boundary = SnapshotBoundary::parse(&build_snapshot_output(&kernel_pass))
            .expect("snapshot boundary should parse");

        assert_eq!(
            build_snapshot_boundary_output(&boundary),
            "SnapshotBoundary\n\
             grove_seam_route: PlebExterior\n\
             hollow_beam_route: BlepReturn\n\
             landing_route: BlepArrival\n\
             landed_point: Point²\n\
             canonical_witness:\n\
             Point\n\
             ↓\n\
             Triway\n\
             ↓\n\
             Fourway\n\
             ↓\n\
             HollowGrove\n\
             ↓\n\
             CurrentSeam [PlebExterior]\n\
             ↓\n\
             AuraBeam [BlepReturn]\n\
             ↓\n\
             Point² (Landed Point) [BlepArrival]"
        );
    }

    #[test]
    fn snapshot_boundary_rejects_missing_route_fields() {
        let error = SnapshotBoundary::parse("{\"canonical_witness\": \"x\"}")
            .expect_err("snapshot without routes should fail");

        assert_eq!(
            error.to_string(),
            "snapshot missing string field `grove_seam_route`"
        );
    }

    #[test]
    fn snapshot_boundary_rejects_unsupported_schema_versions() {
        let error = SnapshotBoundary::parse(
            "{\n\
             \x20\x20\"schema_version\": \"2\",\n\
             \x20\x20\"grove_seam_route\": \"PlebExterior\",\n\
             \x20\x20\"hollow_beam_route\": \"BlepReturn\",\n\
             \x20\x20\"landing_route\": \"BlepArrival\",\n\
             \x20\x20\"landed_point\": \"Point²\",\n\
             \x20\x20\"canonical_witness\": \"Point\\n↓\\nTriway\\n↓\\nFourway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam [PlebExterior]\\n↓\\nAuraBeam [BlepReturn]\\n↓\\nPoint² (Landed Point) [BlepArrival]\"\n\
             }",
        )
        .expect_err("unsupported snapshot schema should fail");

        assert_eq!(
            error.to_string(),
            "snapshot schema version `2` is unsupported"
        );
    }

    #[test]
    fn snapshot_boundary_rejects_duplicate_authoritative_fields() {
        let error = SnapshotBoundary::parse(
            "{\n\
             \x20\x20\"grove_seam_route\": \"PlebExterior\",\n\
             \x20\x20\"hollow_beam_route\": \"BlepReturn\",\n\
             \x20\x20\"landing_route\": \"BlepArrival\",\n\
             \x20\x20\"landed_point\": \"Point²\",\n\
             \x20\x20\"canonical_witness\": \"Point\\n↓\\nTriway\\n↓\\nFourway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam [PlebExterior]\\n↓\\nAuraBeam [BlepReturn]\\n↓\\nPoint² (Landed Point) [BlepArrival]\",\n\
             \x20\x20\"grove_seam_route\": \"MetaExterior\"\n\
             }",
        )
        .expect_err("duplicate fields should fail");

        assert_eq!(
            error.to_string(),
            "snapshot contains duplicate string field `grove_seam_route`"
        );
    }

    #[test]
    fn snapshot_boundary_rejects_contradictory_route_chains() {
        let error = SnapshotBoundary::parse(
            "{\n\
             \x20\x20\"grove_seam_route\": \"PlebExterior\",\n\
             \x20\x20\"hollow_beam_route\": \"AtemReturn\",\n\
             \x20\x20\"landing_route\": \"AtemArrival\",\n\
             \x20\x20\"landed_point\": \"Point²\",\n\
             \x20\x20\"canonical_witness\": \"Point\\n↓\\nTriway\\n↓\\nFourway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam [PlebExterior]\\n↓\\nAuraBeam [AtemReturn]\\n↓\\nPoint² (Landed Point) [AtemArrival]\"\n\
             }",
        )
        .expect_err("contradictory route chain should fail");

        assert_eq!(
            error.to_string(),
            "snapshot boundary route chain is contradictory: PlebExterior -> AtemReturn -> AtemArrival"
        );
    }

    #[test]
    fn snapshot_boundary_rejects_non_universal_landed_points() {
        let error = SnapshotBoundary::parse(
            "{\n\
             \x20\x20\"grove_seam_route\": \"PlebExterior\",\n\
             \x20\x20\"hollow_beam_route\": \"BlepReturn\",\n\
             \x20\x20\"landing_route\": \"BlepArrival\",\n\
             \x20\x20\"landed_point\": \"Symptom\",\n\
             \x20\x20\"canonical_witness\": \"Point\\n↓\\nTriway\\n↓\\nFourway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam [PlebExterior]\\n↓\\nAuraBeam [BlepReturn]\\n↓\\nPoint² (Landed Point) [BlepArrival]\"\n\
             }",
        )
        .expect_err("non-universal landed point should fail");

        assert_eq!(
            error.to_string(),
            "snapshot boundary landed_point must remain the universal `Point²`, got `Symptom`"
        );
    }

    #[test]
    fn snapshot_boundary_rejects_witness_prefix_garbage() {
        let error = SnapshotBoundary::parse(
            "{\n\
             \x20\x20\"grove_seam_route\": \"PlebExterior\",\n\
             \x20\x20\"hollow_beam_route\": \"BlepReturn\",\n\
             \x20\x20\"landing_route\": \"BlepArrival\",\n\
             \x20\x20\"landed_point\": \"Point²\",\n\
             \x20\x20\"canonical_witness\": \"garbagePoint\\n↓\\nTriway\\n↓\\nFourway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam [PlebExterior]\\n↓\\nAuraBeam [BlepReturn]\\n↓\\nPoint² (Landed Point) [BlepArrival]\"\n\
             }",
        )
        .expect_err("witness prefix garbage should fail");

        assert_eq!(
            error.to_string(),
            "snapshot canonical witness does not match the frozen route witness"
        );
    }
}
