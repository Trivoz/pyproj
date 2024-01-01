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

pub fn create_project(name: Option<String>, path_buffer: Option<PathBuf>) {
    // if the name doesn't exist but the path buffer does, it uses that

    let path = match path_buffer {
        Some(path) => path,
        None => {
            if name.is_some() {
                PathBuf::from(name.unwrap().as_str())
            } else {
                panic!("Either a name or path must be given");
            }
        }
    };

    check_path_exists(path.clone());

    match create_dir(path.clone()) {
        Ok(result) => {
            println!("Creating new project in {}", path.display());
        }
        Err(msg) => {
            println!("Failed to create new project in {}", path.display());
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
        create_project(Some("test_folder".to_string()), Some(path.clone()));
        assert!(path.exists());
        std::fs::remove_dir_all(path.clone()).unwrap();
    }

    #[test]
    fn test_create_and_remove_project() {
        let path = PathBuf::from("proj");
        if path.exists() {
            std::fs::remove_dir_all(path.clone()).unwrap();
        }
        create_project(Some("proj".to_string()), Some(path.clone()));
        assert!(path.exists());
        remove_project("proj".to_string());
        assert!(!path.exists());
    }
}
