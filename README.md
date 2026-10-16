# tuicli

[![Rust](https://github.com/qalmurri/tuicli/actions/workflows/rust.yml/badge.svg)](https://github.com/qalmurri/tuicli/actions/workflows/rust.yml)
[![Documentation](https://github.com/qalmurri/tuicli/actions/workflows/docs.yml/badge.svg)](https://github.com/qalmurri/tuicli/actions/workflows/docs.yml)
[![Release](https://github.com/qalmurri/tuicli/actions/workflows/release.yml/badge.svg)](https://github.com/qalmurri/tuicli/actions/workflows/release.yml)
[![Security Audit](https://github.com/qalmurri/tuicli/actions/workflows/security.yml/badge.svg)](https://github.com/qalmurri/tuicli/actions/workflows/security.yml)

**tuicli** is a lightweight, terminal-based desktop environment for Linux, designed to provide a structured CLI workspace without a graphical desktop environment.

Built with Rust, tuicli focuses on stability, efficiency, and simplicity.

## Features

- **Window management** — manage multiple terminal windows.
- **Workspaces** — organize windows across separate workspaces.
- **PTY integration** — run interactive shell sessions inside windows.
- **TOML configuration** — customize desktop settings.
- **Linux TTY oriented** — designed for minimal Linux environments without Xorg or Wayland.

> tuicli is under active development. Features, configuration, and behavior may change.

## Configuration

The main configuration file is `~/.config/tuicli/config.toml`. The status line shows the system time on its right side. Set `desktop.time_format` to a [`strftime`](https://man7.org/linux/man-pages/man3/strftime.3.html) format to choose how it is displayed:

```toml
[desktop]
title = "TUICLI"
time_format = "%H:%M:%S"
```

The format uses the system's local timezone. If `time_format` is empty or omitted, tuicli displays the raw Unix timestamp in seconds. Existing configuration files without this setting continue to work and use the timestamp fallback.

## Installation

### Debian / Ubuntu (APT)

The APT repository provides 64-bit x86 (`amd64`) packages for Debian 12+ and Ubuntu 22.04+. Once the repository is published, add its signing key and package source, then install tuicli:

```bash
sudo install -d -m 0755 /etc/apt/keyrings
curl -fsSL https://qalmurri.github.io/tuicli/tuicli-archive-keyring.gpg \
  | sudo tee /etc/apt/keyrings/tuicli.gpg >/dev/null
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/tuicli.gpg] https://qalmurri.github.io/tuicli stable main" \
  | sudo tee /etc/apt/sources.list.d/tuicli.list
sudo apt update
sudo apt install tuicli
```

Repository publishing setup for maintainers is documented in [docs/apt-repository.md](docs/apt-repository.md).

### Cargo

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
