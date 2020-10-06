use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Crate {
    pub name: String,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub recent_downloads: u32,
    pub downloads: u32,
}

#[derive(Deserialize, Debug)]
pub struct Crates {
    pub crates: Vec<Crate>,
}

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    DeserializeError(serde_json::Error),
    TelegramError(telegram_bot::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::RequestError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::DeserializeError(err)
    }
}

pub async fn search(client: &reqwest::Client, crate_name: &str) -> Result<Crates, reqwest::Error> {
    let req = client
        .get(&format!("https://crates.io/api/v1/crates?q={}", crate_name))
        .query(&[("q", crate_name)])
        .build()?;

    client.execute(req).await?.json().await
}
