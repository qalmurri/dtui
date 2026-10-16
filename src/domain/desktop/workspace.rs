use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkspaceId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: String,
}

impl Workspace {
    pub fn new(id: usize, name: impl Into<String>) -> Self {
        Self {
            id: WorkspaceId(id),
            name: name.into(),
        }
    }
}
