# Fuzzing Demo

This code needs nightly Rust.

```
$ rustup default nightly
$ rustc --version
rustc 1.60.0-nightly (1409c015b 2022-01-11)
```

```
$ cargo fuzz run fuzz_target_1
```
* This will be panicked at 'attempt to add with overflow'

```
$ cargo fuzz run fuzz_target_2
```
* This will be OK.


After testing, set back to stable rustc.
```
$ rustup default stable
$ rustc --version
rustc 1.56.1 (59eed8a2a 2021-11-01)
```