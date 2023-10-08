use std::{path::{Path, PathBuf}, fs, ffi::OsString};
use chrono::Utc;
use chrono::DateTime;

use crate::{directory::Directory, copy};


pub fn rename_loop(directory: Directory, file_index_list: Vec<usize>, usr_scheme: String) {
    let mut counter = 0;
    let dir_path: PathBuf = directory.return_dir_path();
    for entry in Directory::return_all_files(directory) {
        // Check if current file is to be impacted
        let mut file_to_be_impacted: bool = false;
        for index in &file_index_list {
            if &counter == index {
                file_to_be_impacted = true;
            }
        }
        if file_to_be_impacted {
            let old_path: PathBuf = entry.return_path();
            let extension = entry.return_extension();
            let creation_time:  DateTime<Utc> = entry.return_creation_time_in_std_format();
            let new_name_with_extension = make_new_name_from_scheme(usr_scheme.clone(), counter, extension, creation_time).expect("Renaming Error C600!");
            let new_path: PathBuf = copy::new_full_path(&dir_path, new_name_with_extension);
            rename_single_file(old_path, new_path).expect("Renaming Error C601!");
            
        }
        counter += 1;
    }
}

// This is the function to enter a renaming scheme and decode it
// Scheme:
// custom name, index
//  - custom name and index is required for unique output names
// custom name needs to support entered whitespace. For linux support only / or NUL are non valid,
// for rest: NUL, \, /, :, *, ?, ", <, >, |. (for win no whitespace at start or end, and no period
// as last)
//
// More TODO: add creation_time to the scheme
fn make_new_name_from_scheme(usr_scheme: String, counter: usize, extension: OsString, creation_time: DateTime<Utc>) -> Result<OsString, std::io::Error> {
    // Scheme validation
    if usr_scheme.len() <= 0 {
        return Err(Into::into(std::io::ErrorKind::InvalidInput))
    }
    if !usr_scheme.contains("index") {
        return Err(Into::into(std::io::ErrorKind::InvalidInput))
    }
    // Actual decoding
    let split_usr_scheme = usr_scheme.split(',');
    let mut temp_name = String::new();
    for entry in split_usr_scheme {
        if entry.contains('\'') {
            let test = entry.replace('\'', "");
            temp_name.push_str(test.as_str());
        } else if entry.contains("index") {
            let index = counter.to_string();
            temp_name.push_str(index.as_str());
        } else if entry.contains("creation time") {
            let time = creation_time.format("%F-%T").to_string();
            let test = time.as_str();
            temp_name.push_str(test);
        } else {
            return Err(Into::into(std::io::ErrorKind::InvalidInput));
        }
    }
    // extension is without leading .; so it is added back here.
    temp_name.push('.');
    temp_name.push_str(extension.as_os_str().to_str().unwrap());
    let output_name = OsString::from(temp_name);
    return Ok(output_name)
}

// REMEMBER ONLY PASS IN FINISHED ABSOLTE PATHS
fn rename_single_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> std::io::Result<()> {
    fs::rename(from, to)?;
    Ok(())
}
