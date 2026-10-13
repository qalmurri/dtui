# DTUI PTY Terminal integration

This source snapshot adds an interactive shell to every DTUI terminal window using `portable-pty` and `vt100`.

## Dependencies

`Cargo.toml` adds:

```toml
portable-pty = "0.9"
vt100 = "0.16"
```

## Keyboard usage

- DTUI starts in terminal-input mode. Type normally to interact with the focused shell.
- Press **F2** to switch to desktop shortcuts.
- In desktop mode, use the existing configured keybindings for window movement, resize, focus, workspace navigation, create/close, help, and quit.
- Press **F2** again to return to terminal input.
- Closing a terminal window terminates its shell process. Quitting DTUI stops managed shell processes and saves the existing window/workspace session.

## Session behavior

The session file continues to store window/workspace metadata only. Processes cannot be restored after DTUI exits; when a saved session is loaded, DTUI creates a new shell for each restored terminal window.

## Validate on Arch Linux

Run from the project root:

```bash
cargo fmt && cargo check && cargo test
cargo run
```

Then test `printf`, ANSI colors, arrow keys, Ctrl-C, a full-screen application such as `less` or `vim`, window resizing, F2 mode switching, closing a window, and restarting DTUI.
