/// TelegramClient provides methods to send messages to Telegram groups or topics.
///
/// # Overview
/// This client uses the Telegram Bot API to send messages. It wraps around the
/// `reqwest::blocking::Client` to perform HTTP requests and utilizes configuration
/// values from `CONFIG` to set the bot token and parse mode.
///
/// # Example
/// ```no_run
/// use telemon::TelegramClient;
/// let client = TelegramClient::new();
/// client.send_to_group(123456789, "Hello, group!").unwrap();
/// client.send_to_topic(123456789, 987654321, "Hello, topic!").unwrap();
/// ```
use crate::config::config::CONFIG;
use reqwest::blocking::Client;
use serde::Serialize;

/// A client for sending messages through the Telegram Bot API.
pub struct TelegramClient {
    client: Client,
    token: String,
}
/// Payload used to send a message via Telegram Bot API.
#[derive(Serialize)]
struct SendMessagePayload<'a> {
    /// The chat ID where the message will be sent.
    chat_id: i64,
    /// The text of the message to be sent.
    text: &'a str,
    /// Optional ID of the message thread (used for topics).
    message_thread_id: Option<i64>,
    /// The parse mode for the message (e.g., "HTML" or "Markdown").
    parse_mode: &'a str,
}

impl TelegramClient {
    /// Creates a new `TelegramClient` using the token defined in global `CONFIG`.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            token: CONFIG.token.clone(),
        }
    }
    /// Sends a message to the specified chat or topic.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat ID where the message will be sent.
    /// * `message` - The message content.
    /// * `message_thread_id` - Optional thread ID for topic-specific messages.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or Telegram returns an error response.
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
    /// Sends a message to a specific thread in a Telegram topic.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat ID of the group.
    /// * `thread_id` - The ID of the message thread (topic).
    /// * `message` - The message content.
    ///
    /// # Errors
    ///
    /// Returns an error if sending the message fails.
    pub fn send_to_topic(
        &self,
        chat_id: i64,
        thread_id: i64,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.send(chat_id, message, Some(thread_id))
    }
    /// Sends a message directly to a Telegram group without using a topic.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat ID of the group.
    /// * `message` - The message content.
    ///
    /// # Errors
    ///
    /// Returns an error if sending the message fails.
    pub fn send_to_group(
        &self,
        chat_id: i64,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.send(chat_id, message, None)
    }
}