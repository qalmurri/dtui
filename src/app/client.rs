pub struct ClientState {
    pub alias: String,

    pub name: String,
    pub version: String,
    pub last_online: String,
    pub ip_address: String,
}

impl ClientState {
    pub fn new() -> Self {
        Self {
            alias: "Anonymous-7F3A".to_string(),

            name: "Anonymous Chat".to_string(),
            version: "0.1.0".to_string(),
            last_online: "Never".to_string(),

            ip_address: "Not connected".to_string(),
        }
    }
}
