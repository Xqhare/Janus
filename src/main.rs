#![feature(
    dir_entry_ext2,
    allocator_api,
    )]
use std::{io, path::PathBuf};
use directory::Directory;

mod directory;
mod file;

fn main() {
    let cur_path = Directory::current_dir().unwrap();
    let cur_path_string = Directory::pathbuf_into_string(cur_path);
    let mut input = String::new();
    println!("Please enter a path. Current path: ");
    println!("{cur_path_string}");
    io::stdin().read_line(&mut input).expect("Failed to read line. E010");
    let trimmed_input = input.trim();
    // TODO: trimmed_input really should be checked if it is a valid path.
    let usr_path = PathBuf::from(trimmed_input);
    debug_print_dir(usr_path);
}

fn debug_print_all() {
    let dir_path = Directory::current_dir().unwrap();
    let dir_path_as_string: String = Directory::pathbuf_into_string(dir_path);
    let dir_path_as_str: &str = dir_path_as_string.as_str();
    let this_dir = Directory::open_dir(&dir_path_as_str).unwrap();
    Directory::print_dir(&this_dir);
}
fn debug_print_dir(path: PathBuf) {
    let path_as_string = Directory::pathbuf_into_string(path);
    let path_as_str = path_as_string.as_str();
    let this_dir = Directory::open_dir(&path_as_str).unwrap();
    Directory::print_dir(&this_dir);
}
