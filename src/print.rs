use crate::directory::Directory;
use crate::{VERSION, AUTHORS};

pub fn welcome_msg() {
    println!("------------------------------------------------");
    println!("Welcome to Janus Version: {VERSION} by {AUTHORS}");
    println!("------------------------------------------------");
}

pub fn keybinds_main_menu() {
    println!("Commands:");
    println!("[q]uit = q");
    println!("[c]hange [d]irectory = cd");
    println!("-------------------------")
}

pub fn index_example() {
    println!("-----------------------");
    // 'a, b-f, g..m, m/w, w, x,y,z' -> returns abc
    println!("Index entry has to follow this format:");
    println!("1, 2-4, 5..7, 7/9, 9,10");
    println!("',' between the indicies; 2-4 & 5..7 = inclusive; 7/9 = exclusive; Spaces don't matter");
    println!("--------------------------------------------------------------------------------------")
}

pub fn keybinds_cd_menu() {
    // back, copy, move, rename, make directory, copy-rename, move-rename
    println!("Commands:");
    println!("[b]ack = b");
    println!("[c]opy = c");
    println!("[m]ove = m");
    println!("[r]ename = r");
    println!("[m]a[k]e [dir]ectory = mkdir");
    println!("[C/c]opy & [r]ename = cr / C");
    println!("[M/m]ove & [r]ename = mr / M");
    println!("----------------------------")
}

pub fn example_dir() {
    println!("-----------------------");
    let path_temp = Directory::current_dir().unwrap();
    let path = Directory::pathbuf_into_string(path_temp);
    println!("Your example path: {path}");
}

pub fn example_home_shortcut() {
    println!("Type: '~' to access your home directory. e.g. '~ExampleDirectory' ")
}

