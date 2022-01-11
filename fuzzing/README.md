# Fuzzing Demo

This code needs nightly Rust.

If not it fails with this error.

```
$fuzz % cargo fuzz run fuzz_target_1
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `rustc - --crate-name ___ --print=file-names -Cpasses=sancov-module -Cllvm-args=-sanitizer-coverage-level=4 -Cllvm-args=-sanitizer-coverage-trace-compares -Cllvm-args=-sanitizer-coverage-inline-8bit-counters -Cllvm-args=-sanitizer-coverage-pc-table --cfg fuzzing -Clink-dead-code -Zsanitizer=address -Cdebug-assertions -C codegen-units=1 --target x86_64-apple-darwin --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit status: 1)
  --- stderr
  error: the option `Z` is only accepted on the nightly compiler

Error: failed to build fuzz script: "cargo" "build" "--manifest-path" "/Volumes/Thunderbolt3_SSD/workspace/Rust/Rust_practice/fuzzing/fuzz/Cargo.toml" "--target" "x86_64-apple-darwin" "--release" "--bin" "fuzz_target_1"
$fuzz % 
```