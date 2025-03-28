# 📬 telemon

A Rust library for sending log messages to specific topics in Telegram groups.

## 🔧 Usage

```rust
use telemon::Telemon;

fn main() {
    // Send using topic_id only (chat_id must be set in `telemon.toml`)
    Telemon::send("❌ An error occurred").to(123456);

    // Send using both chat_id and topic_id manually
    Telemon::send("✅ Operation successful").to_with_chat(-1001234567890, 123456);
}
```
## 🔧 Configuration (telemon.toml)
`telemon.toml` file should contain in your project root folder
```.dotenv
token = "YOUR_BOT_TOKEN"
chat_id = "CHAT_ID"
```
