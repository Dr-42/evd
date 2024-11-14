use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub eww_binary_path: String,
    pub poll_time: u64, // in ms
    pub notification_fade_time: u64,
    pub eww_window_name: String,
    pub eww_value_name: String,
    pub eww_value_string_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            eww_binary_path: String::from("eww"),
            notification_fade_time: 1000,
            poll_time: 50,
            eww_window_name: String::from("evd-frame"),
            eww_value_name: String::from("evd-content"),
            eww_value_string_name: String::from("evd-content-string"),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let user_config_dir = directories::ProjectDirs::from("com", "dr42", "evd");
        if let Some(dir) = user_config_dir {
            let config_path = dir.config_dir().join("config.toml");
            if !config_path.exists() {
                if std::fs::create_dir_all(dir.config_dir()).is_ok() {
                    let config = Config::default();
                    let config_str = toml::to_string(&config).unwrap();
                    std::fs::write(config_path, config_str).unwrap();
                    Ok(config)
                } else {
                    Err(String::from("Failed to create config directory"))
                }
            } else {
                let config = std::fs::read_to_string(&config_path);
                if let Ok(config) = config {
                    let config = toml::from_str(&config);
                    if let Ok(config) = config {
                        Ok(config)
                    } else {
                        std::fs::remove_file(&config_path).unwrap();
                        let config = Config::default();
                        let config_str = toml::to_string(&config).unwrap();
                        std::fs::write(&config_path, config_str).unwrap();
                        Ok(config)
                    }
                } else {
                    Err(String::from("Failed to read config file"))
                }
            }
        } else {
            Err(String::from("Failed to get user config directory"))
        }
    }
}
