use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct IdParam {
    pub id: u64,
}
