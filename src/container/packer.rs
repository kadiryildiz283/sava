use anyhow::{Context, Result};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

pub struct SAVAPacker;

impl SAVAPacker {
    /// Packs low-res base video, helper config JSON, and extra binary tracks into a compressed `.sava` container.
    pub fn pack<P: AsRef<Path>>(
        output_sava_path: P,
        lowres_video_path: P,
        helper_config_path: P,
        extra_files: Vec<(&str, &Path)>,
    ) -> Result<()> {
        let file = File::create(&output_sava_path)
            .with_context(|| format!("Failed to create .sava container file: {:?}", output_sava_path.as_ref()))?;

        let mut zip = ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o644);

        // 1. Write lowres_video.mp4
        if lowres_video_path.as_ref().exists() {
            zip.start_file("lowres_video.mp4", options)?;
            let mut v_file = File::open(&lowres_video_path)?;
            let mut buffer = Vec::new();
            v_file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }

        // 2. Write helper_config.json
        if helper_config_path.as_ref().exists() {
            zip.start_file("helper_config.json", options)?;
            let mut c_file = File::open(&helper_config_path)?;
            let mut buffer = Vec::new();
            c_file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }

        // 3. Write any extra binary tracks (.bin, .opus)
        for (arc_name, file_path) in extra_files {
            if file_path.exists() {
                zip.start_file(arc_name, options)?;
                let mut e_file = File::open(file_path)?;
                let mut buffer = Vec::new();
                e_file.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
            }
        }

        zip.finish()?;
        let metadata = std::fs::metadata(&output_sava_path)?;
        let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
        println!(
            "[SAVA Rust Container] Successfully packed .sava archive: {:?} ({:.2} MB)",
            output_sava_path.as_ref(),
            size_mb
        );

        Ok(())
    }
}
