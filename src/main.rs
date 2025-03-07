use std::{
    ffi::OsStr,
    path::PathBuf,
    process::{Command, Stdio},
};

use clap::{Args, Parser, Subcommand};

mod commands;

const DEFAULT_FILE: &str = "docker-compose.yaml";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure the project: create a docker-compose skeleton file.
    ///
    /// You will be able to edit it freely after generation.
    Init(Init),
    /// Start the local node as defined in docker-compose.yaml.
    ///
    /// Any newly added accounts will be seeded. Accounts that already exist
    /// will not be changed.
    Start(FileSpec),
    /// Stop the local node.
    ///
    /// All data will be retained when started again).
    Stop(FileSpec),
    /// Clear remains from previous runs.
    ///
    /// This will erase all transactions history.
    Clear(FileSpec),
}

#[derive(Args)]
struct Init {
    /// docker-compose file name to use.
    #[arg(short = 'f', long = "file", env = "ETNSC_COMPOSE_FILE", default_value=DEFAULT_FILE)]
    file: PathBuf,
    /// Allow overwriting existing file with the same name.
    #[arg(short = 'F', long = "force", action)]
    force: bool,
}

#[derive(Args)]
struct FileSpec {
    /// docker-compose file name to use.
    #[arg(short = 'f', long = "file", env = "ETNSC_COMPOSE_FILE", default_value=DEFAULT_FILE)]
    file: PathBuf,
    /// Display docker compose output and print more information.
    #[arg(short = 'v', long = "verbose", action)]
    verbose: bool,
}

impl FileSpec {
    pub fn run_docker_command<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(
        &self,
        args: I,
    ) -> Result<(), std::io::Error> {
        let mut cmd = Command::new("docker");
        let mut cmd = cmd.arg("compose").arg("-f").arg(&self.file).args(args);
        if !self.verbose {
            cmd = cmd.stdout(Stdio::null()).stderr(Stdio::null());
        }
        match cmd.status() {
            Ok(c) if c.success() => Ok(()),
            Ok(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Command execution failed.",
            )),
            Err(e) => Err(e),
        }
    }

    pub fn require_compose_file(&self) -> Result<(), String> {
        if self.file.exists() {
            Ok(())
        } else {
            Err(format!(
                "Compose file not found at {}.",
                self.file.display()
            ))
        }
    }
}

fn entrypoint(cli: Cli) -> Result<(), String> {
    match &cli.command {
        Commands::Init(Init { force, file }) => commands::init_command(file, *force),
        Commands::Start(spec) => commands::start_command(spec),
        Commands::Stop(spec) => commands::stop_command(spec),
        Commands::Clear(spec) => commands::clear_command(spec),
    }
}

fn main() {
    let cli = Cli::parse();
    match entrypoint(cli) {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Command execution error:\n{e}"),
    }
}
