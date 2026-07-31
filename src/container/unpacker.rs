use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

pub struct SAVAUnpacker;

impl SAVAUnpacker {
    /// Unpacks a `.sava` container file into a target directory.
    pub fn unpack<P: AsRef<Path>>(
        sava_path: P,
        output_dir: P,
    ) -> Result<HashMap<String, PathBuf>> {
        fs::create_dir_all(&output_dir)?;
        let file = File::open(&sava_path)
            .with_context(|| format!("Failed to open .sava file: {:?}", sava_path.as_ref()))?;

        let mut archive = ZipArchive::new(file)?;
        let mut extracted_files = HashMap::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = output_dir.as_ref().join(file.name());

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = File::create(&outpath)?;
                io::copy(&mut file, &mut outfile)?;
            }

            extracted_files.insert(file.name().to_string(), outpath);
        }

        println!(
            "[SAVA Rust Container] Unpacked {} components from {:?}",
            extracted_files.len(),
            sava_path.as_ref()
        );

        Ok(extracted_files)
    }
}
