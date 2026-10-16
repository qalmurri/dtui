#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopStatus {
    pub message: String,
}

impl Default for DesktopStatus {
    fn default() -> Self {
        Self {
            message: "Ready".into(),
        }
    }
}
