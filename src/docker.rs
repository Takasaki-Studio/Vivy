use std::path::Path;
use std::process::{Command, Stdio};

pub enum ComposeOperations {
    Up,
    Down,
}

pub fn execute_docker_compose(path: &str, file: &Option<String>, op: ComposeOperations) {
    let (args, cmd) = match op {
        ComposeOperations::Up => (create_args(path, file, vec!["up", "-d", "--build"]), "up"),
        ComposeOperations::Down => (create_args(path, file, vec!["down"]), "down"),
    };

    let child = Command::new("docker")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn docker compose");

    child
        .wait_with_output()
        .expect("Failed to get docker compose output");

    println!("Sucess docker compose {}", cmd);
}

fn create_args(path: &str, file: &Option<String>, extra_args: Vec<&str>) -> Vec<String> {
    let mut args = vec![
        "compose".to_owned(),
        "--project-directory".to_owned(),
        path.to_owned(),
    ];

    if let Some(file) = file {
        let file_path = Path::new(path)
            .join(file)
            .to_str()
            .expect("Fail get docker compose file path")
            .to_owned();

        args.push("-f".into());
        args.push(file_path);
    }

    args.append(&mut extra_args.into_iter().map(|x| x.to_owned()).collect());

    args
}
