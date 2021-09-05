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

    pub fn reset_timer(&mut self, mvp_name: String, map: String) -> Result<(), String> {
        let mvp_timers: &mut HashMap<String, u64> = match self.timers.get_mut(&mvp_name) {
            None => {
                return Err(format!(
                    "MvP {} doesn't have any active respawn times",
                    mvp_name
                ))
            }
            Some(mvp_timers) => mvp_timers,
        };

        match mvp_timers.remove(&map) {
            None => Err(format!(
                "MvP {} doesn't have any active respawn times at {}",
                mvp_name, map
            )),
            Some(_v) => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const DEFAULT_MVP_NAME: &str = "mistress";
    const DEFAULT_MAP: &str = "mjolnir_04";
    const DEFAULT_TIME: u64 = 120;

    fn make_timer(
        base_hashmap: &mut HashMap<String, HashMap<String, u64>>,
        mvp_name: String,
        map: String,
        time: u64,
    ) -> &mut HashMap<String, HashMap<String, u64>> {
        let expected_mvp_entry: &mut HashMap<String, u64> = &mut HashMap::new();
        expected_mvp_entry.insert(map, time);

        base_hashmap.insert(mvp_name, expected_mvp_entry.clone());

        base_hashmap
    }

    #[test]
    fn starts_a_timer() {
        let base_hashmap: &mut HashMap<String, HashMap<String, u64>> = &mut HashMap::new();
        let expected_timers = make_timer(
            base_hashmap,
            String::from(DEFAULT_MVP_NAME),
            String::from(DEFAULT_MAP),
            DEFAULT_TIME,
        );
        let mut guild_data = GuildData::new();
        let result = guild_data.start_timer(
            String::from(DEFAULT_MVP_NAME).to_uppercase(),
            String::from(DEFAULT_MAP),
        );

        assert_eq!(result, Ok(()));
        assert_eq!(guild_data.timers, expected_timers.clone());
    }

    #[test]
    fn resets_a_timer() {
        let base_hashmap: &mut HashMap<String, HashMap<String, u64>> = &mut HashMap::new();
        let initial_timer = make_timer(
            base_hashmap,
            String::from(DEFAULT_MVP_NAME),
            String::from(DEFAULT_MAP),
            DEFAULT_TIME,
        );

        let mut guild_data = GuildData {
            config: mvp::get_mvp_data(),
            timers: initial_timer.clone(),
        };

        let result =
            guild_data.reset_timer(String::from(DEFAULT_MVP_NAME), String::from(DEFAULT_MAP));

        initial_timer
            .entry(String::from(DEFAULT_MVP_NAME))
            .or_default()
            .remove(DEFAULT_MAP);

        assert_eq!(result, Ok(()));
        assert_eq!(guild_data.timers, initial_timer.clone());
    }
}
