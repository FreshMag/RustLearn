# RustLearn

Quick guide for common Rust/Cargo tasks in this repository.

## Project structure

This repository contains multiple standalone Cargo projects (for example `00-Hello`, `01-hello_cargo`, `02-guessing_game`, etc.).

Most commands below can be run in either way:

- from inside a project folder (`cd 01-hello_cargo`)
- from the repository root with `--manifest-path`

## Compile a project

Debug build:

```bash
cargo build --manifest-path 00-hello_cargo/Cargo.toml
```

Release (optimized) build:

```bash
cargo build --release --manifest-path 00-hello_cargo/Cargo.toml
```

## Run a project

Run in debug mode:

```bash
cargo run --manifest-path 00-hello_cargo/Cargo.toml
```

Run with release optimizations:

```bash
cargo run --release --manifest-path 00-hello_cargo/Cargo.toml
```

## Sync toolchain and dependencies

Install/update Rust toolchain:

```bash
rustup update
rustup default stable
```

Download dependencies in advance (without building):

```bash
cargo fetch --manifest-path 00-hello_cargo/Cargo.toml
```

Update dependency versions allowed by `Cargo.toml` and refresh `Cargo.lock`:

```bash
cargo update --manifest-path 00-hello_cargo/Cargo.toml
```

## Format code

Install rustfmt (once per toolchain):

```bash
rustup component add rustfmt
```

Format source files:

```bash
cargo fmt --manifest-path 00-hello_cargo/Cargo.toml
```

Check formatting in CI/local checks (no file changes):

```bash
cargo fmt --check --manifest-path 00-hello_cargo/Cargo.toml
```

## Useful quick checks

```bash
cargo check --manifest-path 00-hello_cargo/Cargo.toml
cargo test --manifest-path 00-hello_cargo/Cargo.toml
```

Replace `01-hello_cargo` with any other project folder in this repo.

