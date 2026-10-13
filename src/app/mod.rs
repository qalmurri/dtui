mod actions;
pub(crate) mod pty;
mod runtime;
mod session;
mod workspace;

use crate::{desktop::DesktopState, window_manager::WindowManager};

use self::pty::TerminalManager;

pub use runtime::run;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceInputMode {
    Create,
    Rename,
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
    pub terminals: TerminalManager,
    pub terminal_input_mode: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            running: true,
            show_help: false,
            desktop: DesktopState::default(),
            window_manager: WindowManager::default(),
            workspace_input: None,
            terminals: TerminalManager::default(),
            terminal_input_mode: true,
        }
    }
}
