use crate::util::accessor::*;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct DelParam {
    #[serde(default = "unset_bool")]
    pub del: bool,
}
