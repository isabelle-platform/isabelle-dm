use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct User {
    #[serde(default = "unset_id")]
    pub id: u64,
    #[serde(default = "empty_string")]
    pub firstname: String,
    #[serde(default = "empty_string")]
    pub surname: String,
    #[serde(default = "empty_string")]
    pub phone: String,
    #[serde(default = "empty_string")]
    pub plan: String,
    #[serde(default = "empty_string")]
    pub notes: String,
}

unsafe impl Send for User {}

impl User {
    pub fn new() -> Self {
        Self {
            id: u64::MAX,
            firstname: String::new(),
            surname: String::new(),
            phone: String::new(),
            plan: String::new(),
            notes: String::new(),
        }
    }
}

fn unset_id() -> u64 {
    return u64::MAX;
}

fn empty_string() -> String {
    return "".to_string();
}
