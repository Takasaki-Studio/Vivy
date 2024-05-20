use std::process::{Child, Command, Stdio};

pub fn execute_docker(path: &str) {
    let child: Child = Command::new("docker")
        .arg("compose")
        .args(["--project-directory", path])
        .arg("up")
        .arg("-d")
        .arg("--build")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Fail docker compose up");

    child.wait_with_output().expect("Fail docker compose up");

    println!("Success docker compose up");
}
