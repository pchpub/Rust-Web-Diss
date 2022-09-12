use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub blacklistnames: Vec<String>,
}