use crate::directory::Directory;
use std::io;

// Now comes the real meat of Janus, the file interaction.
// copy, move, rename, mkdir
// choosen by a full usr provided list, add x..z or x-z functionality later
pub fn access_dir(directory: Directory) {
    println!("The directory contains:");
    directory.print_contents_in_usr_format();
}

pub fn usr_cd() -> Result<Directory, io::Error> {
    let path_temp = Directory::current_dir().unwrap();
    let path = Directory::pathbuf_into_string(path_temp);
    println!("The current path is: {path}");
    let usr_input = get_usr_cmd_input("Please enter a path.");
    let output = Directory::open_dir(usr_input.as_str());
    return output;
}

pub fn get_usr_cmd_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_input_already_pushed_into_inputvar) => {
            let output = input.trim().to_owned();
            return output;
        },
        Err(_input_clearly_invalid) => {
            let invalid_input = "Invalid command".to_owned();
            return invalid_input;
        },
    }
}
