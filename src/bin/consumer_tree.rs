use hollow_grove::{KernelPass, Point, Way, run_kernel_cycle};

fn way_name(way: Way) -> &'static str {
    match way {
        Way::One => "One",
        Way::Two => "Two",
        Way::Three => "Three",
    }
}

fn build_tree_from_consumer(kernel_pass: &KernelPass) -> String {
    let triway = kernel_pass.triway();
    let [way_one, way_two, way_three] = triway.ways();

    let hollow_grove = kernel_pass.hollow_grove();
    let [atmosphere_one, atmosphere_two] = hollow_grove.atmosphere();

    format!(
        "KernelPass\n\
         ├─ start: {:?}\n\
         ├─ triway\n\
         │  ├─ ways: [{}, {}, {}]\n\
         ├─ hollow_grove\n\
         │  ├─ bond: {}\n\
         │  └─ atmosphere: [{}, {}]\n\
         ├─ current_seam: CurrentSeam\n\
         ├─ aura_beam: AuraBeam\n\
         └─ landed: {:?}",
        kernel_pass.start_point(),
        way_name(way_one),
        way_name(way_two),
        way_name(way_three),
        way_name(hollow_grove.link()),
        way_name(atmosphere_one),
        way_name(atmosphere_two),
        kernel_pass.landed_point()
    )
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
