use std::process::{Child, Command, Stdio};

pub fn execute_docker_compose_down(path: &str){
    let child: Child = Command::new("docker")
        .arg("compose")
        .args(["--project-directory", path])
        .arg("down")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Fail docker compose down");

    child.wait_with_output().expect("Fail docker compose down");

    println!("Success docker compose down");
}

pub fn execute_docker_compose_up(path: &str) {
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
