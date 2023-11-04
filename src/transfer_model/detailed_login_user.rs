use crate::util::accessor::*;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct DetailedLoginUser {
    #[serde(default = "unset_str")]
    pub username: String,

    #[serde(default = "unset_id")]
    pub id: u64,

    #[serde(default = "unset_str_vec")]
    pub role: Vec<String>,

    #[serde(default = "unset_str")]
    pub site_name: String,

    #[serde(default = "unset_str")]
    pub site_logo: String,

    #[serde(default = "unset_str")]
    pub licensed_to: String,
}
