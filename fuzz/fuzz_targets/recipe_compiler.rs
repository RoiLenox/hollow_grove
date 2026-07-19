#![no_main]

use hollow_grove::verification::fuzz_recipe_compiler_bytes;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    fuzz_recipe_compiler_bytes(data);
});
