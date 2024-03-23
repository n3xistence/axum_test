use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub id: Option<u32>,
    pub email: String,
}

impl User {
    pub fn new(name: String, id: Option<u32>, email: String) -> Self {
        Self { name, id, email }
    }
}
