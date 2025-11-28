use std::fs::{File, remove_file};
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

    write_env(&file_path, envs);

    println!("Success to create .env");
}

fn update_env(path: PathBuf, envs: Vec<String>) {
    let mut file = File::open(&path).expect("Fail to open .env file");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Fail to read content");

    let old_envs = content.split('\n');
    let mut new_env = envs.clone();

    for old_env in old_envs {
        let key_value = old_env
            .split('=')
            .map(String::from)
            .collect::<Vec<String>>();
        let new_value = get_env_value(&envs, &key_value[0]);

        if new_value.is_some() {
            continue;
        }

        new_env.push(key_value.join("="))
    }

    remove_file(&path).expect("Fail to remove .env file");
    write_env(&path, new_env);
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

fn write_env(path: &PathBuf, env: Vec<String>) {
    let mut file = File::create(path).expect("Fail to create .env file");
    file.write_all(
        env.iter()
            .map(prepare_env)
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )
    .expect("Fail to update .env file");
}

fn prepare_env(env: &String) -> String {
    let parts = env.split('=').collect::<Vec<&str>>();
    let mut env_value = String::from(parts[1]);

    if !env_value.starts_with('"') {
        env_value.insert(0, '"');
    }

    if !env_value.ends_with('"') {
        env_value.push('"');
    }

    format!("{}={}", parts[0], env_value)
}
