use std::path::Path;

use clap::Parser;

use crate::cli::Args;
use crate::docker::{execute_docker_compose_down, execute_docker_compose_up};
use crate::env::create_env;
use crate::folder::check_folder;
use crate::git::{git_clone, git_pull};

mod cli;
mod docker;
mod env;
mod folder;
mod git;

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.path);

    if !check_folder(path) {
        git_clone(&args.path, &args.git);
    }
    git_pull(&args.path);
    create_env(path, args.env_file, args.env, args.update_envs);

    let final_path = if let Some(dir) = args.execution_dir {
        Path::new(path)
            .join(dir)
            .to_str()
            .expect("Fail get path")
            .to_string()
    } else {
        Path::new(path).to_str().expect("Fail get path").to_string()
    };

    execute_docker_compose_down(&final_path, &args.docker_compose_file);
    execute_docker_compose_up(&final_path, &args.docker_compose_file);
}
