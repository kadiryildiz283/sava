use crate::bridge::protocol::{AIServiceResponse, DecodeRequest, EncodeRequest};
use anyhow::{Result, anyhow};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub struct PythonAIClient {
    sidecar_script_path: String,
}

impl PythonAIClient {
    pub fn new<P: AsRef<Path>>(sidecar_script_path: P) -> Self {
        Self {
            sidecar_script_path: sidecar_script_path.as_ref().to_string_lossy().to_string(),
        }
    }

    /// Invokes the Python AI Sidecar for ENCODING (Extraction of YOLO, InsightFace, EasyOCR, Depth).
    pub fn request_encode(
        &self,
        input_video: &str,
        output_config_path: &str,
        sample_rate: u32,
    ) -> Result<AIServiceResponse> {
        let req = EncodeRequest {
            command: "encode".to_string(),
            input_video: input_video.to_string(),
            output_config_path: output_config_path.to_string(),
            sample_rate,
        };

        let json_payload = serde_json::to_string(&req)?;
        self._send_request(&json_payload)
    }

    /// Invokes the Python AI Sidecar for DECODING (Generative Video Restoration via ControlNet).
    pub fn request_decode(
        &self,
        lowres_video: &str,
        helper_config_path: &str,
        output_video: &str,
        target_resolution: (u32, u32),
    ) -> Result<AIServiceResponse> {
        let req = DecodeRequest {
            command: "decode".to_string(),
            lowres_video: lowres_video.to_string(),
            helper_config_path: helper_config_path.to_string(),
            output_video: output_video.to_string(),
            target_resolution,
        };

        let json_payload = serde_json::to_string(&req)?;
        self._send_request(&json_payload)
    }

    fn _send_request(&self, json_payload: &str) -> Result<AIServiceResponse> {
        println!("[SAVA Rust IPC Bridge] Dispatching command to Python AI sidecar...");

        let mut child = Command::new("python3")
            .arg(&self.sidecar_script_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(json_payload.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        let raw_stdout = String::from_utf8_lossy(&output.stdout);

        if !output.status.success() {
            let raw_stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Python AI Sidecar Error: {}", raw_stderr));
        }

        let resp: AIServiceResponse = serde_json::from_str(raw_stdout.trim()).map_err(|e| {
            anyhow!(
                "Failed to parse Python sidecar JSON response: {} (raw: {})",
                e,
                raw_stdout
            )
        })?;

        Ok(resp)
    }
}
