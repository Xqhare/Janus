use std::{fs, io, env};
use std::path::PathBuf;

use crate::file::File;

#[derive(Default)]
pub struct Directory {
    files: Vec<File>,
    path: PathBuf,
}

impl Directory {
    pub fn open_dir(dir_path: &str) -> Result<Self, io::Error> {
        let dir_entries = Self::get_dir_entries(&dir_path);
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
    pub fn print_dir(&self) {
        for entry in &self.files {
            let file = &entry;
            let _print = File::print_all(&file);
        }
    }
    pub fn pathbuf_into_string(path: PathBuf) -> String {
        let path_as_string: String = path.as_os_str().to_string_lossy().into_owned();
        return path_as_string;
    }
    pub fn debug_print_dir(path: PathBuf) {
        let path_as_string = Self::pathbuf_into_string(path);
        let path_as_str = path_as_string.as_str();
        let this_dir = Self::open_dir(&path_as_str).unwrap();
        Self::print_dir(&this_dir);
    }
}
