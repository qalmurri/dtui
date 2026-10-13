# DTUI session persistence update

Replace these files in the existing `dtui` project:

- `src/session.rs`
- `src/app.rs`
- `src/desktop/desktop.rs`

No Cargo dependency changes are required. The existing `serde`, `toml`, and `directories` dependencies are used.

Then run from the project root:

```bash
cargo fmt && cargo check && cargo test
```

Session state is saved on normal application exit to the DTUI configuration directory (`session.toml`). A first launch creates one `Terminal` window on workspace `MAIN`; later launches restore the saved windows instead of creating duplicates.

The session tests use unique files under the system temporary directory and do not read or overwrite the user's real session file.
