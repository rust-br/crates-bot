extern crate reqwest;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


#[derive(Deserialize, Debug)]
pub struct Crate {
    pub name: String,
    pub description: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Crates {
    pub crates: Vec<Crate>,
}

pub fn query(query_string: String) -> Crates {
    reqwest::get(
        format!("https://crates.io/api/v1/crates?q={}", query_string).as_str(),
    ).unwrap()
        .json()
        .unwrap()
}
