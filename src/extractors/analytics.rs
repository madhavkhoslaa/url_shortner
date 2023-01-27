use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Analytics {
    pub country: Option<String>,
    pub ip: Option<String>,
    pub device: Option<String>,
    pub browser: Option<String>,
}
