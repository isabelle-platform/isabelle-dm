use std::collections::HashMap;
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use crate::util::accessor::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct AllSettings {
	#[serde(default = "unset_str_map")]
    pub str_params: HashMap<String, String>,
    #[serde(default = "unset_bool_map")]
    pub bool_params: HashMap<String, bool>,
    #[serde(default = "unset_id_map")]
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

    pub fn safe_str(self, key: &str, def: &str) -> String {
	    if self.str_params.contains_key(key) {
	        return self.str_params[key].clone();
	    }
	    return def.to_string();
	}

	pub fn safe_bool(self, key: &str, def: bool) -> bool {
	    if self.bool_params.contains_key(key) {
	        return self.bool_params[key].clone();
	    }
	    return def;
	}

	pub fn safe_id(self, key: &str, def: u64) -> u64 {
	    if self.id_params.contains_key(key) {
	        return self.id_params[key].clone();
	    }
	    return def;
	}
}
