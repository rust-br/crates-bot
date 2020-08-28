use serde::Deserialize;

use std::error;
use std::fmt;

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

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::RequestError(ref req_err) => req_err.description(),
            Error::DeserializeError(ref serde_err) => serde_err.description(),
            Error::TelegramError(ref telegram_bot_err) => telegram_bot_err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::RequestError(ref req_err) => Some(req_err),
            Error::DeserializeError(ref serde_err) => Some(serde_err),
            Error::TelegramError(ref telegram_bot_err) => Some(telegram_bot_err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DeserializeError(ref serde_err) => serde_err.fmt(f),
            Error::RequestError(ref req_err) => req_err.fmt(f),
            Error::TelegramError(ref telegram_bot_err) => telegram_bot_err.fmt(f),
        }
    }
}

pub async fn search(crate_name: &str) -> Result<Crates, reqwest::Error> {
    reqwest::get(&format!("https://crates.io/api/v1/crates?q={}", crate_name))
        .await?
        .json()
        .await
}
