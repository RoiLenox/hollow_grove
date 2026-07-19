#![no_main]

use hollow_grove::verification::fuzz_snapshot_boundary_bytes;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    fuzz_snapshot_boundary_bytes(data);
});
