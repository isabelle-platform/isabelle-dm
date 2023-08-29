use crate::data_model::item::Item;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ItemList {
    pub items: Vec<Item>,
}
