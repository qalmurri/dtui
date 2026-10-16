pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

// Compatibility re-exports for common types and modules from the old layout.
pub use application as app;
pub use domain::{desktop, session, window_manager};
pub use infrastructure::config;
pub use presentation::Tui;
pub use presentation::controller::keybindings;
pub use presentation::{controller, terminal as core, ui};
