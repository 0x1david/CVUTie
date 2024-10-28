use anyhow::Result;
mod config;

use clap::{Parser, Subcommand};
use config::Config;

const CONFIG: &str = ".cvutie";

#[derive(Parser)]
#[command(name = "cvutie")]
#[command(about = "CLI for (initially) compiling and testing BIK-PA1 projects.", long_about = None)]
#[command(version = "0.1")]
struct Cli {
    #[arg(short, long, global = true)]
    region: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a target C file
    Compile {
        // Try read region if not a path, else target directory path
        target: String,
        #[arg(long, short)]
        output: Option<String>,
    },

    /// Execute a target binary.
    Execute { target: String },

    /// Allows to pipe together CVUTie commands and python/bash scripts to create ad-hoc commands.
    Pipe {
        #[arg(long, short)]
        output: Option<String>,

        commands: Vec<String>,
    },

    /// Run tests for compilation and execution across the entire sub-directory
    TestAll { target: String },

    /// Add directory
    Region {
        /// Directories to add to a Region
        folders: Vec<String>,

        /// Add folders to a region (instead of overwriting a region)
        #[arg(short, long)]
        add: bool,

        /// Required for overwriting an existing region
        #[arg(long = "force")]
        force: bool,
    },
}

fn get_configuration() -> Config {
    let home = std::env::var_os("HOME");
    if home.is_none() {
        eprintln!("Could not find or read home directory. Please ensure $HOME environment variable is set");
        return Config::default();
    };
    let home = home.unwrap();
    let home = home.to_string_lossy();

    let config_path = format!("{}/{}", home, CONFIG);

    match Config::load(config_path) {
        Ok(config) => config,
        Err(_) => {
            println!(
                "Couldn't detect a `.cvutie` file in `{home}`. Creating a config file with defaults.."
            );
            let config = Config::default();
            if let Err(e) = config.save(home.to_string()) {
                if e.downcast_ref::<std::io::Error>().is_some() {
                    println!("Failed to create config file: No permission to write in home directory. Changing config won't be possible.");
                } else if e.downcast_ref::<serde_json::Error>().is_some() {
                    eprintln!(
                        "Failed to serialize config: Internal error occurred while writing config."
                    );
                    std::process::exit(1);
                } else {
                    eprintln!("An unexpected error occurred");
                    std::process::exit(1);
                };
            }
            config
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Verbose mode enabled");
        println!("Using repository at: {}", cli.git_dir);
    }

    match cli.command {
        Commands::Init { bare } => {
            println!("Initializing {}repository", if bare { "bare " } else { "" });
        }

        Commands::Status => {
            println!("Checking working tree status");
        }

        Commands::Add { files, update, all } => {
            if all {
                println!("Adding all changes");
            } else if update {
                println!("Adding modified files");
            } else {
                println!("Adding files: {:?}", files);
            }
        }

        Commands::Commit { message, amend } => {
            if amend {
                println!("Amending previous commit");
            } else {
                match message {
                    Some(msg) => println!("Creating commit with message: {}", msg),
                    None => println!("Opening editor for commit message"),
                }
            }
        }

        Commands::Log {
            number,
            one_line,
            stat,
        } => {
            println!(
                "Showing {} commits{}{}",
                number,
                if one_line { " in one-line format" } else { "" },
                if stat { " with stats" } else { "" }
            );
        }

        Commands::Checkout { branch, new_branch } => {
            if new_branch {
                println!("Creating and checking out new branch: {}", branch);
            } else {
                println!("Checking out branch: {}", branch);
            }
        }

        Commands::Branch {
            name,
            delete,
            remotes,
        } => match (name, delete, remotes) {
            (Some(branch), true, _) => println!("Deleting branch: {}", branch),
            (Some(branch), false, _) => println!("Creating branch: {}", branch),
            (None, _, true) => println!("Listing remote branches"),
            (None, _, false) => println!("Listing local branches"),
        },

        Commands::Pull {
            remote,
            branch,
            rebase,
        } => {
            println!(
                "Pulling {} from {}/{}",
                if rebase { "with rebase" } else { "with merge" },
                remote,
                branch
            );
        }
    }
}
