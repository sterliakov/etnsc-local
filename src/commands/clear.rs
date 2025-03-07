use crate::FileSpec;

use super::start_command;

pub fn clear_command(spec: &FileSpec) -> Result<(), String> {
    spec.require_compose_file()?;
    start_command(spec)?;
    spec.run_docker_command([
        "exec",
        "-T",
        "electroneum-node",
        "sh",
        "-c",
        "rm -rf /opt/data/*",
    ])
    .map(|_| ())
    .map_err(|_| "Failed to clear".to_string())?;
    start_command(spec)
}
