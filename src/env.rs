use std::fs::{remove_file, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub fn create_env(path: &Path, env_file: Option<String>, envs: Vec<String>, update_file: bool) {
    let file_path = path.join(env_file.unwrap_or_else(|| ".env".to_string()));

    if file_path.exists() && update_file {
        update_env(file_path, envs);

        println!("Success to update .env");
        return;
    } else if file_path.exists() {
        remove_file(&file_path).expect("Fail to remove .env file");
    }

    let mut file = File::create(file_path).expect("Fail create .env file");
    file.write_all(envs.join("\n").as_bytes())
        .expect("Fail write env");

    println!("Success to create .env");
}

fn update_env(path: PathBuf, envs: Vec<String>) {
    let mut file = File::open(&path).expect("Fail to open .env file");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Fail to read content");

    let old_envs = content.split('\n');
    let mut new_env = Vec::new();

    for old_env in old_envs {
        let mut key_value = old_env
            .split('=')
            .map(String::from)
            .collect::<Vec<String>>();
        let new_value = get_env_value(&envs, &key_value[0]);

        if let Some(value) = new_value {
            key_value[1] = value;
        }

        new_env.push(key_value.join("="))
    }

    remove_file(&path).expect("Fail to remove .env file");

    let mut new_file = File::create(path).expect("Fail to create .env file");
    new_file
        .write_all(new_env.join("\n").as_bytes())
        .expect("Fail to update .env file");
}

fn get_env_value(envs: &Vec<String>, key: &str) -> Option<String> {
    for env in envs {
        let key_value = env.split('=').collect::<Vec<&str>>();

        if key_value[0] == key {
            return Some(key_value[1].to_string());
        }
    }

    None
}
