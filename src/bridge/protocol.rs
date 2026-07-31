use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EncodeRequest {
    pub command: String, // "encode"
    pub input_video: String,
    pub output_config_path: String,
    pub sample_rate: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodeRequest {
    pub command: String, // "decode"
    pub lowres_video: String,
    pub helper_config_path: String,
    pub output_video: String,
    pub target_resolution: (u32, u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIServiceResponse {
    pub status: String,
    pub message: String,
    pub payload: Option<serde_json::Value>,
}
