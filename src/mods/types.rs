use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub blacklistnames: Vec<String>,
}