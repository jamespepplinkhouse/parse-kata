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
