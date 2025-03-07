use std::{fs, path::Path};

const COMPOSE_FILE: &str = include_str!("../docker-compose.yaml");

pub fn init_command(filename: &Path, force: bool) -> Result<(), String> {
    if !force {
        for ext in ["yaml", "yml"] {
            match fs::exists(filename.with_extension(ext)) {
                Ok(false) => (),
                Ok(true) => {
                    return Err("Compose file already exists.".to_string());
                }
                Err(_) => {
                    return Err("Cannot read current directory contents.".to_string());
                }
            }
        }
    }
    fs::write(filename, COMPOSE_FILE)
        .map_err(|_| "Failed to create docker-compose file.".to_string())
}
