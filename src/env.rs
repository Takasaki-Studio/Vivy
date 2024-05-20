use std::fs::{File, remove_file};
use std::io::Write;
use std::path::Path;

pub fn create_env(path: &Path, envs: Vec<String>) {
    let file_path = path.join(".env");

    if file_path.exists() {
        remove_file(&file_path).expect("Fail to remove .env file");
    }
    
    let mut file = File::create(file_path).expect("Fail create .env file");

    for env in envs {
        file.write(format!("{}\n", env).as_bytes()).expect("Fail write env");
    }
    
    println!("Success to create .env");
}
