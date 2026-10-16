use std::{error::Error, io};

use crate::domain::session::SessionState;
use crate::domain::window_manager::{Geometry, Window};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalColor {
    Default,
    Indexed(u8),
    Rgb(u8, u8, u8),
}

pub struct TerminalCell<'a> {
    pub contents: &'a str,
    pub foreground: TerminalColor,
    pub background: TerminalColor,
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub inverse: bool,
    pub has_contents: bool,
    pub wide_continuation: bool,
}

pub trait TerminalScreen {
    fn cell(&self, row: u16, column: u16) -> Option<TerminalCell<'_>>;
    fn cursor_position(&self) -> (u16, u16);
    fn hide_cursor(&self) -> bool;
}

/// Outbound port for terminal process and virtual-screen operations.
pub trait TerminalBackend {
    fn spawn_for_window(
        &mut self,
        window_id: u64,
        geometry: Geometry,
    ) -> Result<(), Box<dyn Error>>;
    fn remove(&mut self, window_id: u64);
    /// Process pending PTY output and report whether any visible screen changed.
    fn poll(&mut self) -> bool;
    fn screen(&self, window_id: u64) -> Option<&dyn TerminalScreen>;
    fn is_exited(&self, window_id: u64) -> bool;
    fn exited_window_ids(&self) -> Vec<u64>;
    fn write_input(&mut self, window_id: u64, bytes: &[u8]) -> io::Result<()>;
    fn resize_window(&mut self, window_id: u64, geometry: Geometry) -> Result<(), Box<dyn Error>>;
    fn resize_all(
        &mut self,
        windows: &[Window],
        area_width: u16,
        area_height: u16,
    ) -> Result<(), Box<dyn Error>>;
    fn start_for_windows(&mut self, windows: &[Window]) -> Vec<(u64, String)>;
    fn close_processes(&mut self);
}

/// Outbound port for persisted desktop sessions.
pub trait SessionRepository {
    fn load(&self) -> Result<Option<SessionState>, Box<dyn Error>>;
    fn save(&self, session: &SessionState) -> Result<(), Box<dyn Error>>;
}
