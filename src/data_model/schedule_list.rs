use yew::prelude::*;
use crate::data_model::schedule_entry::ScheduleEntry;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct ScheduleList {
    pub schedule_entries: Vec<ScheduleEntry>,
}
