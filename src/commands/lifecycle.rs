use crate::FileSpec;
use colored::Colorize;

pub fn start_command(spec: &FileSpec) -> Result<bool, String> {
    spec.require_compose_file()?;
    spec.run_docker_command(["up", "-d", "--force-recreate", "--wait"])
        .map_err(|e| e.to_string())?;
    Ok(true)
}

pub fn stop_command(spec: &FileSpec) -> Result<bool, String> {
    spec.require_compose_file()?;
    spec.run_docker_command(["down"])
        .map_err(|e| e.to_string())?;
    Ok(true)
}

pub fn reset_command(spec: &FileSpec) -> Result<bool, String> {
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
    .map_err(|e| e.to_string())?;
    start_command(spec)?;
    Ok(true)
}

pub fn status_command(spec: &FileSpec) -> Result<bool, String> {
    spec.require_compose_file()?;
    {
        let mut silent_spec = spec.clone();
        silent_spec.verbose = false;
        let ps = silent_spec
            .run_docker_command([
                "ps",
                "--format",
                "{{ .Service }}: {{ .State }}, {{ .Status }}",
            ])
            .map_err(|e| e.to_string())?;
        let colored = if ps.contains("electroneum-node: running") {
            ps.green()
        } else {
            ps.red()
        };
        println!("{colored}");
    }
    if spec.verbose {
        spec.run_docker_command(["logs"])
            .map_err(|e| e.to_string())?;
    }
    Ok(false)
}
