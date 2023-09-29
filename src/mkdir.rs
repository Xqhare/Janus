use std::{path::Path, fs};

pub fn create_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}
