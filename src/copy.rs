use std::ffi::OsString;
use std::fs::DirEntry;
use std::path::{PathBuf, Path};

// WIP actual copying
// extract current name and extension, paste them on the new module_path!
// run fs::copy(old, new)?;
//
fn cur_name_and_extension(file_entry: DirEntry) -> OsString {
    let name = file_entry.path().file_name();
    let output = name.expect("Crash code C400").to_os_string();
    return output;
}


