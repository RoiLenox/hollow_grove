use crate::{KernelPass, LANDED_WITNESS_LABEL, START_WITNESS_LABEL, Way};

pub const SNAPSHOT_ARTIFACT_PATH: &str = "artifacts/kernel_pass_snapshot.json";
pub const PROMPT_ARTIFACT_PATH: &str = "artifacts/consumer_prompt.md";
pub const DESKTOP_STATUS_ARTIFACT_PATH: &str = "artifacts/desktop_status.txt";
pub const INVERSE_PATH_QUESTION: &str =
    "What does this completed pass reveal about the inverse path of the end use?";
pub const BOUNDARY_REMINDER: &str = "Do not mutate the kernel. Interpret only.";

fn way_name(way: Way) -> &'static str {
    match way {
        Way::One => "One",
        Way::Two => "Two",
        Way::Three => "Three",
    }
}

fn escaped_canonical_witness(kernel_pass: &KernelPass) -> String {
    kernel_pass.to_string().replace('\n', "\\n")
}

pub fn build_snapshot_output(kernel_pass: &KernelPass) -> String {
    let triway = kernel_pass.triway();
    let [way_one, way_two, way_three] = triway.ways();

    let hollow_grove = kernel_pass.hollow_grove();
    let [atmosphere_one, atmosphere_two] = hollow_grove.atmosphere();

    format!(
        "{{\n\
         \x20\x20\"start\": \"{}\",\n\
         \x20\x20\"triway\": {{\n\
         \x20\x20\x20\x20\"ways\": [\"{}\", \"{}\", \"{}\"]\n\
         \x20\x20}},\n\
         \x20\x20\"hollow_grove\": {{\n\
         \x20\x20\x20\x20\"bond\": \"{}\",\n\
         \x20\x20\x20\x20\"atmosphere\": [\"{}\", \"{}\"]\n\
         \x20\x20}},\n\
         \x20\x20\"grove_seam\": \"GroveSeam\",\n\
         \x20\x20\"hollow_beam\": \"HollowBeam\",\n\
         \x20\x20\"landed\": \"{}\",\n\
         \x20\x20\"canonical_witness\": \"{}\"\n\
         }}",
        START_WITNESS_LABEL,
        way_name(way_one),
        way_name(way_two),
        way_name(way_three),
        way_name(hollow_grove.link()),
        way_name(atmosphere_one),
        way_name(atmosphere_two),
        LANDED_WITNESS_LABEL,
        escaped_canonical_witness(kernel_pass)
    )
}

pub fn build_prompt_artifact_output(kernel_pass: &KernelPass) -> String {
    format!(
        "# Consumer Prompt\n\n\
         ## Canonical Witness\n\n\
         ```text\n\
         {}\n\
         ```\n\n\
         ## Structured Snapshot Reference\n\n\
         `{SNAPSHOT_ARTIFACT_PATH}`\n\n\
         ## Inverse-Path Question\n\n\
         {INVERSE_PATH_QUESTION}\n\n\
         ## Boundary Reminder\n\n\
         {BOUNDARY_REMINDER}\n",
        kernel_pass
    )
}

pub fn build_desktop_status_output(kernel_pass: &KernelPass) -> String {
    format!(
        "Hollow Grove status: one completed witnessed recursion\n\n\
         Canonical witness:\n\
         {}\n\n\
         Note: read-only desktop artifact\n\
         Note: niri/river configs untouched\n",
        kernel_pass
    )
}

pub fn build_tree_output(kernel_pass: &KernelPass) -> String {
    let triway = kernel_pass.triway();
    let [way_one, way_two, way_three] = triway.ways();

    let hollow_grove = kernel_pass.hollow_grove();
    let [atmosphere_one, atmosphere_two] = hollow_grove.atmosphere();

    format!(
        "KernelPass\n\
         ├─ start: {}\n\
         ├─ triway\n\
         │  ├─ ways: [{}, {}, {}]\n\
         ├─ hollow_grove\n\
         │  ├─ bond: {}\n\
         │  └─ atmosphere: [{}, {}]\n\
         ├─ grove_seam: GroveSeam\n\
         ├─ hollow_beam: HollowBeam\n\
         └─ landed: {}",
        START_WITNESS_LABEL,
        way_name(way_one),
        way_name(way_two),
        way_name(way_three),
        way_name(hollow_grove.link()),
        way_name(atmosphere_one),
        way_name(atmosphere_two),
        LANDED_WITNESS_LABEL
    )
}

pub fn build_inverse_path_prompt(witness: &str) -> String {
    format!("{witness}\n\n{INVERSE_PATH_QUESTION}")
}

#[cfg(test)]
mod tests {
    use crate::{CANONICAL_WITNESS, Symptom, run_kernel_cycle};

    use super::{
        build_desktop_status_output, build_inverse_path_prompt, build_prompt_artifact_output,
        build_snapshot_output, build_tree_output,
    };

    #[test]
    fn snapshot_output_remains_canonical() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());

        assert_eq!(
            build_snapshot_output(&kernel_pass),
            "{\n\
             \x20\x20\"start\": \"Symptom 1\",\n\
             \x20\x20\"triway\": {\n\
             \x20\x20\x20\x20\"ways\": [\"One\", \"Two\", \"Three\"]\n\
             \x20\x20},\n\
             \x20\x20\"hollow_grove\": {\n\
             \x20\x20\x20\x20\"bond\": \"One\",\n\
             \x20\x20\x20\x20\"atmosphere\": [\"Two\", \"Three\"]\n\
             \x20\x20},\n\
             \x20\x20\"grove_seam\": \"GroveSeam\",\n\
             \x20\x20\"hollow_beam\": \"HollowBeam\",\n\
             \x20\x20\"landed\": \"Symptom 2\",\n\
             \x20\x20\"canonical_witness\": \"start Symptom 1\\n↓\\nTriway\\n↓\\nHollowGrove\\n↓\\nGroveSeam\\n↓\\nHollowBeam\\n↓\\nlanded Symptom 2\"\n\
             }"
        );
    }

    #[test]
    fn prompt_artifact_output_remains_canonical() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());

        assert_eq!(
            build_prompt_artifact_output(&kernel_pass),
            "# Consumer Prompt\n\n\
             ## Canonical Witness\n\n\
             ```text\n\
             start Symptom 1\n\
             ↓\n\
             Triway\n\
             ↓\n\
             HollowGrove\n\
             ↓\n\
             GroveSeam\n\
             ↓\n\
             HollowBeam\n\
             ↓\n\
             landed Symptom 2\n\
             ```\n\n\
             ## Structured Snapshot Reference\n\n\
             `artifacts/kernel_pass_snapshot.json`\n\n\
             ## Inverse-Path Question\n\n\
             What does this completed pass reveal about the inverse path of the end use?\n\n\
             ## Boundary Reminder\n\n\
             Do not mutate the kernel. Interpret only.\n"
        );
    }

    #[test]
    fn desktop_status_output_remains_canonical() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());

        assert_eq!(
            build_desktop_status_output(&kernel_pass),
            "Hollow Grove status: one completed witnessed recursion\n\n\
             Canonical witness:\n\
             start Symptom 1\n\
             ↓\n\
             Triway\n\
             ↓\n\
             HollowGrove\n\
             ↓\n\
             GroveSeam\n\
             ↓\n\
             HollowBeam\n\
             ↓\n\
             landed Symptom 2\n\n\
             Note: read-only desktop artifact\n\
             Note: niri/river configs untouched\n"
        );
    }

    #[test]
    fn tree_output_remains_canonical() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());

        assert_eq!(
            build_tree_output(&kernel_pass),
            "KernelPass\n\
             ├─ start: Symptom 1\n\
             ├─ triway\n\
             │  ├─ ways: [One, Two, Three]\n\
             ├─ hollow_grove\n\
             │  ├─ bond: One\n\
             │  └─ atmosphere: [Two, Three]\n\
             ├─ grove_seam: GroveSeam\n\
             ├─ hollow_beam: HollowBeam\n\
             └─ landed: Symptom 2"
        );
    }

    #[test]
    fn inverse_path_prompt_preserves_the_given_witness_exactly() {
        assert_eq!(
            build_inverse_path_prompt(CANONICAL_WITNESS),
            format!(
                "{CANONICAL_WITNESS}\n\nWhat does this completed pass reveal about the inverse path of the end use?"
            )
        );
    }
}
