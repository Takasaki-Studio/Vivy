use clap::Parser;

/// A assistant pipeline cli
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Path to folder project
    #[arg(short, long)]
    pub path: String,
    /// Git url reference
    #[arg(short, long)]
    pub git: String,
    /// Docker envs
    #[arg(short, long)]
    pub env: Vec<String>,
    /// Docker compose file
    #[arg(short = 'f', long)]
    pub docker_compose_file: Option<String>
}