use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct DelParam {
    pub id: u64,
}
