use hollow_grove::{KernelPass, Point, run_kernel_cycle};

const INVERSE_PATH_QUESTION: &str =
    "What does this completed pass reveal about the inverse path of the end use?";

fn build_prompt_from_consumer(kernel_pass: &KernelPass) -> String {
    build_prompt_from_witness(&kernel_pass.to_string())
}

fn build_prompt_from_witness(witness: &str) -> String {
    format!("{witness}\n\n{INVERSE_PATH_QUESTION}")
}

fn main() {
    let kernel_pass = run_kernel_cycle(Point);
    println!("{}", build_prompt_from_consumer(&kernel_pass));
}

#[cfg(test)]
mod tests {
    use super::{INVERSE_PATH_QUESTION, build_prompt_from_consumer, build_prompt_from_witness};
    use hollow_grove::{Point, run_kernel_cycle};

    const CANONICAL_WITNESS: &str =
        "start Point\n↓\nTriway\n↓\nHollowGrove\n↓\nCurrentSeam\n↓\nAuraBeam\n↓\nlanded Point";

    #[test]
    fn consumer_prompt_preserves_the_canonical_witness() {
        let kernel_pass = run_kernel_cycle(Point);

        assert_eq!(
            build_prompt_from_consumer(&kernel_pass),
            format!("{CANONICAL_WITNESS}\n\n{INVERSE_PATH_QUESTION}")
        );
        assert_eq!(kernel_pass.to_string(), CANONICAL_WITNESS);
    }

    #[test]
    fn prompt_from_witness_preserves_the_given_witness_exactly() {
        assert_eq!(
            build_prompt_from_witness(CANONICAL_WITNESS),
            format!("{CANONICAL_WITNESS}\n\n{INVERSE_PATH_QUESTION}")
        );
    }
}
