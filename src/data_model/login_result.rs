use crate::util::accessor::*;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct LoginResult {
    #[serde(default = "unset_bool")]
    pub succeeded: bool,

    #[serde(default = "unset_str")]
    pub error: String,
}
