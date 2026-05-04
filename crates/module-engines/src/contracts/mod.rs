use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEnginesRequestDto {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEngineStatusRequestDto {
    pub engine_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunEngineVerificationRequestDto {
    pub engine_id: String,
}
