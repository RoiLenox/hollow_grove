use hollow_grove::{KernelPass, Symptom, build_inverse_path_prompt, run_kernel_cycle};

fn build_prompt_from_consumer(kernel_pass: &KernelPass) -> String {
    build_prompt_from_witness(&kernel_pass.to_string())
}

fn build_prompt_from_witness(witness: &str) -> String {
    build_inverse_path_prompt(witness)
}

fn main() {
    let kernel_pass = run_kernel_cycle(Symptom::origin());
    println!("{}", build_prompt_from_consumer(&kernel_pass));
}

#[cfg(test)]
mod tests {
    use super::{build_prompt_from_consumer, build_prompt_from_witness};
    use hollow_grove::{CANONICAL_WITNESS, INVERSE_PATH_QUESTION, Symptom, run_kernel_cycle};

    #[test]
    fn consumer_prompt_preserves_the_canonical_witness() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());

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
