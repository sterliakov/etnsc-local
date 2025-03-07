use crate::FileSpec;

pub fn stop_command(spec: &FileSpec) -> Result<(), String> {
    spec.require_compose_file()?;
    spec.run_docker_command(["down"])
        .map(|_| ())
        .map_err(|_| "Failed to launch the node".to_string())
}
