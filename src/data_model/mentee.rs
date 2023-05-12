use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct Mentee {
    #[serde(default = "unset_id")]
    pub id: u64,
    #[serde(default = "empty_string")]
    pub name: String,
    #[serde(default = "empty_string")]
    pub notes: String,
    #[serde(default = "empty_string")]
    pub food: String,
}

unsafe impl Send for Mentee {}

impl Mentee {
    pub fn new() -> Self {
        Self {
            id: u64::MAX,
            name: String::new(),
            notes: String::new(),
            food: String::new(),
        }
    }
}

fn unset_id() -> u64 {
    return u64::MAX;
}

fn empty_string() -> String {
    return "".to_string();
}
