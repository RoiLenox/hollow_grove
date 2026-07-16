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
        let output = build_export_from_consumer(&kernel_pass);

        assert!(output.contains("\"start\": \"Point\""));
        assert!(output.contains("\"fourway\": \"Fourway\""));
        assert!(output.contains("\"landed_point\": \"Point²\""));
        assert!(output.contains("Point² (Landed Point) [BlepArrival]"));
        assert_eq!(kernel_pass.to_string(), CANONICAL_WITNESS);
    }
}
