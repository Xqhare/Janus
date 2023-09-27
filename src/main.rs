#![feature(
    dir_entry_ext2,
    allocator_api,
    )]
#![allow(
    dead_code,
)]
use std::io;
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
    let usr_dir = Directory::open_dir(trimmed_input).expect("Error E020");
    usr_dir.print_contents_in_usr_format()
}
