# Rust Solution

## Testing

```sh
cargo test
```

## Benchmarking

```sh
cargo bench
```

## Profiling

```sh
cargo install samply
samply record ./target/release/parse-kata -i ../../samples/10mb-input.txt -o ../../data/rust-10mb-ouput.txt -f
```

## Profile Guided Optimisation

[The rustc book - Profile-guided Optimization](https://doc.rust-lang.org/rustc/profile-guided-optimization.html)

1. **Install nightly Rust**: Use `rustup install nightly`.
2. **Set nightly as default**: `rustup default nightly` (Optional).
3. **Edit Cargo.toml**: Add profile settings for PGO.
   ```toml
   [profile.release]
   lto = true
   codegen-units = 1
   panic = 'abort'
   ```
4. **Initial Compile**: Compile with `-Cprofile-generate`.
   ```
   RUSTFLAGS="-Cprofile-generate" cargo build --release
   ```
5. **Run Executable**: Execute your binary to collect profile data.
6. **Merge Data**: Use `llvm-profdata merge` to merge `.profraw` files.
   ```
   xcrun llvm-profdata merge -output=default.profdata default_*.profraw
   ```
7. **Optimized Compile**: Compile with `-Cprofile-use`.
   ```
   RUSTFLAGS="-Cprofile-use=default.profdata" cargo build --release
   ```
   Tip: I had to use the full path for `default.profdata` file.
8. **Done**: Your binary is now optimized with PGO.

### Points to Note:

- Benefits from **real-world usage patterns** for the profiling run.
- Involves **two-step compilation**: one for generating profile and one for optimized build.
