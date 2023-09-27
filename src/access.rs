use crate::directory::Directory;
use std::io;

// Now comes the real meat of Janus, the file interaction.
// copy, move, rename, mkdir
// choosen by a full usr provided list, add x..z or x-z functionality later

// Main Function
pub fn access_dir(directory: Directory) {
    println!("The directory contains:");
    directory.print_contents_in_usr_format();
    print_keybinds();
    let usr_cmd_input = get_usr_cmd_input("Please enter a command:");
    let back_cmd = "b".to_string();
    let copy_cmd = "c".to_string();
    let move_cmd = "m".to_string();
    let rename_cmd = "r".to_string();
    let mkdir_cmd = "mkdir".to_string();


    if back_cmd == usr_cmd_input {
        return;
    } else if copy_cmd == usr_cmd_input {
        return;
        let usr_file_index_list = get_usr_cmd_input("Please enter the shown index of all files you want to impact.");
    } else if move_cmd == usr_cmd_input {
        return;
        let usr_file_index_list = get_usr_cmd_input("Please enter the shown index of all files you want to impact.");
    } else if rename_cmd == usr_cmd_input {
        return;
        let usr_file_index_list = get_usr_cmd_input("Please enter the shown index of all files you want to impact.");
    } else if mkdir_cmd == usr_cmd_input {
        return;
        let usr_file_index_list = get_usr_cmd_input("Please enter the shown index of all files you want to impact.");
    } else {
        // provided input Invalid!
        return;
    }
}

fn print_keybinds() {
    println!("Commands:");
    println!("[b]ack = b; [c]opy = c; [m]ove = m; [r]ename = r; [m]a[k]e [dir]ectory = mkdir;");
}

// General functions
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
