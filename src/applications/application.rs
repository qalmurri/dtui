#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApplicationId(pub String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInfo {
    pub id: ApplicationId,
    pub name: String,
    pub description: String,
}
impl ApplicationInfo {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: ApplicationId(id.into()),
            name: name.into(),
            description: description.into(),
        }
    }
}
/// Kontrak aplikasi in-process. Aplikasi terminal eksternal memerlukan integrasi PTY.
pub trait Application {
    fn info(&self) -> &ApplicationInfo;
    fn on_start(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn on_stop(&mut self) -> Result<(), String> {
        Ok(())
    }
}
