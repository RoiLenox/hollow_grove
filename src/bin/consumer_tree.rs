use hollow_grove::{KernelPass, Point, build_tree_output, run_kernel_cycle};

fn build_tree_from_consumer(kernel_pass: &KernelPass) -> String {
    build_tree_output(kernel_pass)
}

fn main() {
    let kernel_pass = run_kernel_cycle(Point);
    println!("{}", build_tree_from_consumer(&kernel_pass));
}

#[cfg(test)]
mod tests {
    use super::build_tree_from_consumer;
    use hollow_grove::{Point, run_kernel_cycle};

    #[test]
    fn tree_consumer_reads_the_completed_kernel_pass() {
        let kernel_pass = run_kernel_cycle(Point);

        assert_eq!(
            build_tree_from_consumer(&kernel_pass),
            "KernelPass\n\
             ├─ start: Point\n\
             ├─ triway\n\
             │  ├─ ways: [One, Two, Three]\n\
             ├─ hollow_grove\n\
             │  ├─ bond: One\n\
             │  └─ atmosphere: [Two, Three]\n\
             ├─ current_seam: CurrentSeam\n\
             ├─ aura_beam: AuraBeam\n\
             └─ landed: Point"
        );
        assert_eq!(
            kernel_pass.to_string(),
            "start Point\n↓\nTriway\n↓\nHollowGrove\n↓\nCurrentSeam\n↓\nAuraBeam\n↓\nlanded Point"
        );
    }
}
