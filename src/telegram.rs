const BOT_TOKEN: &str = "";
const CHAT_ID: &str = "";

pub fn send(content: &str) {
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
        BOT_TOKEN, CHAT_ID, content
    );
    reqwest::blocking::get(&url).unwrap();
}
