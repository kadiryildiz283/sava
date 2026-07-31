// ============================================================================
// Module: SAVA CLI & Core Entry Point
// Single Responsibility: Command-line orchestration for SAVA Encoding & Decoding.
//
// EXAMPLE JSON OUTPUT SCHEMA (Returned on stdout):
// {
//   "status": "SUCCESS",
//   "code": 200,
//   "message": "SAVA Encoding completed successfully",
//   "data": {
//     "input_video": "kadir.mp4",
//     "output_archive": "kadir.sava",
//     "archive_size_mb": 0.15
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
    /// Encodes a 4K video into a compressed .sava semantic archive
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
            let helper_config_path = temp_dir.join("helper_config.json");

            // 1. Downscale low-res skeleton video
            MediaDownscaler::downscale_video(input, &lowres_video_path, target_width, target_height)?;

            // 2. Python AI Sidecar Extractor
            let resp = ai_client.request_encode(
                input.to_str().unwrap(),
                helper_config_path.to_str().unwrap(),
                cfg.codec.sample_rate,
            )?;

            if resp.status != "SUCCESS" {
                logger.log_action("ENCODE_ERROR", &resp.message);
                let json_resp = JsonResponse::error(500, &resp.message, None);
                println!("{}", json_resp.to_json_string());
                return Ok(());
            }

            // 3. Pack into .sava container
            SAVAPacker::pack(output, &lowres_video_path, &helper_config_path, vec![])?;

            let archive_metadata = fs::metadata(output)?;
            let archive_size_mb = archive_metadata.len() as f64 / (1024.0 * 1024.0);

            logger.log_action("ENCODE_SUCCESS", &format!("Archive Size: {:.2} MB", archive_size_mb));

            let json_resp = JsonResponse::success(
                "SAVA Encoding completed successfully",
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

            // 1. Unpack .sava container
            let extracted = SAVAUnpacker::unpack(input, &temp_dir)?;
            let lowres_path = extracted
                .get("lowres_video.mp4")
                .ok_missing("lowres_video.mp4")?;
            let config_path = extracted
                .get("helper_config.json")
                .ok_missing("helper_config.json")?;

            // 2. Python AI Restoration Engine
            let resp = ai_client.request_decode(
                lowres_path.to_str().unwrap(),
                config_path.to_str().unwrap(),
                output.to_str().unwrap(),
                (cfg.codec.default_target_width, cfg.codec.default_target_height),
            )?;

            if resp.status != "SUCCESS" {
                logger.log_action("DECODE_ERROR", &resp.message);
                let json_resp = JsonResponse::error(500, &resp.message, None);
                println!("{}", json_resp.to_json_string());
                return Ok(());
            }

            logger.log_action("DECODE_SUCCESS", &format!("Restored 4K Video: {:?}", output));

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
