use std::{path::{Path, PathBuf}, fs, ffi::OsString};
use Into;

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
            let new_name_with_extension = make_new_name_from_scheme(usr_scheme.clone(), counter, extension).expect("Renaming Error C600!");
            let new_path: PathBuf = copy::new_full_path(&dir_path, new_name_with_extension);
            rename_single_file(old_path, new_path).expect("Renaming Error C601!");
        }
        
        counter += 1;
    }
}

fn make_new_name_from_scheme(usr_scheme: String, counter: usize, extension: OsString) -> Result<OsString, std::io::Error> {
    // Scheme validation
    if usr_scheme.len() <= 0 {
        return Err(Into::into(std::io::ErrorKind::InvalidInput))
    }
    if !usr_scheme.contains(",") {
        return Err(Into::into(std::io::ErrorKind::InvalidInput))
    }
    if !usr_scheme.contains("index") {
        return Err(Into::into(std::io::ErrorKind::InvalidInput))
    }

    let split_usr_scheme = usr_scheme.split(",");
    let mut temp_name = String::new();
    for entry in split_usr_scheme {
        if entry.contains("'") {
            let test = entry.replace("'", "");
            temp_name.push_str(test.as_str());
        } else if entry.contains("index") {
            let index = counter.to_string();
            temp_name.push_str(index.as_str());
        } else if entry.contains("creation time") {
            // WIP
            // here be creation time
            return Err(Into::into(std::io::ErrorKind::InvalidInput));
        } else {
            return Err(Into::into(std::io::ErrorKind::InvalidInput));
        }
    }
    // extension is without leading .; so it is added back here.
    temp_name.push_str(".");
    temp_name.push_str(extension.as_os_str().to_str().unwrap());
    let output_name = OsString::from(temp_name);
    return Ok(output_name);
}



// we need a way to enter a renaming scheme, decode it, and implement it for every file.
// Scheme:
// custom name, index
//  - custom name and index is required for unique output names
// custom name needs to support entered whitespace. For linux support only / or NUL are non valid,
// for rest: NUL, \, /, :, *, ?, ", <, >, |. (for win no whitespace at start or end, and no period
// as last)
//
// More TODO: add creation_time to the scheme

fn make_rename_scheme(usr_scheme: String, file_indexes: Vec<usize>) -> Result<Vec<String>, std::io::Error> {
    if usr_scheme.len() <= 0 {
        return Err(Into::into(std::io::ErrorKind::InvalidData))
    }
    if !usr_scheme.contains(",") {
        return Err(Into::into(std::io::ErrorKind::InvalidInput))
    }
    if !usr_scheme.contains("index") {
        return Err(Into::into(std::io::ErrorKind::InvalidInput))
    }
    // let cleaned_usr_sheme = access::remove_all_whitespace(usr_scheme);
    let mut fn_output: Vec<String> = Vec::new();
    let split_usr_scheme: Vec<&str> = usr_scheme.split(",").collect();

    for file_index in file_indexes {
        for entry in &split_usr_scheme {
            let mut tmp_output: String = String::new();
            if entry.contains("index") {
                let index_string = file_index.to_string();
                tmp_output.push_str(index_string.as_str());
            } else {
                tmp_output.push_str(entry);
            }
            fn_output.push(tmp_output);
        }
    }
    Ok(fn_output)
}

// REMEMBER ONLY PASS IN FINISHED ABSOLTE PATHS
fn rename_single_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> std::io::Result<()> {
    fs::rename(from, to)?;
    Ok(())
}

/* fn rename_by_list(list: Vec<usize>, dir: Directory, new_name_list: Vec<PathBuf>) -> std::io::Result<()> {
    let mut counter = 0;
    let test = dir;
    for index in list {
        let file = test.file_at_index(index);
        let old_name = file.return_path().as_path();
        let new_name_temp = new_name_list.get(counter).expect("How did you do this? E300");
        let new_name = new_name_temp.as_path();
        // We assume that the provided new name list is made up of valid paths
        rename_single_file(old_name, new_name);
        counter += 1;
    }
    Ok(())
} */
