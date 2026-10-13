use serde::{Deserialize, Serialize};

use super::geometry::Geometry;
use crate::desktop::workspace::WorkspaceId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Window {
    pub id: u64,
    pub title: String,
    pub geometry: Geometry,
    pub workspace_id: WorkspaceId,
}

impl Window {
    pub fn new(
        id: u64,
        title: impl Into<String>,
        geometry: Geometry,
        workspace_id: WorkspaceId,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            geometry,
            workspace_id,
        }
    }
}
