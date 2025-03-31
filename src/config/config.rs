use std::fs;
use std::path::Path;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub token: String,
    pub chat_id: Option<i64>,
    #[serde(default)]
    pub show_logs: bool,
    pub parse_mode: String,
}
impl Config {
    pub fn get() -> Self {
        Self{
            token: CONFIG.token.clone(),
            chat_id: CONFIG.chat_id,
            show_logs: CONFIG.show_logs,
            parse_mode: CONFIG.parse_mode.clone(),
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let path = Path::new("telemon.toml");
    let content = fs::read_to_string(path).expect("telemon.toml file not found");
    toml::from_str(&content).expect("Failed to parse telemon.toml")
});