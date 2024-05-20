use std::process::{Child, Command, Stdio};

pub fn git_clone(path: &str, git: &str) {
    let child: Child = Command::new("git")
        .args(["-C", path])
        .args(["clone", git])
        .arg(".")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Fail clone repository");

    child.wait_with_output().expect("Fail clone repository");

    println!("Success to clone {}", git);
}

pub fn git_pull(path: &str) {
    let child: Child = Command::new("git")
        .args(["-C", path])
        .arg("pull")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Fail pull repository");

    child.wait_with_output().expect("Fail pull repository");

    println!("Success to pull");
}