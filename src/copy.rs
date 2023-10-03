use std::ffi::OsString;
use std::fs::{self};
use std::path::{PathBuf, Path};

use crate::file;
use crate::directory::{self, Directory};

// actual copying
// extract current name and extension, paste them on the new module_path!
// run fs::copy(old, new)?;

pub fn new_full_path(path: &PathBuf, name_with_extension: OsString) -> PathBuf {
    let mut output: PathBuf = path.to_path_buf();
    let temp_path = Path::new(&name_with_extension);
    output.push(temp_path);
    return output.to_path_buf();
}

fn copy_single_file(old: PathBuf, new: PathBuf) {
    if old != new {
        let _success = match fs::copy(old, new) {
            Ok(anything) => {anything},
            Err(any_err) => {panic!("Encountered Error {:?} during copy. Aborting C500", any_err)}
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
            copy_single_file(old_name, new_name)
        }
    counter += 1;
    }
}
