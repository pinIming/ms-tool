use anyhow::{bail, Result};
use std::fs;
use std::path::PathBuf;

pub fn run(service: &str) -> Result<()> {
    let base = PathBuf::from(".microservice").join(service);

    if !base.is_dir() {
        bail!("error: service '{}' not found at .microservice/{}/", service, service);
    }

    let file = base.join("overview.md");
    if !file.is_file() {
        bail!(
            "error: overview file not found: .microservice/{}/overview.md",
            service
        );
    }

    let content = fs::read_to_string(&file)?;
    print!("{content}");
    Ok(())
}
