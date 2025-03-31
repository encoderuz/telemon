//! Configuration loader for the Telemon crate.
//!
//! Loads configuration values from a `telemon.toml` file located in the root directory.
//! This module uses `once_cell::sync::Lazy` to lazily initialize the global configuration instance (`CONFIG`).
//!
//! # Example `telemon.toml`
//! ```toml
//! token = "your-telegram-bot-token"
//! chat_id = your telegram group chat_id
//! show_logs = true|false
//! parse_mode = "HTML"
//! group_id = your telegram group id
//! ```
//!
//! # Usage
//! ```rust
//! use telemon::Config;
//! let config = Config::get();
//! println!("Bot token: {}", config.token);
//! ```

use std::fs;
use std::path::Path;
use once_cell::sync::Lazy;
use serde::Deserialize;

/// The configuration structure for the application, populated from `telemon.toml`.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// The Telegram bot token.
    pub token: String,
    /// Optional default chat ID for sending messages.
    pub chat_id: Option<i64>,
    /// Whether to show logs during message sending (default: `false`).
    #[serde(default)]
    pub show_logs: bool,
    /// Optional parse mode for message formatting (e.g., `HTML`, `Markdown`).
    pub parse_mode: Option<String>,
    /// Optional group ID used for sending group messages.
    pub group_id: Option<i64>,
}
impl Config {
    /// Returns a copy of the global `CONFIG` instance.
    ///
    /// # Example
    /// ```rust
    /// use telemon::Config;
    /// let config = Config::get();
    /// if config.show_logs {
    ///     println!("Logging is enabled");
    /// }
    /// ```
    pub fn get() -> Self {
        Self{
            token: CONFIG.token.clone(),
            chat_id: CONFIG.chat_id,
            show_logs: CONFIG.show_logs,
            parse_mode: CONFIG.parse_mode.clone(),
            group_id: CONFIG.group_id,
        }
    }
}
/// Lazily loaded global configuration instance.
///
/// This reads and parses the `telemon.toml` file at startup.
/// Panics if the file is not found or fails to parse.
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let path = Path::new("telemon.toml");
    let content = fs::read_to_string(path).expect("telemon.toml file not found");
    toml::from_str(&content).expect("Failed to parse telemon.toml")
});
