use std::{fs, process::Command};

use serde::{Deserialize, Serialize};

pub const PROFILES: usize = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    #[serde(skip_serializing)]
    pub number_of_profiles: usize,
    pub current_profile: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            number_of_profiles: PROFILES,
            current_profile: Default::default(),
        }
    }
}

impl Config {
    pub fn new(profile: usize) -> Self {
        Self {
            number_of_profiles: PROFILES,
            current_profile: profile,
        }
    }

    pub fn next(&mut self) {
        if self.current_profile >= self.number_of_profiles {
            self.current_profile = 0;
            return ();
        }

        self.current_profile += 1;
    }

    pub fn load() -> Self {
        let data = fs::read_to_string(home::home_dir().unwrap().join(".macro_scriptx")).unwrap();
        let mut value: Config = serde_json::from_str(&data).unwrap();

        value.number_of_profiles = PROFILES;

        value
    }

    pub fn save(&self) {
        let data = serde_json::to_string_pretty(&self).unwrap();

        fs::write(home::home_dir().unwrap().join(".macro_scriptx"), data).unwrap();
    }
}

pub fn open(app: &str) {
    Command::new(app).spawn().unwrap();
}

pub fn toggle_audio(source: &str) {
    match source {
        "audio" => {
            Command::new("amixer")
                .args(["sset", "Master", "toggle"])
                .spawn()
                .unwrap();
        }
        "mic" => {
            Command::new("amixer")
                .args(["-D", "pulse", "sset", "Capture", "toggle"])
                .spawn()
                .unwrap();
        }
        _ => unreachable!(),
    }
}
