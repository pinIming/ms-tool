use anyhow::{bail, Result};
use std::fs;
use std::path::PathBuf;

pub fn run(api_path: &str, service: &str) -> Result<()> {
    let file = PathBuf::from(".microservice")
        .join(service)
        .join("reference")
        .join("openapi.yaml");

    if !file.is_file() {
        bail!(
            "error: openapi.yaml not found for service '{}'",
            service
        );
    }

    let content = fs::read_to_string(&file)?;
    let doc: serde_json::Value = serde_json::from_str(&content)?;

    let entry = doc
        .get("paths")
        .and_then(|paths| paths.get(api_path))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "error: path '{}' not found in openapi.yaml for service '{}'",
                api_path,
                service
            )
        })?;

    println!("{}", serde_json::to_string_pretty(entry)?);
    Ok(())
}
