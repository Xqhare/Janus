use std::ffi::OsString;
use std::fs;
use std::path::{PathBuf, Path};

use crate::file;
use crate::directory::{self, Directory};

pub fn new_full_path(path: &PathBuf, name_with_extension: OsString) -> PathBuf {
    let mut output: PathBuf = path.clone();
    let temp_path = Path::new(&name_with_extension);
    output.push(temp_path);
    output.clone()
}

fn copy_single_file(old: PathBuf, new: PathBuf) {
    if old != new {
        let _success = match fs::copy(old, new) {
            Ok(anything) => {anything},
            Err(any_err) => {panic!("Encountered Error {any_err:?} during copy. Aborting C500")}
        };
    } else {
        panic!("Old and new copy path are equal. Aborting C501")
    }
}

pub fn copy_loop(provided_directory: Directory, provided_index_list: Vec<usize>, provided_new_path: PathBuf) {
    let mut counter = 0;
    for entry in directory::Directory::return_all_files(provided_directory) {
        // Check if current file is to be impacted
        let mut file_to_be_impacted: bool = false;
        for index in &provided_index_list {
            if &counter == index {
                file_to_be_impacted = true;
            }
        }
        if file_to_be_impacted {
            // If current file is to be impacted, generate old and new names.
            let old_name = file::File::return_path(&entry);
            let name_with_extension = file::File::return_name_and_extension(&entry);
            let new_name = new_full_path(&provided_new_path, name_with_extension);
            copy_single_file(old_name, new_name);
        }
    counter += 1;
    }
}

pub fn move_loop(provided_directory: Directory, provided_index_list: Vec<usize>, provided_new_path: PathBuf) {
    let mut counter = 0;
    for entry in directory::Directory::return_all_files(provided_directory) {
        // Check if current file is to be impacted
        let mut file_to_be_impacted: bool = false;
        for index in &provided_index_list {
            if &counter == index {
                file_to_be_impacted = true;
            }
        }
        if file_to_be_impacted {
            // If current file is to be impacted, generate old and new names.
            let old_name = file::File::return_path(&entry);
            let name_with_extension = file::File::return_name_and_extension(&entry);
            let new_name = new_full_path(&provided_new_path, name_with_extension);
            copy_single_file(old_name.clone(), new_name);
            // check if dir or file,
            // execute remove_file or remove_dir
            if file::File::is_dir(&entry) {
                // its a dir to be removed
                fs::remove_dir(old_name.as_path()).unwrap();
            } else if file::File::is_file(&entry) {
                // its a file to be remove
                let () = fs::remove_file(old_name.as_path()).expect("Somethhing went terribly wrong. C511");
            } else {
                // Fallback and Feelgood Option; just try to remove with file_remove
                let () = fs::remove_file(old_name.as_path()).expect("Somethhing went terribly wrong. C512");
            }
        }
    counter += 1;
    }
}
