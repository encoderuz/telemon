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
                    let _ = client.send_message(chat_id, topic_id, self.text);
                } else {
                    if CONFIG.show_logs {
                        eprintln!(
                            "⚠️chat_id not found! The `chat_id` field is missing in telemon.toml. Please use `.to((chat_id, topic_id))` instead."
                        );
                    }
                }
            }
            ToTarget::ChatWithTopic(chat_id, topic_id) => {
                let _ = match client.send_message(chat_id, topic_id, self.text) {
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
