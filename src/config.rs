use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
}
