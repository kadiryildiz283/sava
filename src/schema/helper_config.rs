// ============================================================================
// Module: SAVA Binary Schema & Metadata Specification
// Single Responsibility: Defines binary track layouts and metadata structures for .sava archives.
//
// EXAMPLE JSON METADATA OUTPUT SCHEMA (metadata.json inside .sava):
// {
//   "metadata": {
//     "video_name": "kadir.mp4",
//     "original_resolution": [3840, 2160],
//     "lowres_resolution": [256, 144],
//     "fps": 30.0,
//     "total_frames": 54000,
//     "encoder_version": "SAVA-v2.0-BinaryTrack",
//     "generative_model_target": "ControlNet-SVD-v2"
//   },
//   "face_gallery": {
//     "101": [-0.024, 0.081, ...]
//   },
//   "prompt_dictionary": {
//     "1": "4k high detail cinematic video of person, stop sign."
//   },
//   "binary_tracks": {
//     "motion": "motion.bin",
//     "depth": "depth.bin",
//     "object": "object.bin",
//     "face": "face.bin",
//     "ocr": "ocr.bin",
//     "latent": "latent.bin",
//     "helper": "helper.bin"
//   }
// }
// ============================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub struct BinaryTrackOffsets {
    pub motion: String,
    pub depth: String,
    pub object: String,
    pub face: String,
    pub ocr: String,
    pub latent: String,
    pub helper: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAVAArchiveMetadata {
    pub metadata: SAVAMetadata,
    pub face_gallery: HashMap<u64, Vec<f32>>,
    pub prompt_dictionary: HashMap<u32, String>,
    pub binary_tracks: BinaryTrackOffsets,
}

impl SAVAArchiveMetadata {
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json_str: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json_str)
    }
}
