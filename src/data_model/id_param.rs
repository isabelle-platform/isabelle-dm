use crate::util::accessor::unset_id;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct IdParam {
    #[serde(default = "unset_id")]
    pub id: u64,
}
