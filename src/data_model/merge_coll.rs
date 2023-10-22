use crate::util::accessor::*;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct MergeColl {
    #[serde(default = "unset_bool")]
    pub merge: bool,

    #[serde(default = "unset_str")]
    pub collection: String,
}
