pub mod action;
mod actions;
pub mod ports;
mod session;
pub mod settings;
mod window_actions;
mod workspace;

use crate::{
    application::ports::TerminalBackend,
    domain::{desktop::DesktopState, window_manager::WindowManager},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceInputMode {
    Rename,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceInputKey {
    Escape,
    Enter,
    Backspace,
    Character(char),
    Other,
}

#[derive(Debug, Clone)]
pub struct WorkspaceInput {
    pub mode: WorkspaceInputMode,
    pub value: String,
}

pub struct AppState {
    pub running: bool,
    pub show_help: bool,
    pub desktop: DesktopState,
    pub window_manager: WindowManager,
    pub workspace_input: Option<WorkspaceInput>,
    pub terminals: Box<dyn TerminalBackend>,
    pub terminal_input_mode: bool,
}

impl AppState {
    pub fn new(terminals: Box<dyn TerminalBackend>) -> Self {
        Self {
            running: true,
            show_help: false,
            desktop: DesktopState::default(),
            window_manager: WindowManager::default(),
            workspace_input: None,
            terminals,
            terminal_input_mode: true,
        }
    }
}

pub use actions::handle;
pub use session::{restore_or_initialize, to_session_state};
pub use workspace::handle_input as handle_workspace_input;
