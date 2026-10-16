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
- **Terminal-first input** — configurable keybindings with fallback operation on standard TTY input.

> tuicli is under active development. Features, configuration, and behavior may change.

## Configuration

The main configuration file is `~/.config/tuicli/config.toml`. The status line shows the average CPU frequency in GHz (for example, `3.20 GHz`) to the left of the system time. tuicli prefers hardware-reported data from Linux CPUFreq, then falls back to the current scaling frequency and `/proc/cpuinfo`; unsupported systems show `-- GHz`. Some systems expose only the frequency requested by the scaling driver, which may differ from the measured hardware frequency. Set `desktop.time_format` to a [`strftime`](https://man7.org/linux/man-pages/man3/strftime.3.html) format to choose how the time is displayed:

```toml
[desktop]
time_format = "%H:%M:%S"
```

The format uses the system's local timezone. If `time_format` is empty or omitted, tuicli displays the raw Unix timestamp in seconds. Existing configuration files without this setting continue to work and use the timestamp fallback.

Keyboard shortcuts are configured in the `[keybindings]` section of the same `~/.config/tuicli/config.toml` file. Defaults are built into tuicli; add only the shortcuts you want to change. For example, to bind quit to Escape while retaining all other defaults:

```toml
[keybindings]
quit = "escape"
toggle_mode = "f2" # default: F2
```

Defaults are applied automatically for any shortcut omitted from `[keybindings]`. Movement and resize shortcuts repeat while held; other shortcuts only run on the initial press.

tuicli requires an interactive terminal on both standard input and output. Its embedded shell runs in an xterm-compatible 256-color PTY profile; this profile is independent of the outer terminal used to display tuicli. Keyboard enhancement is optional, and keybindings continue to use Crossterm's regular key events when it is unavailable. Session state follows the XDG state directory (`~/.local/state/tuicli` by default). A `session.toml` in the config directory is ignored.

Running `tuicli` from one of its managed terminal windows is rejected to avoid nested desktop instances. Exit the current tuicli session first, then launch it again from the login shell.

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

### Other package managers

Package definitions for Arch Linux (AUR), Homebrew on Linux, and Nix are being prepared. See [additional package distribution](docs/package-distributions.md) for their current status.

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
