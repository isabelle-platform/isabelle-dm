use crate::util::accessor::*;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct ListQuery {
    #[serde(default = "unset_str")]
    pub collection: String,

    #[serde(default = "unset_str")]
    pub context: String,

    #[serde(default = "unset_id")]
    pub id: u64,

    #[serde(default = "unset_id")]
    pub id_min: u64,

    #[serde(default = "unset_id")]
    pub id_max: u64,

    #[serde(default = "unset_u64_max")]
    pub skip: u64,

    #[serde(default = "unset_u64_max")]
    pub limit: u64,

    #[serde(default = "unset_u64_vec")]
    pub id_list: Vec<u64>,

    #[serde(default = "unset_str")]
    pub sort_key: String,

    #[serde(default = "unset_str")]
    pub filter: String,
}
