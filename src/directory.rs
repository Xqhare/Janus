use std::alloc::Global;
use std::fs::DirEntry;
use std::{fs, io};
use std::path::PathBuf;
use std::env;

use crate::file::File;

#[derive(Default)]
pub struct Directory {
    files: Vec<File>,
    path: PathBuf,
}

impl Directory {
    // add code here
    pub fn open_dir(dir_path: &str) -> Result<Self, io::Error> {
        let dir_entries = get_dir_entries(&dir_path);
        //Error handling
        match dir_entries {
            Ok(dir_entires) => {
                let mut dir_files = Vec::new();
                let this_path = PathBuf::from(&dir_path);
                for entry in dir_entires {
                    //why do i need to reassign here; wtf
                    let test = entry;
                    let new_file = File::from(test);
                    dir_files.push(new_file);
                }
                Ok(Self { 
                    files: dir_files,
                    path: this_path,
                })
            }
            _ => { panic!("OPENING AS GONE WRONG! E110")
            }
        }
    }
}

fn get_dir_entries(read_dir_path: &str) -> Result<Vec<fs::DirEntry>, std::io::Error> {
    let mut dir_entries = vec![];
    for dir_entry in fs::read_dir(read_dir_path)? {
        let dir_entry = dir_entry?;
        dir_entries.push(dir_entry);
    }
    Ok(dir_entries)
}

pub fn current_dir() -> std::io::Result<PathBuf> {
    let path = env::current_dir()?;
    Ok(path)
}

pub fn print_everything(dir_path: &str) {
    let dir_entries = get_dir_entries(&dir_path);
        match dir_entries {
            // Error Handling
            Ok(dir_entries) => {
                for dir_entry in dir_entries {
                    let test = File::from(dir_entry);
                    let _test2 = File::print_all(&test);
                }
            }
            // Error Handling for real
            _ => {}
        }
}
