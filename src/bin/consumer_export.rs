use hollow_grove::{KernelPass, Point, Way, run_kernel_cycle};

fn way_name(way: Way) -> &'static str {
    match way {
        Way::One => "One",
        Way::Two => "Two",
        Way::Three => "Three",
    }
}

fn build_export_from_consumer(kernel_pass: &KernelPass) -> String {
    let triway = kernel_pass.triway();
    let [way_one, way_two, way_three] = triway.ways();

    let hollow_grove = kernel_pass.hollow_grove();
    let [atmosphere_one, atmosphere_two] = hollow_grove.atmosphere();

    format!(
        "{{\n\
         \x20\x20\"start\": \"{:?}\",\n\
         \x20\x20\"triway\": {{\n\
         \x20\x20\x20\x20\"ways\": [\"{}\", \"{}\", \"{}\"]\n\
         \x20\x20}},\n\
         \x20\x20\"hollow_grove\": {{\n\
         \x20\x20\x20\x20\"bond\": \"{}\",\n\
         \x20\x20\x20\x20\"atmosphere\": [\"{}\", \"{}\"]\n\
         \x20\x20}},\n\
         \x20\x20\"current_seam\": \"CurrentSeam\",\n\
         \x20\x20\"aura_beam\": \"AuraBeam\",\n\
         \x20\x20\"landed\": \"{:?}\",\n\
         \x20\x20\"canonical_witness\": \"{}\"\n\
         }}",
        kernel_pass.start_point(),
        way_name(way_one),
        way_name(way_two),
        way_name(way_three),
        way_name(hollow_grove.link()),
        way_name(atmosphere_one),
        way_name(atmosphere_two),
        kernel_pass.landed_point(),
        kernel_pass.to_string().replace('\n', "\\n")
    )
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
