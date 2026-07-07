use hollow_grove::{Point, run_kernel_cycle};

fn main() {
    let kernel_pass = run_kernel_cycle(Point);
    println!("{kernel_pass}");
}
