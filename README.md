# Accounting Ledger CLI Tool

Inspired by [ebcrowder/rust_ledger](https://github.com/ebcrowder/rust_ledger)


## Build
- `cargo build` for a faster build time
- `cargo build --release` to compile to an optimized production build

## Test
All tests are located in the corresponding module.

Run `cargo test`

## Running test coverage (instructions from [mozilla grcov](https://github.com/mozilla/grcov))
1. Install grcov `cargo install grcov`
2. Add llvm-tools `rustup component add llvm-tools-preview`
3. Make rust collect coverage `export RUSTFLAGS="-Zinstrument-coverage"`
4. Build with `cargo build`
5. make profiles `LLVM_PROFILE_FILE="your_name-%p-%m.profraw"`
6. Run tests `cargo test`
7. Collect Coverage `grcov . --binary-path target/debug -s . -t html --branch --ignore-not-existing -o ./coverage/`
