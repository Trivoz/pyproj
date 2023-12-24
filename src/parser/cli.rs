use std::path::PathBuf;

use clap::{arg, Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// create a new python project
    #[command(arg_required_else_help = true, alias = "n")]
    New {
        /// The name of the project
        #[arg(required = true, value_name = "project", value_enum)]
        name: String,

        /// Specify where to create the project
        ///
        /// Defaults to the current directory, unless `--path` is passed
        #[arg(long, value_name = "directory")]
        path: Option<PathBuf>,
    },

    /// delete a python project
    #[command(arg_required_else_help = true)]
    Delete {
        /// The name of the project
        #[arg(required = true, value_name = "project", value_enum)]
        name: String,
    },

    /// Activate the virtual environment in the current directory
    #[command()]
    Activate(Activate),
}

#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub struct Activate {
    /// The name of the project
    ///
    /// If no project is specified, the current directory is used
    #[arg(required = true, value_name = "project", value_enum)]
    pub name: Option<String>,

    /// Use pip to install dependencies if a requirements.txt file is found
    #[arg(long, default_value = "true", value_name = "install-deps", value_enum)]
    pub install_deps: bool,
}
