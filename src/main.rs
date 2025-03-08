use std::{
    ffi::OsStr,
    io,
    path::PathBuf,
    process::{Command, Stdio},
};

use clap::{Args, Parser, Subcommand};
use colored::Colorize;

mod commands;

const DEFAULT_FILE: &str = "docker-compose.yaml";

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
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
    /// Start the local node as defined in the selected compose file.
    ///
    /// Any newly added accounts will be seeded. Accounts that already exist
    /// will not be changed.
    Start(FileSpec),
    /// Stop the local node. All data will be retained when started again.
    Stop(FileSpec),
    /// Reset exiting data. This will erase all transactions history.
    Reset(FileSpec),
    /// Get current node status and logs (if verbose).
    Status(FileSpec),
}

#[derive(Args)]
struct Init {
    /// docker compose file name to use.
    #[arg(short='f', long="file", env="ETNSC_COMPOSE_FILE", default_value=DEFAULT_FILE)]
    file: PathBuf,
    /// Allow overwriting existing file with the same name.
    #[arg(short = 'F', long = "force", action)]
    force: bool,
}

#[derive(Args, Clone)]
struct FileSpec {
    /// docker compose file name to use.
    #[arg(short='f', long="file", env="ETNSC_COMPOSE_FILE", default_value=DEFAULT_FILE)]
    file: PathBuf,
    /// Display docker compose output and print more information.
    #[arg(short = 'v', long = "verbose", action)]
    verbose: bool,
}

impl FileSpec {
    pub fn run_docker_command<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(
        &self,
        args: I,
    ) -> io::Result<String> {
        let mut cmd = Command::new("docker");
        let cmd = cmd
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let output = cmd.output();
        let (stderr, stdout) = match output {
            Ok(ref c) => (
                String::from_utf8(c.stderr.clone())
                    .unwrap_or("Failed to decode output".to_string()),
                String::from_utf8(c.stdout.clone())
                    .unwrap_or("Failed to decode output".to_string()),
            ),
            Err(_) => (String::new(), String::new()),
        };
        if self.verbose {
            if !stdout.is_empty() {
                println!("Command output:\n{}", stdout)
            }
            if !stderr.is_empty() {
                println!("Command stderr:\n{}", stderr)
            }
        }
        let combined = if stderr.is_empty() && stdout.is_empty() {
            "No output found.".to_string()
        } else if stderr.is_empty() {
            stdout
        } else if stdout.is_empty() {
            stderr
        } else {
            [stderr, stdout].join("\n")
        };
        match output {
            Ok(c) if c.status.success() => Ok(combined),
            Ok(_) => {
                let logs = self
                    .run_docker_command(["logs"])
                    .unwrap_or("Failed to retrieve the logs".to_string());
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Command execution failed. Full output:\n{}\nCompose logs:\n{}",
                        combined, logs
                    ),
                ))
            }
            Err(e) => Err(e),
        }
    }

    pub fn require_compose_file(&self) -> Result<(), String> {
        if self.file.exists() {
            Ok(())
        } else {
            Err(format!(
                "Compose file not found at {}. \
                Did you run `init`? Is ETNSC_COMPOSE_FILE or --file correct?",
                self.file.display()
            ))
        }
    }
}

fn entrypoint(cli: Cli) -> Result<bool, String> {
    match &cli.command {
        Commands::Init(Init { force, file }) => commands::init_command(file, *force),
        Commands::Start(spec) => commands::start_command(spec),
        Commands::Stop(spec) => commands::stop_command(spec),
        Commands::Reset(spec) => commands::reset_command(spec),
        Commands::Status(spec) => commands::status_command(spec),
    }
}

fn main() {
    let cli = Cli::parse();
    match entrypoint(cli) {
        Ok(true) => println!("{}", "Success!".green()),
        Ok(false) => {}
        Err(e) => println!("{}", format!("Command execution error:\n{e}").red()),
    }
}
