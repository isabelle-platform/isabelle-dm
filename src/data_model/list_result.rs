use crate::data_model::item::Item;
use crate::util::accessor::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct ListResult {
    #[serde(default = "unset_item_map")]
    pub map: HashMap<u64, Item>,

    #[serde(default = "unset_u64")]
    pub total_count: u64,
}
