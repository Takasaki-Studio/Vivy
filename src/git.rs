use std::process::{Command, Stdio};

pub enum GitOperations<'a> {
    Clone(&'a str),
    Pull,
}

pub fn git(path: &str, op: GitOperations) {
    let (args, op_name) = match op {
        GitOperations::Pull => (vec!["pull"], "pull"),
        GitOperations::Clone(repo) => (vec!["clone", repo], "clone"),
    };

    let cmd = Command::new("git")
        .args(["-C", path])
        .args(args)
        .arg(".")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn git");

    cmd.wait_with_output()
        .unwrap_or_else(|_| panic!("Failed to {} repository", op_name));

    println!("{} success", op_name);
}
