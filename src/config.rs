use serde::Deserialize;
use toml::de::Deserializer;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub user: String,
    pub servers: Vec<String>,
    pub reference_server: String,
}

pub fn load_config() -> Config {
    let config_str = include_str!("config.toml");
    let mut deserializer = Deserializer::new(&config_str);
    Config::deserialize(&mut deserializer).expect("Failed to parse config")
}
