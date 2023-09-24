use std::env;
use std::path::PathBuf;



fn current_dir() -> std::io::Result<PathBuf> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    Ok(path)
}
fn main() {
    println!("Hello, world!");
    let _ = current_dir();
}
