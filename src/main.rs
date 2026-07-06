use hollow_grove::kernel_proof;

fn main() {
    for line in kernel_proof() {
        println!("{line}");
    }
}
