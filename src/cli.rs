use clap::Parser;

/// A assistant pipeline cli
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Path to folder project
    #[arg(short = 'p', long)]
    pub path: String,
    /// Git url reference
    #[arg(short = 'g', long)]
    pub git: String,
    /// Docker envs
    #[arg(short = 'e', long)]
    pub env: Vec<String>,
    /// Append envs
    #[arg(short = 'u', long, default_value_t = false)]
    pub update_envs: bool,
    /// Env file location
    #[arg(short = 'E', long)]
    pub env_file: Option<String>,
    /// Docker compose file location
    #[arg(short = 'f', long)]
    pub docker_compose_file: Option<String>,
}