use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct MvpConfig {
    pub name: String,
    pub respawn: HashMap<String, RespawnData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespawnData {
    pub map: String,
    pub map_name: String,
    pub time: Option<u64>,
    pub time_var: Option<u64>,
}

pub fn get_respawn_data<'a>(
    config: &'a HashMap<String, MvpConfig>,
    mvp_name: &str,
    map: &str,
) -> Option<&'a RespawnData> {
    let mvp_config = match config.get(&mvp_name.to_lowercase()) {
        None => return None,
        Some(mvp_entry) => mvp_entry,
    };

    mvp_config.respawn.get(&map.to_lowercase())
}

pub fn get_mvp_data() -> HashMap<String, MvpConfig> {
    let bytes = include_bytes!("../../assets/mvps.json");
    serde_json::from_slice(bytes).expect("Error reading MvP data file")
}
