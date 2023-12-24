use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
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
}
