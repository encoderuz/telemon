use crate::config::config::CONFIG;
use reqwest::blocking::Client;
use serde::Serialize;

pub struct TelegramClient {
    client: Client,
    token: String,
}

#[derive(Serialize)]
struct SendMessagePayload<'a> {
    chat_id: i64,
    text: &'a str,
    message_thread_id: Option<i64>,
    parse_mode: &'a str,
}

impl TelegramClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            token: CONFIG.token.clone(),
        }
    }
    fn send(
        &self,
        chat_id: i64,
        message: &str,
        message_thread_id: Option<i64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.token);
        let payload = SendMessagePayload {
            chat_id,
            text: message,
            message_thread_id,
            parse_mode: &CONFIG.parse_mode.as_deref().unwrap_or("HTML"),
        };

        let res = self.client.post(&url).json(&payload).send()?;
        if res.status().is_success() {
            if CONFIG.show_logs {
                println!("𝌮 {:?}", res);
            }
            Ok(())
        } else {
            let status = res.status();
            let text = res.text()?;
            if CONFIG.show_logs {
                eprintln!("Telegram error ({}): {}", status, text);
            }
            Err(format!("Telegram error: {}", text).into())
        }
    }
    pub fn send_to_topic(
        &self,
        chat_id: i64,
        thread_id: i64,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.send(chat_id, message, Some(thread_id))
    }

    pub fn send_to_group(
        &self,
        chat_id: i64,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.send(chat_id, message, None)
    }
}
