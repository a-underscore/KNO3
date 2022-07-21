use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::Path,
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RoomData {
    pub id: String,
    pub top: Vec<String>,
    pub bottom: Vec<String>,
    pub left: Vec<String>,
    pub right: Vec<String>,
}

impl RoomData {
    pub fn load_rooms(path: &String) -> anyhow::Result<HashMap<String, Self>> {
        let mut rooms = HashMap::new();

        for file in fs::read_dir(path)? {
            let path = file?.path();
            let room = Self::from_file(&path)?;

            rooms.insert(room.id.clone(), room);
        }

        Ok(rooms)
    }

    fn from_file(path: &Path) -> anyhow::Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        let room = serde_json::from_str(content.as_str())?;

        Ok(room)
    }
}
