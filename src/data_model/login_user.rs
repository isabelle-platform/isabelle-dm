use crate::util::accessor::*;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct LoginUser {
    #[serde(default = "unset_str")]
    pub login: String,

    #[serde(default = "unset_str")]
    pub password: String,
}
