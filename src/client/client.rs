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
    message_thread_id: i64,
    parse_mode: &'a str,
}

impl TelegramClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            token: CONFIG.token.clone(),
        }
    }
    pub fn escape_markdown_v2(text: &str) -> String {
        let special_chars = r"_*[]()~`>#+-=|{}.!";

        let mut escaped = String::with_capacity(text.len());
        for c in text.chars() {
            if special_chars.contains(c) {
                escaped.push('\\');
            }
            escaped.push(c);
        }
        escaped
    }
    pub fn send_message(
        &self,
        chat_id: i64,
        thread_id: i64,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.token);
        let payload = SendMessagePayload {
            chat_id,
            text: message,
            message_thread_id: thread_id,
            parse_mode: &CONFIG.parse_mode,
        };

        let res = self.client.post(&url).json(&payload).send()?;
        if res.status().is_success() {
            if CONFIG.show_logs {
                println!("{:?}", res);
            }
            Ok(())
        } else {
            let response = res;
            let status = &response.status();
            let text = response.text()?;
            if CONFIG.show_logs {
                eprintln!("Telegram error ({}): {}", status, text);
            }
            Err(format!("Telegram error: {}", text).into())
        }
    }
}
