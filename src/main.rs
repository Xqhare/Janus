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
mod print;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let mut app: bool = false;
    loop {
        // app kill switch
        if app {
            break;  
        }
        print::welcome_msg();
        // print possible commands
        print::keybinds_main_menu();
        // now main usr input loop
        // WIP: there needs to be more usr input validation than trailing whitespace removal;
        // Remeber: all user input is hostile
        let usr_cmd = access::get_usr_cmd_input("Please enter a command:");
        let cd = "cd".to_owned();
        let quit = "q".to_owned();
        if usr_cmd == quit {
            app = true;
        } else if usr_cmd == cd {
            println!("-----------------------");
            let usr_cd_input = access::usr_cd();
            match usr_cd_input {
                Ok(directory) => {
                    access::access_dir(directory);
                    // Now comes the real meat of Janus, the file interaction.
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

fn panic_quit() {
    panic!("Janus paniced! E001")
}
