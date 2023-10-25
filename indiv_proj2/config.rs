// config.rs

use std::env;

pub struct DbConfig {
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: String,
}

pub fn get_db_config() -> DbConfig {
    DbConfig {
        db_name: env::var("DB_NAME").unwrap(),
        db_user: env::var("DB_USER").unwrap(),
        db_password: env::var("DB_PASSWORD").unwrap(),
        db_host: env::var("DB_HOST").unwrap(),
        db_port: env::var("DB_PORT").unwrap(),
    }
}
