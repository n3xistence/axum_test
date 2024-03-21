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

pub struct DBConfig {
    pub url: String,
    pub password: String,
    pub port: String,
    pub name: String,
}

impl DBConfig {
    pub fn new(url: String, password: String, port: String, name: String) -> Self {
        Self {
            url,
            password,
            port,
            name,
        }
    }
}
