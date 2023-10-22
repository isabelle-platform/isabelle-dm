use crate::util::accessor::*;
use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct ListQuery {
    #[serde(default = "unset_id")]
    pub id: u64,

    #[serde(default = "unset_id")]
    pub id_min: u64,

    #[serde(default = "unset_id")]
    pub id_max: u64,
}
