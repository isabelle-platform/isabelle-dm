use yew::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct ScheduleEntry {
    #[serde(default = "unset_id")]
    pub id: u64,

    #[serde(default = "unset_time")]
    pub time: u64,

    #[serde(default = "unset_bool_params")]
    pub bool_params: HashMap<String, bool>,
    #[serde(default = "unset_id_params")]
    pub id_params: HashMap<String, u64>,
    #[serde(default = "unset_str_params")]
    pub str_params: HashMap<String, String>,
}

unsafe impl Send for ScheduleEntry {}

impl ScheduleEntry {
    pub fn new() -> Self {
        Self {
            id: u64::MAX,
            time: 0,
            bool_params: HashMap::new(),
            id_params: HashMap::new(),
            str_params: HashMap::new(),
        }
    }
}

fn unset_id() -> u64 {
    return u64::MAX;
}

fn unset_time() -> u64 {
    return 0;
}

fn unset_bool_params() -> HashMap<String, bool> {
    return HashMap::new()
}

fn unset_id_params() -> HashMap<String, u64> {
    return HashMap::new()
}

fn unset_str_params() -> HashMap<String, String> {
    return HashMap::new()
}
