# DTUI app.rs refactor

This patch splits the current `src/app.rs` into a directory module while preserving the existing `app::run(...)` call in `main.rs`.

## Files

- `src/main.rs` — updated module entry, functionally same startup path.
- `src/app/mod.rs` — shared `AppState` and input types.
- `src/app/runtime.rs` — event loop.
- `src/app/actions.rs` — keyboard action dispatch and window operations.
- `src/app/workspace.rs` — workspace selection, CRUD, and name input.
- `src/app/session.rs` — restore/save orchestration using the existing `crate::session::SessionState`.

## Apply

From the project root, extract this archive. Then remove the old `src/app.rs` because Rust cannot have both `src/app.rs` and `src/app/mod.rs` for the same module:

```bash
rm src/app.rs
cargo fmt && cargo check && cargo test
```

The archive intentionally does not replace `src/session.rs`, `src/desktop/desktop.rs`, or `src/window_manager/manager.rs`.
