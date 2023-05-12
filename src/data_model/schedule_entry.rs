use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct ScheduleEntry {
    #[serde(default = "unset_id")]
    pub id: u64,
    #[serde(default = "set_false")]
    pub is_group: bool,
    #[serde(default = "empty_u64_set")]
    pub mentees: Vec<u64>,
    #[serde(default = "empty_u64_set")]
    pub users: Vec<u64>,
    #[serde(default = "empty_u64_set")]
    pub teachers: Vec<u64>,
    #[serde(default = "empty_u64_set")]
    pub times: Vec<u64>,
    #[serde(default = "unset_duration")]
    pub duration: u64,
    #[serde(default = "empty_string")]
    pub notes: String,
}

unsafe impl Send for ScheduleEntry {}

impl ScheduleEntry {
    pub fn new() -> Self {
        Self {
            id: u64::MAX,
            is_group: false,
            mentees: Vec::new(),
            users: Vec::new(),
            teachers: Vec::new(),
            times: Vec::new(),
            duration: u64::MAX,
            notes: String::new(),
        }
    }
}

fn unset_id() -> u64 {
    return u64::MAX;
}

fn unset_duration() -> u64 {
    return 0;
}

fn empty_string() -> String {
    return "".to_string();
}

fn set_false() -> bool {
    return false;
}

fn empty_u64_set() -> Vec<u64> {
    return Vec::new();
}