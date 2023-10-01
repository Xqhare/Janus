use std::{path::Path, fs};

// I think I'm starting to understand!
// the <> after the function name is te type of the input! AsRef uses any defined type I guess?
// Also wrapping stuff again! - wrapping is fun, call me a wrapeist!
// REMEMBER ONLY PASS IN FINISHED ABSOLTE PATHS
pub fn create_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}
