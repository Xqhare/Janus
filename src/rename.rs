use std::{path::{Path, PathBuf}, fs, io::{ErrorKind, self}};
use Into;

use crate::{directory::Directory, access};

// we need a way to enter a renaming scheme, decode it, and implement it for every file.
// Scheme:
// custom name, index
//  - custom name and index is required for unique output names
// custom name needs to support entered whitespace. For linux support only / or NUL are non valid,
// for rest: NUL, \, /, :, *, ?, ", <, >, |. (for win no whitespace at start or end, and no period
// as last)

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
