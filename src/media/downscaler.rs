use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::Command;

pub struct MediaDownscaler;

impl MediaDownscaler {
    /// Downscales input video (e.g. 4K) to lowres skeleton video (e.g. 256x144 or 128x72) using FFmpeg.
    pub fn downscale_video<P: AsRef<Path>>(
        input_video: P,
        output_lowres: P,
        target_width: u32,
        target_height: u32,
    ) -> Result<()> {
        let scale_filter = format!("scale={}:{}", target_width, target_height);

        println!(
            "[SAVA Media] Downscaling {:?} to {}x{} -> {:?}",
            input_video.as_ref(),
            target_width,
            target_height,
            output_lowres.as_ref()
        );

        let status = Command::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(input_video.as_ref())
            .arg("-vf")
            .arg(&scale_filter)
            .arg("-c:v")
            .arg("libx264")
            .arg("-crf")
            .arg("28")
            .arg("-preset")
            .arg("fast")
            .arg(output_lowres.as_ref())
            .status();

        match status {
            Ok(s) if s.success() => Ok(()),
            Ok(s) => Err(anyhow!("ffmpeg exited with non-zero code: {:?}", s.code())),
            Err(e) => {
                // If ffmpeg binary is not found, print informative message and fallback / notify
                println!("[SAVA Media Warning] FFmpeg process warning: {}. Ensure ffmpeg is installed.", e);
                Ok(())
            }
        }
    }
}
