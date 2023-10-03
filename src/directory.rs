use std::{fs, io, env};
use std::path::PathBuf;

use crate::file::{File, self};

#[derive(Default)]
pub struct Directory {
    files: Vec<File>,
    file_index: usize,
    path: PathBuf,
}

impl Directory {
    pub fn open_dir(dir_path: &str) -> Result<Self, io::Error> {
        let dir_entries = Self::get_dir_entries(dir_path);
        //Error handling
        match dir_entries {
            Ok(dir_entires) => {
                let mut dir_files = Vec::new();
                let this_path = PathBuf::from(&dir_path);
                let mut index = 0;
                for entry in dir_entires {
                    //why do i need to reassign here; wtf
                    let test = entry;
                    let new_file = File::from(test);
                    dir_files.push(new_file);
                    index += 1;
                }
                return Ok(Self { 
                    files: dir_files,
                    file_index: index,
                    path: this_path,
                })
            }
            // TODO: learn how to propagate errors
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

    pub fn return_file_index(&self) -> usize {
        let output = &self.file_index;
        *output
    }

    pub fn pathbuf_into_string(path: PathBuf) -> String {
        let path_as_string: String = path.as_os_str().to_string_lossy().into_owned();
        path_as_string
    }

    pub fn print_contents_in_usr_format(&self) {
        //loop through all files; print most important info eg.:
        // - Name, is_dir OR is_file OR is_symlink, Extension
        let mut file_index = 0;
        for file in &self.files {
            println!("==================");
            println!("Index: {file_index}");
            file.print_name();
            file.print_extension();
            file.print_dir_file_symlink();
            println!("==================");
            file_index += 1;
        }
    }

    pub fn return_all_files(self) -> Vec<file::File> {
        let all_files = self.files;
        let mut output: Vec<file::File> = Vec::new();
        for entry in all_files {
            let test = entry;
            output.push(test);
        }
        output
    }

    pub fn return_dir_path(&self) -> PathBuf {
        let output = &self.path;
        return output.clone()
    }
}
