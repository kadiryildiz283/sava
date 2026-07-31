// ============================================================================
// Module: MediaDownscaler (FFmpeg 72p Video & Audio Extractor)
// Single Responsibility: Downscales 4K video to lowres skeleton video and extracts audio.opus.
//
// EXAMPLE JSON OUTPUT SCHEMA:
// {
//   "status": "SUCCESS",
//   "code": 200,
//   "message": "Extracted 72p video and audio.opus",
//   "data": {
//     "lowres_video": "./sava_temp_encode/lowres_video.mp4",
//     "audio": "./sava_temp_encode/audio.opus"
//   },
//   "error_details": null
// }
// ============================================================================

use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct MediaDownscaler;

impl MediaDownscaler {
    /// Downscales input video to lowres skeleton video (e.g. 256x144) and extracts OPUS audio.
    pub fn downscale_video_and_audio<P: AsRef<Path>>(
        input_video: P,
        output_lowres: P,
        output_audio: P,
        target_width: u32,
        target_height: u32,
    ) -> Result<()> {
        let scale_filter = format!("scale={}:{}", target_width, target_height);

        println!(
            "[SAVA Media] Extracting 72p video ({:?}) and OPUS audio ({:?})...",
            output_lowres.as_ref(),
            output_audio.as_ref()
        );

        // 1. Extract LowRes Skeleton Video
        let status_video = Command::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(input_video.as_ref())
            .arg("-vf")
            .arg(&scale_filter)
            .arg("-an")
            .arg("-c:v")
            .arg("libx264")
            .arg("-crf")
            .arg("28")
            .arg("-preset")
            .arg("fast")
            .arg(output_lowres.as_ref())
            .status();

        if let Err(e) = status_video {
            println!("[SAVA Media Warning] FFmpeg video downscale error: {}", e);
        }

        // 2. Extract Audio Stream to OPUS Codec (32 kbps)
        let _status_audio = Command::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(input_video.as_ref())
            .arg("-vn")
            .arg("-c:a")
            .arg("libopus")
            .arg("-b:a")
            .arg("32k")
            .arg(output_audio.as_ref())
            .status();

        Ok(())
    }
}
