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
pub enum CratesError {
    RequestError(reqwest::Error),
    DeserializeError(serde_json::Error),
}

impl From<reqwest::Error> for CratesError {
    fn from(err: reqwest::Error) -> CratesError {
        CratesError::RequestError(err)
    }
}

impl From<serde_json::Error> for CratesError {
    fn from(err: serde_json::Error) -> CratesError {
        CratesError::DeserializeError(err)
    }
}

impl error::Error for CratesError {
    fn description(&self) -> &str {
        match *self {
            CratesError::RequestError(ref req_err) => req_err.description(),
            CratesError::DeserializeError(ref serde_err) => serde_err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CratesError::RequestError(ref req_err) => Some(req_err),
            CratesError::DeserializeError(ref serde_err) => Some(serde_err),
        }
    }
}

impl fmt::Display for CratesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CratesError::DeserializeError(ref serde_err) => serde_err.fmt(f),
            CratesError::RequestError(ref req_err) => req_err.fmt(f),
        }
    }
}

pub fn search(crate_name: &str) -> Result<Crates, CratesError> {
    let crates: Crates = reqwest::get(
        &format!("https://crates.io/api/v1/crates?q={}", crate_name),
    )?
        .json()?;

    Ok(crates)
}
