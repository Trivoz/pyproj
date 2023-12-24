use std::fs::create_dir;
use std::path::PathBuf;
use std::process::exit;

fn check_path_exists(path: PathBuf) {
    if path.exists() {
        println!("The specified path already exists.");
        exit(1);
    }
}

pub fn create_project(name: String, path_buffer: Option<PathBuf>) {
    let path = match path_buffer {
        Some(path) => path,
        None => PathBuf::from(name.as_str()),
    };
    check_path_exists(path.clone());
    match create_dir(path.clone()) {
        Ok(_) => println!("Creating {} in {}", name, path.display()),
        Err(msg) => {
            println!("Failed to create {} in {}", name, path.display());
            panic!("{}", msg);
        }
    }
}

pub fn remove_project(name: String) {
    let path = PathBuf::from(name.as_str());
    if path.clone().exists() {
        match std::fs::remove_dir_all(path.clone()) {
            Ok(_) => println!("Removing {} in {}", name, path.display()),
            Err(msg) => {
                println!("Failed to remove {} in {}", name, path.display());
                panic!("{}", msg);
            }
        }
    } else {
        println!("{} is not a project", name);
        exit(1);
    }
}
