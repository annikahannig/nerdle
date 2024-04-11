use anyhow::Result;
use gloo::net::http::Request;
use serde::Deserialize;
use web_sys::RequestMode;

use crate::state::Wordlist;

pub async fn load_wordlist() -> Result<Wordlist> {
    let request = Request::get("/data/words.txt");
    let response = request.send().await?;
    let text = response.text().await?;
    let wordlist: Wordlist = Wordlist::from(text);
    Ok(wordlist)
}

impl Wordlist {
    pub async fn fetch() -> Result<Wordlist> {
        let request = Request::get("/data/words.txt");
        let response = request.send().await?;
        let text = response.text().await?;
        let wordlist: Wordlist = Wordlist::from(text);
        Ok(wordlist)
    }
}

#[derive(Debug, Deserialize, Default, PartialEq, Clone)]
pub struct Wordle {
    pub days_since_launch: usize,
    pub editor: String,
    pub id: u32,
    pub print_date: String,
    pub solution: String,
}

pub async fn load_wordle() -> Result<Wordle> {
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let url = format!("data/{}.json", today);
    let response = Request::get(&url)
        .referrer("")
        .mode(RequestMode::NoCors)
        .send()
        .await?;
    let wordle: Wordle = response.json().await?;
    Ok(wordle)
}
