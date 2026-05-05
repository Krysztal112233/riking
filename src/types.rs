use serde::{Deserialize, Serialize};

pub mod v1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub time: f64,

    pub result: T,
}
