# 📦 Telemon

**Telemon** is a lightweight Rust library for sending logs or messages to a Telegram group using a bot. It supports topic-based messaging and flexible configuration through a `telemon.toml` file.

## 🚀 Features

- Send messages to Telegram topics
- Configuration via `telemon.toml`
- Optional logging to console
- Flexible usage with or without global `chat_id`

## 🛠 Configuration

Create a `telemon.toml` file in your project root:

```toml
token = "BOT_TOKEN"          # Your Telegram bot token
chat_id = -1001234567890     # (Optional) Telegram group chat ID
show_logs = false            # Set to true to display logs in console
parse_mode = "MarkdownV2"      # (Optional) Message format: Markdown, HTML, or empty for plain text
```

- `token`: The bot token provided by BotFather
- `chat_id`: (Optional) The group chat ID where messages will be sent
- `show_logs`: If true, logs will also be printed to the terminal
- `parse_mode`: Defines the message format sent to Telegram. Accepted values are: `Markdown`, `MarkdownV2`, `Html`

Markdown – Enables Markdown formatting

HTML – Enables HTML formatting

Leave empty or omit for plain text

## 📤 Usage

You can send messages in two ways, depending on whether `chat_id` is defined in the config.

### 1. If `chat_id` is set in the config

Use `.to(topic_id)` — the `chat_id` will be automatically read from the config:

```rust
use telemon::Telemon;

Telemon::send("✅ Success").to(34);
```

### 2. If `chat_id` is **not** set in the config

You must provide the `chat_id` as the first value in a tuple:

```rust
use telemon::Telemon;

Telemon::send("🚨 Error occurred").to((-1002483629475, 34));
```

In this case:
- The first value is the `chat_id`
- The second value is the topic ID within the group

## 📌 Notes

- If `show_logs = true`, every message sent will also be printed to the terminal.
- If `show_logs = false`, logs will be silent.
- `topic_id` refers to the forum topic ID inside the Telegram group.

## 📄 License

MIT License
