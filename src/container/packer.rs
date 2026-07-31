// ============================================================================
// Module: SAVAPacker (.sava Binary Container Writer)
// Single Responsibility: Packs 10 modular binary tracks & metadata into a compressed .sava archive.
//
// EXAMPLE JSON OUTPUT SCHEMA (Status output after packing):
// {
//   "status": "SUCCESS",
//   "code": 200,
//   "message": "Packed 10 binary tracks into .sava container",
//   "data": {
//     "archive_path": "kadir.sava",
//     "archive_size_mb": 51.9
//   },
//   "error_details": null
// }
// ============================================================================

use anyhow::{Context, Result};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::CompressionMethod;
use zip::ZipWriter;
use zip::write::FileOptions;

pub struct SAVAPacker;

impl SAVAPacker {
    /// Packs 10 binary tracks and metadata.json into a unified `.sava` container archive.
    pub fn pack<P: AsRef<Path>>(
        output_sava_path: P,
        binary_track_files: Vec<(&str, &Path)>,
    ) -> Result<()> {
        let file = File::create(&output_sava_path).with_context(|| {
            format!(
                "Failed to create .sava container file: {:?}",
                output_sava_path.as_ref()
            )
        })?;

        let mut zip = ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o644);

        for (arc_name, file_path) in binary_track_files {
            if file_path.exists() {
                zip.start_file(arc_name, options)?;
                let mut track_file = File::open(file_path)?;
                let mut buffer = Vec::new();
                track_file.read_to_end(&mut buffer)?;
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
