use std::{fs, path::Path};

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm};

const COMPOSE_FILE: &str = include_str!("../docker-compose.yaml");

pub fn init_command(filename: &Path, force: bool) -> Result<bool, String> {
    if !force {
        require_no_similar_files(filename)?;
    }

    println!(
        "{}\n{}",
        format!(
            "Will create {} with the following contents:",
            filename.display()
        )
        .yellow(),
        COMPOSE_FILE
    );
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .default(true)
        .show_default(true)
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        fs::write(filename, COMPOSE_FILE)
            .map_err(|_| "Failed to create docker-compose file.".to_string())
            .map(|_| true)
    } else {
        Err("Aborting as requested.".to_string())
    }
}

fn require_no_similar_files(filename: &Path) -> Result<(), String> {
    for ext in ["yaml", "yml"] {
        let similar = filename.with_extension(ext);
        match fs::exists(&similar) {
            Ok(false) => (),
            Ok(true) => {
                return Err(format!(
                    "Compose file {} already exists.",
                    similar.display()
                ));
            }
            Err(e) => {
                return Err(format!("Cannot read current directory contents: {e}"));
            }
        }
    }
    Ok(())
}
