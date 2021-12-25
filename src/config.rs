use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u32,
    pub directories: Vec<Directory>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Directory {
    pub prefix: String,
    pub path: String,
}
