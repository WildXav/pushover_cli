use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Config {
    pub app_token: String,
    pub user_token: String,
}