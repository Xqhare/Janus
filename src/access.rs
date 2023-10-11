use crate::directory::Directory;
use crate::mkdir;
use crate::copy;
use crate::print;
use crate::rename;
use home::home_dir;
use std::io;
use core::num::ParseIntError;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

// Now comes the real meat of Janus, the file interaction.
// copy, move, rename, mkdir
// choosen by a full usr provided list

// Main Function
pub fn access_dir(directory: Directory) {
    println!("-----------------------");
    println!("The directory contains:");
    directory.print_contents_in_usr_format();
    print::keybinds_cd_menu();
    let usr_cmd_input = get_usr_cmd_input("Please enter a command:");
    // go back
    if "b".to_string() == usr_cmd_input || "q".to_string() == usr_cmd_input {
        return;
    // copy files
    } else if "c".to_string() == usr_cmd_input {
        // creating the index list
        let index_list: Vec<usize> = match get_index_with_usr_input() {
            Ok(index_list) => {index_list},
            Err(any_err) => {
                println!("Error {any_err} encountered. Aborting step.");
                return;
            },
        };
        // creating the path
        let copy_dir_decoded: PathBuf = match get_dir_path_usr_input() {
            Ok(ok_path) => {ok_path},
            Err(any_err) => {
                    println!("Error {any_err} encountered. Aborting step.");
                    return;
                }
        };
        // TODO: This is weird and needs a rework
        if path_existence_and_creator(copy_dir_decoded.clone()) {
            // Actual copying
            copy::copy_loop(directory, index_list, copy_dir_decoded);
            println!("Copying successful. Returning to main menu.");
            println!("-------------------------------------------");
        // I believe this to be superfluous, but I feel better having it
        } else {
            return;
        };
    // move files -> means DELETION of files.
    } else if "m".to_string() == usr_cmd_input {
        // creating the index list
        let index_list: Vec<usize> = match get_index_with_usr_input() {
            Ok(index_list) => {index_list},
            Err(any_err) => {
                println!("Error {any_err} encountered. Aborting step.");
                return;
            },
        };
        // creating the path
        let copy_dir_decoded: PathBuf = match get_dir_path_usr_input() {
            Ok(ok_path) => {ok_path},
            Err(any_err) => {
                    println!("Error {any_err} encountered. Aborting step.");
                    return;
                }
        };
        // TODO: This is weird and needs a rework
        if path_existence_and_creator(copy_dir_decoded.clone()) {
            // Actual moving
            copy::move_loop(directory, index_list, copy_dir_decoded);
            println!("Moving successful. Returning to main menu.");
            println!("-------------------------------------------");
        } else {
            return;
        };
    // rename files
    } else if "r".to_string() == usr_cmd_input {
        let index_list: Vec<usize> = match get_index_with_usr_input() {
            Ok(index_list) => {index_list},
            Err(any_err) => {
                println!("Error {any_err} encountered. Aborting step.");
                return;
            },
        };
        let usr_scheme_input = get_remame_scheme_usr_input();
        // Actual renaming
        rename::rename_loop(directory, index_list, usr_scheme_input);
        println!("Renaming successful. Returning to main menu.");
        println!("-------------------------------------------");
        return;
    // make directory
    } else if "mkdir".to_string() == usr_cmd_input {
        make_dir_usr_input();
        return;
    // copy AND rename files
    } else if "cr".to_string() == usr_cmd_input || "C".to_string() == usr_cmd_input {
        let mut copy_success = false;
    // first I just copy
        // creating the index list
        let index_list: Vec<usize> = match get_index_with_usr_input() {
            Ok(index_list) => {index_list},
            Err(any_err) => {
                println!("Error {any_err} encountered. Aborting step.");
                return;
            },
        };
        // creating the path
        let copy_dir_decoded: PathBuf = match get_dir_path_usr_input() {
            Ok(ok_path) => {ok_path},
            Err(any_err) => {
                    println!("Error {any_err} encountered. Aborting step.");
                    return;
                }
        };
        // TODO: This is weird and needs a rework
        if path_existence_and_creator(copy_dir_decoded.clone()) {
            // Actual copying
            copy::copy_loop(directory, index_list, copy_dir_decoded.clone());
            copy_success = true;
        }
        // NOW I move to rename
        // first sanity check if copy was done.
        if copy_success {
            let new_dir = Directory::open_dir(copy_dir_decoded.as_os_str().to_str().unwrap()).unwrap();
            println!("-----------------------");
            println!("The directory contains:");
            new_dir.print_contents_in_usr_format();
            // creating the index list
            let index_list: Vec<usize> = match get_index_with_usr_input() {
                Ok(index_list) => {index_list},
                Err(any_err) => {
                    println!("Error {any_err} encountered. Aborting step.");
                    return;
                },
        };
            let usr_scheme_input = get_remame_scheme_usr_input();
            rename::rename_loop(new_dir, index_list, usr_scheme_input);
            println!("Copy and renaming successful. Returning to main menu.");
            println!("-------------------------------------------");
            return;
        }
        // Failsafe
        return;
    // move AND rename files
    } else if "mr".to_string() == usr_cmd_input || "M".to_string() == usr_cmd_input {
        let mut copy_success = false;
    // first I just copy
        // creating the index list
        let index_list: Vec<usize> = match get_index_with_usr_input() {
            Ok(index_list) => {index_list},
            Err(any_err) => {
                println!("Error {any_err} encountered. Aborting step.");
                return;
            },
        };
        // creating the path
        let copy_dir_decoded: PathBuf = match get_dir_path_usr_input() {
            Ok(ok_path) => {ok_path},
            Err(any_err) => {
                    println!("Error {any_err} encountered. Aborting step.");
                    return;
                }
        };
        // TODO: This is weird and needs a rework
        if path_existence_and_creator(copy_dir_decoded.clone()) {
            // Actual copying
            copy::move_loop(directory, index_list, copy_dir_decoded.clone());
            copy_success = true;
        }
        // NOW I move to rename
        // first sanity check if copy was done.
        if copy_success {
            let new_dir = Directory::open_dir(copy_dir_decoded.as_os_str().to_str().unwrap()).unwrap();
            println!("-----------------------");
            println!("The directory contains:");
            new_dir.print_contents_in_usr_format();
            // creating the index list
            let index_list: Vec<usize> = match get_index_with_usr_input() {
                Ok(index_list) => {index_list},
                Err(any_err) => {
                    println!("Error {any_err} encountered. Aborting step.");
                    return;
                },
        };
            let usr_scheme_input = get_remame_scheme_usr_input();
            rename::rename_loop(new_dir, index_list, usr_scheme_input);
            println!("Copy and renaming successful. Returning to main menu.");
            println!("-------------------------------------------");
            return;
        }
        // Failsafe
        return;
    // TESTING
    } else if "t".to_string() == usr_cmd_input {
        return_home_dir_path();
        let debug1 = directory.return_file_index();
        println!("debug {debug1:?}");
        for entry in directory.return_all_files() {
            entry.debug_print_all();
        }
    // provided input Invalid!
    } else {
        println!("Invalid command entered. Aborting.");
        return;
    }
}

fn yn_decoder(input: String) -> bool {
    return input == *"Yes" || input == *"yes" || input == *"Y" || input == *"y"
}

fn path_existence_and_creator(path: PathBuf) -> bool {
        let path_to_test = path.clone();
        // If path exists, continue, if not ask usr for consent to create it.
        if check_existance_dir(path_to_test) {
            // Path exists
            return true
        } else {
            // Path DOESNT exist
            let usr_answer = get_usr_cmd_input("Choosen path does not exist. Do you want to create it? y/n");
            let usr_answer_decoded = yn_decoder(usr_answer);
            if !usr_answer_decoded {
                return false
            } else {
                let _ = mkdir::create_dir(path.as_path());
                return true
            }
        }
}

fn check_string_into_path(input: String) -> std::io::Result<PathBuf> {
    if input.starts_with('~') {
        let stripped_input = input.trim_start_matches('~').to_string();
        let stripped_input_path = PathBuf::from(stripped_input);
        let usr_home_dir = return_home_dir_path();
        let output_path = usr_home_dir.join(stripped_input_path);
        return Ok(output_path)
    } else {
        let path_to_test = Path::new(&input);
        // if this check returns true; the Input can be used without any more modification.
        canon(path_to_test.to_path_buf())
    }
}

fn check_existance_dir(path: PathBuf) -> bool {
    return path.exists()
}

fn canon(path: PathBuf) -> std::io::Result<PathBuf> {
    let out = path.canonicalize()?;
    return Ok(out)
}

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
    let split_file_list: Vec<&str> = cleaned_file_index_list.split(',').collect();
    for index in split_file_list {
        if index.contains('-') {
            let start_end_vec = index.split('-').collect::<Vec<&str>>();
            // I don't know what to put in the else block; Maybe a panic or Error of some kind.
            // There is no need for that atm as the code works as expected
            let [start_str, end_str] = start_end_vec[..] else { todo!()};
            let start = check_str_into_pos_int(start_str)?;
            let end = check_str_into_pos_int(end_str)?;
            for n in start..=end {
                fn_output.push(n);
            }
        } else if index.contains("..") {
            let start_end_vec = index.split("..").collect::<Vec<&str>>();
            // I don't know what to put in the else block; Maybe a panic or Error of some kind.
            // There is no need for that atm as the code works as expected
            let [start_str, end_str] = start_end_vec[..] else { todo!()};
            let start = check_str_into_pos_int(start_str)?;
            let end = check_str_into_pos_int(end_str)?;
            for n in start..=end {
                fn_output.push(n);
            }
        } else if index.contains('/') {
            let start_end_vec = index.split('/').collect::<Vec<&str>>();
            // I don't know what to put in the else block; Maybe a panic or Error of some kind.
            // There is no need for that atm as the code works as expected
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
    return Ok(number)
}

pub fn usr_cd() -> Result<Directory, io::Error> {
    print::example_dir();
    print::example_home_shortcut();
    let usr_input = get_usr_cmd_input("Please enter a path.");
    let temp = check_string_into_path(usr_input).unwrap();
    let output = Directory::open_dir(temp.as_os_str().to_str().unwrap());
    return output
}

// WIP: there needs to be more usr input validation than trailing whitespace removal;
// The only problem is, I don't really know how.
// Remeber: all user input is hostile
pub fn get_usr_cmd_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_input_already_pushed_into_inputvar) => {
            let output = input.trim().to_owned();
            return output
        },
        Err(_input_clearly_invalid) => {
            "Invalid command".to_owned()
        },
    }
}

fn return_home_dir_path() -> PathBuf {
    // This home_dir is different from the env::home_dir one. The latter is depricated the former
    // is not. Why? Fuck me thats why!
    let usr_dir: PathBuf = home_dir().unwrap();
    return usr_dir
}

fn get_index_with_usr_input() -> Result<Vec<usize>, io::Error> {
    print::index_example();
    let usr_file_index_list = get_usr_cmd_input("Please enter the shown index of all files you want to impact.");
    match usr_file_input_decoder(usr_file_index_list) {
        Ok(index_list) => {return Ok(index_list);},
        Err(_any_err) => {
            return Err(ErrorKind::InvalidInput.into());
        },
    };
}

fn get_dir_path_usr_input() -> Result<PathBuf, io::Error> {
    print::example_dir();
    let copy_to_dir = get_usr_cmd_input("Please enter the path of the directory you want to paste into.");
    match check_string_into_path(copy_to_dir.clone()) {
        Ok(ok_path) => {return Ok(ok_path);},
        Err(_any_err) => {
            if path_existence_and_creator(PathBuf::from(copy_to_dir.clone())) {
                return Ok(PathBuf::from(copy_to_dir));
            } else {
                return Err(ErrorKind::InvalidInput.into());
            }
        },
    };
}

fn get_remame_scheme_usr_input() -> String {
    print::rename_schema_example();
    let usr_scheme_input = get_usr_cmd_input("Please enter your schema:");
    return usr_scheme_input;
}

fn make_dir_usr_input() {
    print::example_dir();
    let new_dir_path: String = get_usr_cmd_input("Please enter the path of the directory you want to create.");
    let parsed_path = Path::new(&new_dir_path);
    let _ignore_error = mkdir::create_dir(parsed_path);
}
