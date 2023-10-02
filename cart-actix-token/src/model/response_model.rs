use serde::{Deserialize, Serialize};

//Error response structure
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
    pub status: bool
}

//Success response structure
#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessResponse {
    pub message: String,
    pub status: bool
}