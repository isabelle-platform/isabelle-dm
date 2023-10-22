use crate::util::accessor::*;
use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct MergeColl {
    #[serde(default = "unset_bool")]
    pub merge: bool,

    #[serde(default = "unset_str")]
    pub collection: String,
}
