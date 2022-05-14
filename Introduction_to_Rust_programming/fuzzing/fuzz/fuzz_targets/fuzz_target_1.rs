#![no_main]
use fuzzing::sum;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    sum(data);
});
