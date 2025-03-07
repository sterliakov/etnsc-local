use crate::FileSpec;

pub fn start_command(spec: &FileSpec) -> Result<(), String> {
    spec.require_compose_file()?;
    spec.run_docker_command(["up", "-d", "--force-recreate"])
        .map(|_| ())
        .map_err(|_| "Failed to launch the node".to_string())
}
