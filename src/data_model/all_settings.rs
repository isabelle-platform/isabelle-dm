use std::collections::HashMap;
use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct AllSettings {
    pub str_params: HashMap<String, String>,
    pub bool_params: HashMap<String, bool>,
    pub id_params: HashMap<String, u64>,
}

impl AllSettings {
    pub fn new() -> Self {
        Self {
            str_params: HashMap::new(),
            bool_params: HashMap::new(),
            id_params: HashMap::new(),
        }
    }
}
