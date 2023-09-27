use std::ffi::OsString;
use std::fs::{Permissions, FileType, DirEntry};
use std::io;
use std::time::SystemTime;
use std::path::PathBuf;
use std::os::unix::fs::DirEntryExt2;

pub struct File {
    // General file info
    name: OsString,
    name_ref: OsString,
    path: PathBuf,
    file_extension: OsString,
    // Metadata
    permissions: Permissions,
    is_dir: bool,
    is_file: bool,
    is_symlink: bool,
    meta_filetype: io::Result<FileType>,
    byte_len: u64,
    time_accessed: io::Result<SystemTime>,
    time_created: io::Result<SystemTime>,
    time_modified: io::Result<SystemTime>,
}

impl From<DirEntry> for File {
    fn from(dir_entry: DirEntry) -> Self {
        Self { 
            name: dir_entry.file_name(),
            name_ref: dir_entry.file_name_ref().to_os_string(),
            path: dir_entry.path(),
            file_extension: {
                //God this is stupid; I love it!
                let test1 = OsString::from("None");
                let test2 = test1.as_os_str();
                dir_entry.path().extension().unwrap_or(test2).to_os_string()
            },
            permissions: dir_entry.metadata().unwrap().permissions(),
            is_dir: dir_entry.metadata().unwrap().is_dir(),
            is_file: dir_entry.metadata().unwrap().is_file(),
            is_symlink: dir_entry.metadata().unwrap().is_symlink(),
            meta_filetype: dir_entry.file_type(),
            byte_len: dir_entry.metadata().unwrap().len(),
            time_accessed: dir_entry.metadata().unwrap().accessed(),
            time_created: dir_entry.metadata().unwrap().created(),
            time_modified: dir_entry.metadata().unwrap().modified(), 
        }
    }
}

impl File {
    pub fn print_name(&self) {
        let name = &self.name;
        println!("Name: {}", name.to_str().unwrap());
    }

    pub fn print_extension(&self) {
        let extension = &self.file_extension;
        println!("Extension: {}", extension.to_str().unwrap());
    }

    pub fn print_dir_file_symlink(&self) {
        if self.is_dir {
            let file_type = "Directory".to_string();
            println!("Type: {}", file_type)
        } else if self.is_file {
            let file_type = "File".to_string();
            println!("Type: {}", file_type)
        } else {
            let file_type = "Systemlink".to_string();
            println!("Type: {}", file_type)
        }
    }

    pub fn debug_print_all(&self) {
        // FILENAME
        let dir_entry_file_name = &self.name;
        println!("FILE NAME: {:?}", dir_entry_file_name);
        let dir_entry_file_name_ref = &self.name_ref;
        println!("FILE NAME REF: {:?}", dir_entry_file_name_ref);
        // PATH
        let dir_entry_path = &self.path;
        println!("PATH: {:?}", dir_entry_path);
        // GET FILETYPE
        let dir_entry_filetype = &self.file_extension;
        println!("FILE TYPE: {:?}", dir_entry_filetype);
        // GET  PERMISSIONS
        let dir_entry_permissions = &self.permissions;
        println!("dir_entry_permissions: {:?}", dir_entry_permissions);
        // GET IS_DIR BOOLEAN
        let dir_entry_is_dir_bool = self.is_dir;
        println!("dir_entry_is_dir_bool: {:?}", dir_entry_is_dir_bool);
        // GET IS_SYMLINK BOOL
        let dir_entry_is_symlink_bool = self.is_symlink;
        println!("dir_entry_is_symlink_bool: {:?}", dir_entry_is_symlink_bool);
        // GET IS_FILE BOOL
        let dir_entry_is_file_bool = self.is_file;
        println!("dir_entry_is_file_bool: {:?}", dir_entry_is_file_bool);
        // GET METADATA FILETYPE
        let dir_entry_meta_filetype = &self.meta_filetype;
        println!("dir_entry_meta_filetype: {:?}", dir_entry_meta_filetype);
        // GET Byte LEN of file
        let dir_entry_byte_len = self.byte_len;
        println!("dir_entry_len: {:?}", dir_entry_byte_len);
        // GET ACCESSED TIME
        let dir_entry_accessed = &self.time_accessed;
        println!("dir_entry_accessed: {:?}", dir_entry_accessed);
        // GET CREATED TIME
        let dir_entry_created = &self.time_created;
        println!("dir_entry_created: {:?}", dir_entry_created);
        // GET MODIFIED TIME
        let dir_entry_modified = &self.time_modified;
        println!("dir_entry_modified: {:?}", dir_entry_modified);
    }
}
