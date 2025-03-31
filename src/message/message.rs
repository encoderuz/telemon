/// A simple message dispatch layer for sending messages to Telegram topics or groups.
///
/// This module provides an ergonomic wrapper around [`TelegramClient`] for sending
/// messages using `.to(...)` and `.to_group()` methods with built-in configuration via `telemon.toml`.
///
/// # Example `telemon.toml`
/// ```toml
/// token = "your-telegram-bot-token"
/// chat_id = 123456789         # used in `.to(topic_id)`
/// group_id = 987654321        # used in `.to_group()`
/// show_logs = true
/// parse_mode = "HTML"
/// ```
///
/// # Usage
/// ```no_run
/// use telemon::Telemon;
///
/// // Send to a topic using chat_id from config
/// Telemon::send("Hello Topic!").to(1234534435);
///
/// // Send to a topic with explicit chat_id and topic_id
/// Telemon::send("Hello with chat!").to((1234534435, 1234534435));
///
/// // Send to a group using group_id from config
/// Telemon::send("Hello Group!").to_group();
/// ```

use crate::{client::client::TelegramClient, config::config::CONFIG};
/// A builder-style struct representing a message to be sent.
pub struct TelemonMessage<'a> {
    /// The text content of the message.
    text: &'a str,
}
/// Enum representing where to send the message.
pub enum ToTarget {
    /// Send to a topic using `chat_id` from config and explicit `topic_id`.
    Topic(i64),
    /// Send to a specific `chat_id` and `topic_id`.
    ChatWithTopic(i64, i64),
}
/// Internal struct to hold message and client.
pub struct SendTarget<'a> {
    msg: &'a str,
    client: TelegramClient,
}

impl<'a> TelemonMessage<'a> {
    /// Sends the message to the specified target (topic or chat with topic).
    ///
    /// Accepts either:
    /// - `i64` representing topic ID (uses `chat_id` from config)
    /// - `(i64, i64)` tuple representing `(chat_id, topic_id)`
    ///
    /// # Panics
    /// Panics if `chat_id` is missing in the config when using `i64`.
    pub fn to(self, target: impl Into<ToTarget>) {
        let client = TelegramClient::new();
        match target.into() {
            ToTarget::Topic(topic_id) => {
                if let Some(chat_id) = CONFIG.chat_id {
                    let _ = client.send_to_topic(chat_id, topic_id, self.text);
                } else {
                    if CONFIG.show_logs {
                        eprintln!(
                            "⚠️chat_id not found! The `chat_id` field is missing in telemon.toml. Please use `.to((chat_id, topic_id))` instead."
                        );
                    }
                }
            }
            ToTarget::ChatWithTopic(chat_id, topic_id) => {
                let _ = match client.send_to_topic(chat_id, topic_id, self.text) {
                    Ok(data) => {
                        if CONFIG.show_logs {
                            println!("{:?}", data);
                        }
                    }
                    Err(err) => {
                        if CONFIG.show_logs {
                            eprintln!("{:?}", err);
                        }
                    }
                };
            }
        }
    }
    /// Sends the message to the group defined in the config (`group_id`).
    ///
    /// # Errors
    /// Logs an error if `group_id` is missing or the message fails to send.
    pub fn to_group(self) {
        let client = TelegramClient::new();
        if let Some(group_id) = CONFIG.group_id.clone() {
            let _ = match client.send_to_group(group_id, self.text) {
                Ok(_) => {
                    if CONFIG.show_logs {
                        println!("- ✅ Message sent to group \n- ℹ️Group id: {}\n- ⚠️ You can turn off these logs by setting show_logs = false in the telemon.toml file.", &group_id);
                    }
                }
                Err(err) => {
                    if CONFIG.show_logs {
                        eprintln!("❌ Error sending to group: {:?}\n ⚠️ You can turn off these logs by setting show_logs = false in the telemon.toml file.", err);
                    }
                }
            };
        } else if CONFIG.show_logs {
            eprintln!("⚠️ group_id is missing in the config file. \n👀Make sure the group_id is set in the telemon.toml file. \n");
        }
    }
}

impl From<i64> for ToTarget {
    fn from(topic_id: i64) -> Self {
        ToTarget::Topic(topic_id)
    }
}

impl From<(i64, i64)> for ToTarget {
    fn from(tuple: (i64, i64)) -> Self {
        ToTarget::ChatWithTopic(tuple.0, tuple.1)
    }
}

/// Entry point struct to start building a message.
pub struct Telemon;

impl Telemon {
    /// Creates a new [`TelemonMessage`] with the given message text.
    ///
    /// # Example
    /// ```rust
    /// use telemon::Telemon;
    /// Telemon::send("Hello").to(1234534435);
    /// ```
    pub fn send(text: &str) -> TelemonMessage {
        TelemonMessage { text }
    }
}
