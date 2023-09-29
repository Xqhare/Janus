use crate::directory::Directory;
use std::io::{self};
use std::num::ParseIntError;

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

    // go back
    if back_cmd == usr_cmd_input {
        return;
    // copy files
    } else if copy_cmd == usr_cmd_input {
        let usr_file_index_list = get_usr_cmd_input("Please enter the shown index of all files you want to impact. Remember no whitespace");
        let test = usr_file_input_decoder(usr_file_index_list);
        println!("TEST: {:?}", test);
        print_example_dir();
        let copy_to_dir = get_usr_cmd_input("Please enter the path of the directory you want to paste into.");
        //copy_files(list, dir);
        return;
    // move files
    } else if move_cmd == usr_cmd_input {
        let usr_file_index_list = get_usr_cmd_input("Please enter the shown index of all files you want to impact. Remember no whitespace");
        return;
    // rename files
    } else if rename_cmd == usr_cmd_input {
        let usr_file_index_list = get_usr_cmd_input("Please enter the shown index of all files you want to impact. Remember no whitespace");
        return;
    // make directory in current directory
    } else if mkdir_cmd == usr_cmd_input {
        let usr_file_index_list = get_usr_cmd_input("Please enter the shown index of all files you want to impact. Remember no whitespace");
        return;
    // provided input Invalid!
    } else {
        println!("Invalid command entered. Aborting.");
        return;
    }
}

// Specialised functions

// This function takes in a usr provided String, containing numbers in the following format:
    // 'a, b-f, g..m, m/w, w, x,y,z' -> returns abc
// - 'a' = singles
// - 'b-f'= b to f -- inclusive
// - 'g..m' = g to m -- inclusive
// - 'm/w' = n to v -- exlusive
// supports input with or without whitespace

// THIS JUST WORKS!! I learned propagating Errors!
fn usr_file_input_decoder(file_index_list: String) -> Result<Vec<usize>, ParseIntError> {
    if file_index_list.len() <= 0 {
        println!("Empty input. E200");
        panic!("Crash code C200.")
    }
    let cleaned_file_index_list = remove_all_whitespace(file_index_list);
    let mut fn_output: Vec<usize> = Vec::new();
    let split_file_list: Vec<&str> = cleaned_file_index_list.split(",").collect();
    for index in split_file_list {
        if index.contains("-") {
            let start_end_vec = index.split("-").collect::<Vec<&str>>();
            
            let [start_str, end_str] = start_end_vec[..] else { todo!()};

            let start = check_str_into_pos_int(start_str)?;
            let end = check_str_into_pos_int(end_str)?;
            for n in start..=end {
                fn_output.push(n);
            }
        } else if index.contains("..") {
            let start_end_vec = index.split("..").collect::<Vec<&str>>();
            let [start_str, end_str] = start_end_vec[..] else { todo!()};
            let start = check_str_into_pos_int(start_str)?;
            let end = check_str_into_pos_int(end_str)?;
            for n in start..=end {
                fn_output.push(n);
            }
        } else if index.contains("/") {
            let start_end_vec = index.split("/").collect::<Vec<&str>>();
            let [start_str, end_str] = start_end_vec[..] else { todo!()};
            let start = check_str_into_pos_int(start_str)? - 1;
            let end = check_str_into_pos_int(end_str)?;
            for n in start..end {
                fn_output.push(n);
            }
        } else {
            let single_index = check_str_into_pos_int(index)?;
            fn_output.push(single_index);
        }
    }

    Ok(fn_output)

}

// General functions

fn remove_all_whitespace(string: String) -> String {
    let input_str = string.as_str();
    let mut output_string = String::new();
    for char in input_str.chars() {
        if !char.is_whitespace() {
            output_string.push(char);
        }
    }
    output_string
}

fn check_str_into_pos_int(to_check: &str) -> Result<usize, ParseIntError> {
    let number: usize = to_check.parse()?;
    return Ok(number);
}

fn print_keybinds() {
    println!("Commands:");
    println!("[b]ack = b; [c]opy = c; [m]ove = m; [r]ename = r; [m]a[k]e [dir]ectory = mkdir;");
}
fn print_example_dir() {
    let path_temp = Directory::current_dir().unwrap();
    let path = Directory::pathbuf_into_string(path_temp);
    println!("Your example path: {path}");
}

pub fn usr_cd() -> Result<Directory, io::Error> {
    print_example_dir();
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
