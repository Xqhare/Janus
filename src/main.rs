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

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let mut app: bool = false;
    print_welcome_msg();
    loop {
        // app kill switch
        if app {
            break;  
        }

        // print possible commands
        print_keybinds();
        
        // now main usr input loop
        // TODO: there needs to be more usr input validation than trailing whitespace removal;
        // Remeber: all user input is hostile
        let usr_cmd = get_usr_cmd_input("Please enter a command:");
        let _err_decode_string = "Invalid command".to_owned();
        let _cd = "cd".to_owned();
        let _quit = "q".to_owned();

        if usr_cmd == _err_decode_string {
            println!("Invalid command '{usr_cmd}' entered. Aborting.");
        } else if usr_cmd == _quit {
            app = true;
        } else if usr_cmd == _cd {
            let usr_cd_input = usr_cd();
            match usr_cd_input {
                Ok(directory) => {
                    //put this into its own function
                    println!("The directory contains:");
                    directory.print_contents_in_usr_format();
                },
                _ => {
                    println!("Invalid command entered. Aborting.")
                },
            }
        } else {
            println!("Invalid command entered. Aborting.");
        }

        
        
    }
}
//This doesn't work because we dont give controll over to the terminal control programm;
// TODO: implement a usr interface
//fn clear_screen() {
    // this works because termion is a dependency in Cargo.toml, and the full path is supplied.
    //crossterm::terminal::Clear;
//}

fn usr_cd() -> Result<Directory, io::Error> {
    let path_temp = Directory::current_dir().unwrap();
    let path = Directory::pathbuf_into_string(path_temp);
    println!("The current path is: {path}");
    let usr_input = get_usr_cmd_input("Please enter a path.");
    let output = Directory::open_dir(usr_input.as_str());
    return output;
}

fn print_welcome_msg() {
    println!("------------------------------------------------");
    println!("Welcome to Janus Version: {VERSION} by {AUTHORS}");
    println!("------------------------------------------------");
}

fn print_keybinds() {
    println!("Commands:");
    println!("[c]hange [d]irectory: cd; [q]uit: q");
}

fn get_usr_cmd_input(prompt: &str) -> String {
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

fn panic_quit() {
    panic!("Janus paniced! E001")
}
