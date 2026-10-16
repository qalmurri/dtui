use crate::domain::desktop::workspace::WorkspaceId;

use super::manager::WindowManager;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ResizeDelta {
    pub width: i16,
    pub height: i16,
}

impl ResizeDelta {
    pub const fn new(width: i16, height: i16) -> Self {
        Self { width, height }
    }

    pub const fn left() -> Self {
        Self::new(-1, 0)
    }

    pub const fn right() -> Self {
        Self::new(1, 0)
    }

    pub const fn up() -> Self {
        Self::new(0, -1)
    }

    pub const fn down() -> Self {
        Self::new(0, 1)
    }

    pub const fn is_zero(self) -> bool {
        self.width == 0 && self.height == 0
    }
}

pub fn apply(
    manager: &mut WindowManager,
    delta: ResizeDelta,
    area_width: u16,
    area_height: u16,
    workspace_id: WorkspaceId,
) {
    if delta.is_zero() {
        return;
    }

    manager.resize_focused(
        delta.width,
        delta.height,
        area_width,
        area_height,
        workspace_id,
    );
}
