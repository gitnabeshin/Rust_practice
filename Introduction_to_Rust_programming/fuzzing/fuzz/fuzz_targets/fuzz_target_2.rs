#![no_main]
use fuzzing::sum_wrap;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    sum_wrap(data);
});
