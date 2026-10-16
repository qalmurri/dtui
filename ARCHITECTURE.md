# Architecture

Tuicli uses four code layers plus a composition root.

```text
main.rs (composition root)
   ├── wires infrastructure implementations
   └── starts presentation/runtime

presentation  ──>  application  ──>  domain
infrastructure ──implements application ports──> application
infrastructure ────────────────────────────────> domain
```

## Layers

- `domain/` contains desktop, workspace, window, geometry, and session state. It has no dependency on application orchestration, terminal UI, or infrastructure.
- `application/` contains app state, actions, settings, and use cases. It depends on domain state and defines outbound contracts in `ports.rs`.
- `presentation/` owns terminal event handling, shortcut resolution, input conversion, rendering, and UI. It translates TUI events into application calls and renders application/domain state.
- `infrastructure/` implements terminal process and session persistence ports and loads application settings.
- `main.rs` is the composition root: it loads configuration/session adapters, creates the terminal backend, and injects those implementations into presentation.

Keybinding defaults live in application settings. Infrastructure loads the optional `[keybindings]` section from `config.toml` over those defaults, so unspecified shortcuts retain their built-in values.

## Dependency rules

1. Domain modules must not import application, presentation, or infrastructure.
2. Application modules may depend on domain and application-owned ports, never on infrastructure or presentation implementations.
3. Presentation may call application use cases and read domain state; it must not reach into infrastructure implementations.
4. Infrastructure may depend on domain and application-owned contracts/models when implementing adapters. Application code must not construct infrastructure implementations.
5. `main.rs` may depend on every layer to assemble the application.

The application ports use tuicli-owned terminal screen and input data contracts. PTY, crossterm, vt100, and ratatui types stay in their outer adapters.

Legacy top-level module re-exports in `lib.rs` remain temporarily for callers using the previous crate paths. New internal code should use the layer-qualified paths.
