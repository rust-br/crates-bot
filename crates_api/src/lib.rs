extern crate reqwest;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::error;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct Crate {
    pub name: String,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
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

pub fn query(query_string: String) -> Result<Crates, CratesError> {
    let crates: Crates = reqwest::get(
        format!("https://crates.io/api/v1/crates?q={}", query_string).as_str(),
    )?
        .json()?;

    Ok(crates)
}
