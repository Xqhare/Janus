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

fn old_main() {
    let cur_path = Directory::current_dir().unwrap();
    let cur_path_string = Directory::pathbuf_into_string(cur_path);
    let mut input = String::new();
    println!("Please enter a path. Current path: ");
    println!("{cur_path_string}");
    io::stdin().read_line(&mut input).expect("Failed to read line. E010");
    let trimmed_input = input.trim();
    // TODO: trimmed_input really should be checked if it is a valid path.
    let usr_dir = Directory::open_dir(trimmed_input).expect("Failed to open directory. Error E020");
    usr_dir.print_contents_in_usr_format()
}

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
            println!("Invalid command entered. Aborting.");
        } else if usr_cmd == _quit {
            app = true;
        } else if usr_cmd == _cd {
            unimplemented!();
        } else {
            println!("Invalid command entered. Aborting.");
        }

        
        
    }
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
    panic!("Janus exited successfully.")
}
