#![feature(dir_entry_ext2)]
//use std::io;
//use std::os;
//use std::fs::ReadDir;
//use std::path::PathBuf;
//use std::path::Path;
//
mod directories;

fn main() {
    let curdir = directories::current_dir();
    // cant unwrap curdir at the start, so I unwrap it with expect; This panics when an error has
    // been handed to it though.
    let dir_path_as_string: String = curdir.expect("REASON").as_os_str().to_string_lossy().into_owned();
    let dir_path: &str = dir_path_as_string.as_str();
    directories::print_everything(dir_path);
}
