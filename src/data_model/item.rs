use yew::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::util::accessor::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct Item {
    #[serde(default = "unset_id")]
    pub id: u64,

    /* Roles: admin, staff, teacher, student, horse */

    #[serde(default = "unset_str_map")]
    pub fields: HashMap<String, String>,

    #[serde(default = "unset_bool_map")]
    pub bool_params: HashMap<String, bool>,

    #[serde(default = "unset_id_map")]
    pub id_params: HashMap<String, u64>,
}

unsafe impl Send for Item {}

impl Item {
    pub fn new() -> Self {
        Self {
            id: u64::MAX,
            fields: HashMap::new(),
            bool_params: HashMap::new(),
            id_params: HashMap::new(),
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

    pub fn safe_bool(&self, name: &str, def: bool) -> bool {
        if self.bool_params.contains_key(name) {
            self.bool_params[name].clone()
        }
        else {
            def
        }
    }

    pub fn safe_id(&self, name: &str, def: u64) -> u64 {
        if self.id_params.contains_key(name) {
            self.id_params[name].clone()
        }
        else {
            def
        }
    }
}

