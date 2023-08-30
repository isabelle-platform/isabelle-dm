use yew::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct Item {
    #[serde(default = "unset_id")]
    pub id: u64,

    /* Roles: admin, staff, teacher, student, horse */

    #[serde(default = "unset_map")]
    pub fields: HashMap<String, String>,
}

unsafe impl Send for Item {}

impl Item {
    pub fn new() -> Self {
        Self {
            id: u64::MAX,
            fields: HashMap::new(),
        }
    }

    pub fn safe_str(&self, name: &str, def: String) -> String {
        if self.fields.contains_key(name) {
            self.fields[name].clone()
        }
        else {
            def
        }
    }
}

fn unset_map() -> HashMap<String, String> {
    return HashMap::new();
}

fn unset_id() -> u64 {
    return u64::MAX;
}
