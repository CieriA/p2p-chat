use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Message {
    Message { id: String, text: String },
}
