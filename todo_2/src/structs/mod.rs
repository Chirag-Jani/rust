use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub created_at: String,
    pub last_updated: String,
    pub title: String,
    pub is_completed: bool,
    pub uid: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub data: Vec<TodoItem>,
}
