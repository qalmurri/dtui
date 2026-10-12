use std::collections::BTreeMap;
use super::application::{ApplicationId, ApplicationInfo};
#[derive(Debug, Default)]
pub struct ApplicationRegistry { entries: BTreeMap<String, ApplicationInfo> }
impl ApplicationRegistry {
    pub fn register(&mut self, info: ApplicationInfo) -> Result<(), String> {
        let key = info.id.0.clone();
        if self.entries.contains_key(&key) { return Err(format!("ID aplikasi '{key}' sudah terdaftar")); }
        self.entries.insert(key, info); Ok(())
    }
    pub fn get(&self, id: &ApplicationId) -> Option<&ApplicationInfo> { self.entries.get(&id.0) }
    pub fn iter(&self) -> impl Iterator<Item = &ApplicationInfo> { self.entries.values() }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
}
