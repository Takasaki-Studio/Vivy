use std::path::Path;
use std::process::{Child, Command, Stdio};

pub fn execute_docker_compose_down(path: &str, file: &Option<String>) {
    let args = create_args(path, file, vec!["down".to_string()]);

    let child: Child = Command::new("docker")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Fail docker compose down");

    child.wait_with_output().expect("Fail docker compose down");

    println!("Success docker compose down");
}

pub fn execute_docker_compose_up(path: &str, file: &Option<String>) {
    let args = create_args(
        path,
        file,
        vec!["up".to_string(), "-d".to_string(), "--build".to_string()],
    );

    let child: Child = Command::new("docker")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Fail docker compose up");

    child.wait_with_output().expect("Fail docker compose up");

    println!("Success docker compose up");
}

fn create_args<'a>(
    path: &'a str,
    file: &'a Option<String>,
    mut extra_args: Vec<String>,
) -> Vec<String> {
    let mut args = vec![
        "compose".to_string(),
        "--project-directory".to_string(),
        path.to_string(),
    ];

    if let Some(file) = file {
        let file_path = Path::new(path)
            .join(file)
            .to_str()
            .expect("Fail get docker compose file path")
            .to_owned();

        args.push("-f".to_string());
        args.push(file_path);
    }

    args.append(&mut extra_args);

    args
}
