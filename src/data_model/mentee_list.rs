use yew::prelude::*;
use crate::data_model::mentee::Mentee;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct MenteeList {
    pub mentees: Vec<Mentee>,
}