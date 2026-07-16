use hollow_grove::{KernelPass, Symptom, build_tree_output, run_kernel_cycle};

fn build_tree_from_consumer(kernel_pass: &KernelPass) -> String {
    build_tree_output(kernel_pass)
}

fn main() {
    let kernel_pass = run_kernel_cycle(Symptom::origin());
    println!("{}", build_tree_from_consumer(&kernel_pass));
}

#[cfg(test)]
mod tests {
    use super::build_tree_from_consumer;
    use hollow_grove::{CANONICAL_WITNESS, Symptom, run_kernel_cycle};

    #[test]
    fn tree_consumer_reads_the_completed_kernel_pass() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        let output = build_tree_from_consumer(&kernel_pass);

        assert!(output.contains("├─ start: Point"));
        assert!(output.contains("├─ fourway: Fourway"));
        assert!(output.contains("├─ current_seam: CurrentSeam [PlebExterior]"));
        assert!(output.contains("├─ aura_beam: AuraBeam [BlepReturn]"));
        assert!(output.contains("└─ point_squared: Point² (Landed Point) [BlepArrival]"));
        assert_eq!(kernel_pass.to_string(), CANONICAL_WITNESS);
    }
}
