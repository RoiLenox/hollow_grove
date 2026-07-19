#![no_main]

use hollow_grove::verification::fuzz_decision_input_bytes;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    fuzz_decision_input_bytes(data);
});
