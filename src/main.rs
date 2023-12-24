mod parser;
mod project;

use clap::Parser;
use parser::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    if let Some(commands) = cli.command {
        match commands {
            Commands::New { name, path } => project::create_project(name, path),
            Commands::Delete { name } => project::remove_project(name),
        }
    }
}
