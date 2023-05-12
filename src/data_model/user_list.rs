use crate::data_model::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct UserList {
    pub users: Vec<User>,
}
