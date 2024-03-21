use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub name: String,
    pub id: u32,
}

impl User {
    pub fn new(name: String, id: u32) -> Self {
        Self { name, id }
    }
}
