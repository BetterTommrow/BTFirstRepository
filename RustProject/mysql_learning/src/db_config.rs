use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub 毛古哥服务器: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String
}
