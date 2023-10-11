#![feature(
    dir_entry_ext2,
    allocator_api,
    )]
#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::explicit_counter_loop,
    clippy::redundant_else,
    clippy::blanket_clippy_restriction_lints,
    clippy::missing_docs_in_private_items,
    clippy::missing_safety_doc,
    clippy::panic_in_result_fn,
    clippy::panic,
    clippy::arithmetic_side_effects,
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
                },
                _ => {
                    println!("Invalid command entered. Reseting Janus.");
                },
            }
        } else {
            println!("Invalid command entered. Reseting Janus.");
        }
    }
}
