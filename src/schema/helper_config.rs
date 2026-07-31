use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x_min: f32,
    pub y_min: f32,
    pub x_max: f32,
    pub y_max: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedObject {
    pub class_id: u32,
    pub label: String,
    pub confidence: f32,
    pub bbox: BoundingBox,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceData {
    pub face_id: u64,
    pub bbox: BoundingBox,
    pub embedding: Vec<f32>,
    pub landmarks: Vec<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRText {
    pub text: String,
    pub confidence: f32,
    pub bbox: BoundingBox,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameHelperConfig {
    pub frame_index: u64,
    pub timestamp_sec: f64,
    pub prompt: String,
    pub objects: Vec<DetectedObject>,
    pub faces: Vec<FaceData>,
    pub ocr_texts: Vec<OCRText>,
    pub depth_map_ref: Option<String>,
    pub motion_vector_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAVAMetadata {
    pub video_name: String,
    pub original_resolution: (u32, u32),
    pub lowres_resolution: (u32, u32),
    pub fps: f64,
    pub total_frames: u64,
    pub encoder_version: String,
    pub generative_model_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAVAArchiveConfig {
    pub metadata: SAVAMetadata,
    pub frames: Vec<FrameHelperConfig>,
}

impl SAVAArchiveConfig {
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json_str: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json_str)
    }
}
