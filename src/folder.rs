use std::fs::create_dir_all;
use std::path::Path;

pub fn check_folder(path: &Path) -> bool {
    let exists = path.exists();
    if !exists {
        create_dir_all(path).expect("Fail to create folder");
    }
    
    if !path.is_dir() {
        panic!("This path is not a folder");
    }
    
    let is_empty = path.read_dir().expect("Fail read folder").next().is_none();
    
    exists && !is_empty
}