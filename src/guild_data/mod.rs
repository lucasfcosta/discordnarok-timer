use std::collections::HashMap;

mod mvp;

pub struct GuildData {
    config: HashMap<String, mvp::MvpConfig>,
    timers: HashMap<String, HashMap<String, u64>>,
}

impl Default for GuildData {
    fn default() -> Self {
        GuildData {
            config: mvp::get_mvp_data(),
            timers: HashMap::new(),
        }
    }
}

impl GuildData {
    pub fn new() -> Self {
        GuildData::default()
    }

    pub fn start_timer(&mut self, mvp_name: String, map: String) -> Result<(), String> {
        let respawn_data = mvp::get_respawn_data(&self.config, &mvp_name, &map);

        match respawn_data {
            None => return Err(format!("Respawn config for MvP {} not found", &mvp_name)),
            Some(respawn_data) => {
                let map_times = self
                    .timers
                    .entry(mvp_name.to_lowercase())
                    .or_insert_with(HashMap::new);

                match respawn_data.time {
                    None => return Err(format!("MvP {} doesn't have a respawn time", &mvp_name)),
                    Some(time) => {
                        map_times.insert(map.to_lowercase(), time);
                    }
                }

                Ok(())
            }
        }
    }

    pub fn reset_timer(&self, _mvp_name: String, _map: String) -> HashMap<String, mvp::MvpConfig> {
        mvp::get_mvp_data()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn starts_a_timer() {
        let expected_mistress_entry: &mut HashMap<String, u64> = &mut HashMap::new();
        expected_mistress_entry.insert("mjolnir_04".to_string(), 120);

        let expected_timers: &mut HashMap<String, HashMap<String, u64>> = &mut HashMap::new();
        expected_timers.insert("mistress".to_string(), expected_mistress_entry.clone());

        let mut guild_data = GuildData::new();
        let result = guild_data.start_timer("Mistress".into(), "mjolnir_04".into());

        assert_eq!(result, Ok(()));
        assert_eq!(guild_data.timers, expected_timers.clone());
    }
}
