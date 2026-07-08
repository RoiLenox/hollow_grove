use hollow_grove::{KernelPass, Symptom, build_snapshot_output, run_kernel_cycle};

fn build_export_from_consumer(kernel_pass: &KernelPass) -> String {
    build_snapshot_output(kernel_pass)
}

fn main() {
    let kernel_pass = run_kernel_cycle(Symptom::origin());
    println!("{}", build_export_from_consumer(&kernel_pass));
}

#[cfg(test)]
mod tests {
    use super::build_export_from_consumer;
    use hollow_grove::{CANONICAL_WITNESS, Symptom, run_kernel_cycle};

    #[test]
    fn export_consumer_reads_the_completed_kernel_pass() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());

        assert_eq!(
            build_export_from_consumer(&kernel_pass),
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
        assert_eq!(kernel_pass.to_string(), CANONICAL_WITNESS);
    }
}
