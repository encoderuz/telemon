use crate::{client::client::TelegramClient, config::config::CONFIG};
use std::error::Error;

pub struct TelemonMessage<'a> {
    text: &'a str,
}
pub enum ToTarget {
    Topic(i64),
    ChatWithTopic(i64, i64),
}
pub struct SendTarget<'a> {
    msg: &'a str,
    client: TelegramClient,
}

impl<'a> TelemonMessage<'a> {
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

pub struct Telemon;

impl Telemon {
    pub fn send(text: &str) -> TelemonMessage {
        TelemonMessage { text }
    }
}
