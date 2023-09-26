#![feature(
    dir_entry_ext2,
    allocator_api,
    )]

use directory::Directory;

mod directory;
mod file;

fn main() {
    let dir_path = Directory::current_dir().unwrap();
    let dir_path_as_string: String = Directory::pathbuf_into_string(dir_path);
    let dir_path_as_str: &str = dir_path_as_string.as_str();
    let this_dir = Directory::open_dir(&dir_path_as_str).unwrap();
    Directory::print_dir(&this_dir);
}
