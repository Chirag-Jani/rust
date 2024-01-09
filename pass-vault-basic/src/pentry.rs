use serde::{ Deserialize, Serialize };

pub struct ServiceInfo {
    pub username: String,
    pub password: String,
    pub service: String,
}

impl ServiceInfo {
    pub fn new(service: String, password: String, username: String) -> Self {
        ServiceInfo { username, password, service }
    }
}
