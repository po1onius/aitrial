use toml;
use core::panic;
use std::{path::{self, PathBuf}, str::FromStr, sync::OnceLock};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Config {
    pub dataset_path: String,
    pub judge_url: String,
    pub fy_port: u32,
    pub atk_type: Vec<String>,
    pub atk_mode: Vec<String>
}


pub fn get_config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        let config_path = PathBuf::from_str("config.toml").unwrap();
        if config_path.exists() {
            let config = std::fs::read_to_string(&config_path).unwrap();
            toml::from_str(config.as_str()).unwrap()
        } else {
            panic!("config error");
        }
    })
}
