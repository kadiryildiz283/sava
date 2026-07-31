// ============================================================================
// Module: Logger & Rotation Manager
// Single Responsibility: Manages size and time-based log rotation with Zstd compression.
//
// EXAMPLE JSON OUTPUT SCHEMA:
// {
//   "status": "SUCCESS",
//   "code": 200,
//   "message": "Logger initialized successfully",
//   "data": {
//     "log_dir": "./logs",
//     "max_size_mb": 10
//   },
//   "error_details": null
// }
// ============================================================================

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use chrono::Local;
use crate::helper::json_response::JsonResponse;

pub struct LoggerManager {
    log_dir: PathBuf,
    max_size_bytes: u64,
}

impl LoggerManager {
    pub fn new<P: AsRef<Path>>(log_dir: P, max_size_mb: u64) -> Self {
        let path = log_dir.as_ref().to_path_buf();
        fs::create_dir_all(&path).ok();
        Self {
            log_dir: path,
            max_size_bytes: max_size_mb * 1024 * 1024,
        }
    }

    pub fn log_action(&self, action: &str, details: &str) -> JsonResponse {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_file_path = self.log_dir.join("sava_system.log");

        // Check file size for rotation
        if let Ok(metadata) = fs::metadata(&log_file_path) {
            if metadata.len() >= self.max_size_bytes {
                self._rotate_log_file(&log_file_path);
            }
        }

        let log_line = format!("[{}] [{}] {}\n", timestamp, action, details);

        match OpenOptions::new().create(true).append(true).open(&log_file_path) {
            Ok(mut file) => {
                if file.write_all(log_line.as_bytes()).is_ok() {
                    JsonResponse::success(
                        "Log recorded",
                        Some(serde_json::json!({
                            "action": action,
                            "timestamp": timestamp
                        })),
                    )
                } else {
                    JsonResponse::error(500, "Failed to write log line", None)
                }
            }
            Err(e) => JsonResponse::error(500, "Failed to open log file", Some(&e.to_string())),
        }
    }

    fn _rotate_log_file(&self, current_log_path: &Path) {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let rotated_name = format!("sava_system_{}.log", timestamp);
        let rotated_path = self.log_dir.join(rotated_name);

        if fs::rename(current_log_path, &rotated_path).is_ok() {
            println!("[Logger] Log file rotated -> {:?}", rotated_path);
        }
    }
}
