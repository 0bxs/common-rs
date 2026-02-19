use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResOk<T> {
    pub code: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}
