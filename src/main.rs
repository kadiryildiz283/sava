// ============================================================================
// Module: SAVA CLI & Core Entry Point
// Single Responsibility: Command-line orchestration for 10-Track Binary SAVA Encoding & Decoding.
//
// EXAMPLE JSON OUTPUT SCHEMA (Returned on stdout):
// {
//   "status": "SUCCESS",
//   "code": 200,
//   "message": "SAVA Encoding completed successfully with 10 binary tracks",
//   "data": {
//     "input_video": "kadir.mp4",
//     "output_archive": "kadir.sava",
//     "archive_size_mb": 51.9
//   },
//   "error_details": null
// }
// ============================================================================

use anyhow::Result;
use clap::{Parser, Subcommand};
use sava::bridge::PythonAIClient;
use sava::container::{SAVAPacker, SAVAUnpacker};
use sava::helper::{ConfigLoader, JsonResponse, LoggerManager};
use sava::media::MediaDownscaler;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "sava")]
#[command(about = "Semantic AI Video Archive (SAVA) - Rust & Generative AI Video Codec", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encodes a 4K video into a compressed .sava semantic archive with 10 binary tracks
    Encode {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,

        #[arg(short = 'w', long)]
        width: Option<u32>,

        #[arg(short = 'H', long)]
        height: Option<u32>,
    },
    /// Decodes a .sava semantic archive into a restored 4K video
    Decode {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    // 1. Zero Hardcoded Config: Load central config.json
    let config_loader = ConfigLoader::new("config.json")?;
    let cfg = config_loader.get();

    // 2. Logging Manager initialization
    let logger = LoggerManager::new(&cfg.logging.log_dir, cfg.logging.max_file_size_mb);

    let ai_client = PythonAIClient::new(&cfg.ai_sidecar.script_path);

    match &cli.command {
        Commands::Encode {
            input,
            output,
            width,
            height,
        } => {
            logger.log_action("ENCODE_START", &format!("Input: {:?}, Output: {:?}", input, output));

            let target_width = width.unwrap_or(cfg.codec.default_lowres_width);
            let target_height = height.unwrap_or(cfg.codec.default_lowres_height);

            let temp_dir = PathBuf::from(&cfg.storage.temp_encode_dir);
            fs::create_dir_all(&temp_dir)?;

            let lowres_video_path = temp_dir.join("lowres_video.mp4");
            let audio_path = temp_dir.join("audio.opus");

            // 1. Downscale 72p skeleton video and extract audio.opus
            MediaDownscaler::downscale_video_and_audio(input, &lowres_video_path, &audio_path, target_width, target_height)?;

            // 2. Python AI Sidecar Extractor (Generates binary tracks in temp_dir)
            let resp = ai_client.request_encode(
                input.to_str().unwrap(),
                temp_dir.to_str().unwrap(),
                cfg.codec.sample_rate,
            )?;

            if resp.status != "SUCCESS" {
                logger.log_action("ENCODE_ERROR", &resp.message);
                let json_resp = JsonResponse::error(500, &resp.message, None);
                println!("{}", json_resp.to_json_string());
                return Ok(());
            }

            // 3. Pack 10 binary tracks into .sava container
            let motion_path = temp_dir.join("motion.bin");
            let depth_path = temp_dir.join("depth.bin");
            let object_path = temp_dir.join("object.bin");
            let face_path = temp_dir.join("face.bin");
            let ocr_path = temp_dir.join("ocr.bin");
            let latent_path = temp_dir.join("latent.bin");
            let helper_path = temp_dir.join("helper.bin");
            let metadata_path = temp_dir.join("metadata.json");

            let binary_tracks = vec![
                ("lowres_video.mp4", lowres_video_path.as_path()),
                ("audio.opus", audio_path.as_path()),
                ("motion.bin", motion_path.as_path()),
                ("depth.bin", depth_path.as_path()),
                ("object.bin", object_path.as_path()),
                ("face.bin", face_path.as_path()),
                ("ocr.bin", ocr_path.as_path()),
                ("latent.bin", latent_path.as_path()),
                ("helper.bin", helper_path.as_path()),
                ("metadata.json", metadata_path.as_path()),
            ];

            SAVAPacker::pack(output, binary_tracks)?;

            let archive_metadata = fs::metadata(output)?;
            let archive_size_mb = archive_metadata.len() as f64 / (1024.0 * 1024.0);

            logger.log_action("ENCODE_SUCCESS", &format!("10-Track Archive Size: {:.2} MB", archive_size_mb));

            let json_resp = JsonResponse::success(
                "SAVA Encoding completed successfully with 10 binary tracks",
                Some(serde_json::json!({
                    "input_video": input,
                    "output_archive": output,
                    "archive_size_mb": archive_size_mb
                })),
            );
            println!("{}", json_resp.to_json_string());
        }
        Commands::Decode { input, output } => {
            logger.log_action("DECODE_START", &format!("Input: {:?}, Output: {:?}", input, output));

            let temp_dir = PathBuf::from(&cfg.storage.temp_decode_dir);
            fs::create_dir_all(&temp_dir)?;

            // 1. Unpack 10 binary tracks from .sava container
            let extracted = SAVAUnpacker::unpack(input, &temp_dir)?;
            let lowres_path = extracted
                .get("lowres_video.mp4")
                .ok_missing("lowres_video.mp4")?;

            // 2. Python AI Restoration Engine (Reads binary tracks from temp_dir)
            let resp = ai_client.request_decode(
                lowres_path.to_str().unwrap(),
                temp_dir.to_str().unwrap(),
                output.to_str().unwrap(),
                (cfg.codec.default_target_width, cfg.codec.default_target_height),
            )?;

            if resp.status != "SUCCESS" {
                logger.log_action("DECODE_ERROR", &resp.message);
                let json_resp = JsonResponse::error(500, &resp.message, None);
                println!("{}", json_resp.to_json_string());
                return Ok(());
            }

            logger.log_action("DECODE_SUCCESS", &format!("Restored 4K Video from binary tracks: {:?}", output));

            let json_resp = JsonResponse::success(
                "SAVA Decoding completed successfully",
                Some(serde_json::json!({
                    "input_archive": input,
                    "output_restored_video": output
                })),
            );
            println!("{}", json_resp.to_json_string());
        }
    }

    Ok(())
}

trait MapMissing<T> {
    fn ok_missing(self, name: &str) -> Result<T>;
}

impl<T> MapMissing<T> for Option<T> {
    fn ok_missing(self, name: &str) -> Result<T> {
        self.ok_or_else(|| anyhow::anyhow!("Archive component missing: {}", name))
    }
}
