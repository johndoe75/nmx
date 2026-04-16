# Agent Guidance

## Build & Run

```bash
cargo build --release   # Produces ./target/release/nmx
cargo install --path .  # Install locally
```

## Testing

```bash
cargo test              # No tests currently defined
```

CI runs `cargo build --verbose && cargo test --verbose` on push/PR to main.

## Project Structure

- Single-file Rust CLI: `src/main.rs`
- Entry point: `main()` in `src/main.rs`
- Dependencies: clap (derive), anyhow, colored, regex

## Key Conventions

- Uses Rust edition 2024
- CLI uses clap derive macros
- Number formats: `0x` (hex), `0o` (octal), plain (decimal)