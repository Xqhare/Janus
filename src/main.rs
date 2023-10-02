#![feature(
    dir_entry_ext2,
    allocator_api,
    )]
#![allow(
    dead_code,
)]

mod directory;
mod file;
mod access;
mod mkdir;
mod rename;
mod copy;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let mut app: bool = false;
    loop {
        // app kill switch
        if app {
            break;  
        }

        print_welcome_msg();
        // print possible commands
        print_keybinds();
        
        // now main usr input loop
        // TODO: there needs to be more usr input validation than trailing whitespace removal;
        // Remeber: all user input is hostile
        let usr_cmd = access::get_usr_cmd_input("Please enter a command:");
        let _err_decode_string = "Invalid command".to_owned();
        let _cd = "cd".to_owned();
        let _quit = "q".to_owned();

        if usr_cmd == _err_decode_string {
            println!("Invalid command '{usr_cmd}' entered. Aborting.");
        } else if usr_cmd == _quit {
            app = true;
        } else if usr_cmd == _cd {
            println!("-----------------------");
            let usr_cd_input = access::usr_cd();
            match usr_cd_input {
                Ok(directory) => {
                    access::access_dir(directory);
                    // Now comes the real meat of Janus, the file interaction.
                    // copy, move, rename, mkdir
                    // choosen by a full usr provided list, add x..z or x-z functionality later
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

fn print_welcome_msg() {
    println!("------------------------------------------------");
    println!("Welcome to Janus Version: {VERSION} by {AUTHORS}");
    println!("------------------------------------------------");
}

fn print_keybinds() {
    println!("Commands:");
    println!("[q]uit = q");
    println!("[c]hange [d]irectory = cd");
    println!("-------------------------")
}



fn panic_quit() {
    panic!("Janus paniced! E001")
}

//This doesn't work because we dont give controll over to the terminal control programm;
// TODO: implement a usr interface
//fn clear_screen() {
    // this works because termion is a dependency in Cargo.toml, and the full path is supplied.
    //crossterm::terminal::Clear;
//}
