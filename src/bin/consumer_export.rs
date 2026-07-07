use hollow_grove::{KernelPass, Point, build_snapshot_output, run_kernel_cycle};

fn build_export_from_consumer(kernel_pass: &KernelPass) -> String {
    build_snapshot_output(kernel_pass)
}

fn main() {
    let kernel_pass = run_kernel_cycle(Point);
    println!("{}", build_export_from_consumer(&kernel_pass));
}

#[cfg(test)]
mod tests {
    use super::build_export_from_consumer;
    use hollow_grove::{Point, run_kernel_cycle};

    #[test]
    fn export_consumer_reads_the_completed_kernel_pass() {
        let kernel_pass = run_kernel_cycle(Point);

        assert_eq!(
            build_export_from_consumer(&kernel_pass),
            "{\n\
             \x20\x20\"start\": \"Point\",\n\
             \x20\x20\"triway\": {\n\
             \x20\x20\x20\x20\"ways\": [\"One\", \"Two\", \"Three\"]\n\
             \x20\x20},\n\
             \x20\x20\"hollow_grove\": {\n\
             \x20\x20\x20\x20\"bond\": \"One\",\n\
             \x20\x20\x20\x20\"atmosphere\": [\"Two\", \"Three\"]\n\
             \x20\x20},\n\
             \x20\x20\"current_seam\": \"CurrentSeam\",\n\
             \x20\x20\"aura_beam\": \"AuraBeam\",\n\
             \x20\x20\"landed\": \"Point\",\n\
             \x20\x20\"canonical_witness\": \"start Point\\n↓\\nTriway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam\\n↓\\nAuraBeam\\n↓\\nlanded Point\"\n\
             }"
        );
        assert_eq!(
            kernel_pass.to_string(),
            "start Point\n↓\nTriway\n↓\nHollowGrove\n↓\nCurrentSeam\n↓\nAuraBeam\n↓\nlanded Point"
        );
    }
}
