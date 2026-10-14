# tuicli

[![Rust](https://github.com/qalmurri/tuicli/actions/workflows/rust.yml/badge.svg)](https://github.com/qalmurri/tuicli/actions/workflows/rust.yml)

**tuicli** is a lightweight, terminal-based desktop environment for Linux, designed to provide a structured CLI workspace without a graphical desktop environment.

Built with Rust, tuicli focuses on stability, efficiency, and simplicity.

## Features

- **Window management** — manage multiple terminal windows.
- **Workspaces** — organize windows across separate workspaces.
- **PTY integration** — run interactive shell sessions inside windows.
- **TOML configuration** — customize desktop settings.
- **Linux TTY oriented** — designed for minimal Linux environments without Xorg or Wayland.

> tuicli is under active development. Features, configuration, and behavior may change.

## Installation

Install from [crates.io](https://crates.io/crates/tuicli):

```bash
cargo install tuicli
```

Ensure `~/.cargo/bin` is in your `PATH`, then run:

```bash
tuicli
```

## Development

Requirements: Rust and Cargo.

```bash
git clone https://github.com/qalmurri/tuicli.git
cd tuicli
cargo run
```

Run checks and tests:

```bash
cargo fmt --check
cargo check
cargo test
```

## Technology

- [Rust](https://www.rust-lang.org/)
- [Ratatui](https://ratatui.rs/)
- [Crossterm](https://github.com/crossterm-rs/crossterm)
- [portable-pty](https://github.com/wezterm/wezterm)
- TOML configuration

## License

Licensed under the [MIT License](LICENSE).
