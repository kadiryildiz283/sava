// ============================================================================
// Module: Config Loader (Hot-Reloadable)
// Single Responsibility: Loads & manages hot-reloadable central config.json.
//
// EXAMPLE JSON OUTPUT SCHEMA (Response when config loaded):
// {
//   "status": "SUCCESS",
//   "code": 200,
//   "message": "Configuration loaded successfully",
//   "data": {
//     "config_path": "./config.json",
//     "version": "0.1.0"
//   },
//   "error_details": null
// }
// ============================================================================

use anyhow::{Context, Result};
use arc_swap::ArcSwap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use crate::helper::json_response::JsonResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub app_name: String,
    pub version: String,
    pub environment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodecConfig {
    pub default_lowres_width: u32,
    pub default_lowres_height: u32,
    pub default_target_width: u32,
    pub default_target_height: u32,
    pub sample_rate: u32,
    pub crf: u32,
    pub preset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub temp_encode_dir: String,
    pub temp_decode_dir: String,
    pub db_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub log_dir: String,
    pub max_file_size_mb: u64,
    pub retention_days: u64,
    pub compress_zstd: bool,
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISidecarConfig {
    pub script_path: String,
    pub python_binary: String,
    pub timeout_sec: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub system: SystemConfig,
    pub codec: CodecConfig,
    pub storage: StorageConfig,
    pub logging: LoggingConfig,
    pub ai_sidecar: AISidecarConfig,
}

pub struct ConfigLoader {
    config_path: String,
    current: ArcSwap<AppConfig>,
}

impl ConfigLoader {
    pub fn new<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let path_str = config_path.as_ref().to_string_lossy().to_string();
        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {:?}", config_path.as_ref()))?;
        let parsed: AppConfig = serde_json::from_str(&content)?;

        Ok(Self {
            config_path: path_str,
            current: ArcSwap::from(Arc::new(parsed)),
        })
    }

    pub fn get(&self) -> Arc<AppConfig> {
        self.current.load().clone()
    }

    pub fn reload(&self) -> JsonResponse {
        match fs::read_to_string(&self.config_path) {
            Ok(content) => match serde_json::from_str::<AppConfig>(&content) {
                Ok(new_config) => {
                    self.current.store(Arc::new(new_config.clone()));
                    JsonResponse::success(
                        "Configuration hot-reloaded successfully",
                        Some(serde_json::json!({
                            "config_path": self.config_path,
                            "version": new_config.system.version
                        })),
                    )
                }
                Err(e) => JsonResponse::error(400, "Failed to parse updated config.json", Some(&e.to_string())),
            },
            Err(e) => JsonResponse::error(500, "Failed to read config file during reload", Some(&e.to_string())),
        }
    }
}
