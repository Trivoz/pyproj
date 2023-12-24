use std::fs::create_dir;
use std::path::PathBuf;

/* Check if the path exists, if it does, exit the program
 *
 * `path` - Check that this path does not exist
 *
 * # Example
 *
 * ```no_run,rust
 * # use std::path::PathBuf;
 * # use project::check_path_exists;
 * check_path_exists(PathBuf::from("/tmp/my_cool_project"));
 * ```
 */
fn check_path_exists(path: PathBuf) {
    if path.exists() {
        panic!("{} already exists", path.display());
    }
}

pub fn create_project(name: String, path_buffer: Option<PathBuf>) {
    let path = match path_buffer {
        Some(path) => path,
        None => PathBuf::from(name.as_str()),
    };
    check_path_exists(path.clone());
    match create_dir(path.clone()) {
        Ok(result) => {
            println!("Creating {} in {}: {:?}", name, path.display(), result);
        }
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
        panic!("{} is not a project", name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_path_exists() {
        let path = PathBuf::from("/tests/test_folder");
        if path.exists() {
            std::fs::remove_dir_all(path.clone()).unwrap();
        }
        check_path_exists(path.clone());
    }

    #[test]
    #[should_panic]
    fn test_check_path_exists_panic() {
        let path = PathBuf::from("/tests/test_folder");
        if !path.exists() {
            std::fs::create_dir(path.clone()).unwrap();
        }
        check_path_exists(path.clone());
    }

    #[test]
    fn test_create_project() {
        let path = PathBuf::from("test_folder");
        if path.exists() {
            std::fs::remove_dir_all(path.clone()).unwrap();
        }
        create_project("test_folder".to_string(), Some(path.clone()));
        assert!(path.exists());
        std::fs::remove_dir_all(path.clone()).unwrap();
    }

    #[test]
    fn test_create_and_remove_project() {
        let path = PathBuf::from("proj");
        if path.exists() {
            std::fs::remove_dir_all(path.clone()).unwrap();
        }
        create_project("proj".to_string(), Some(path.clone()));
        assert!(path.exists());
        remove_project("proj".to_string());
        assert!(!path.exists());
    }
}