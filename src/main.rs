use telemon::config::config::Config;
use telemon::Telemon;

fn main() {
    // chat_id configda mavjud bo‘lsa
    Telemon::send("✅ Success").to(34);

    // configda chat_id yo‘q bo‘lsa
    Telemon::send("🚨 Xatolik").to((-1002483629475, 34));
}