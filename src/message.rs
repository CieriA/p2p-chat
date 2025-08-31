use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub text: String
}
impl Message {
    #[inline]
    pub const fn new(id: String, text: String) -> Self {
        Self { id, text }
    }
}
