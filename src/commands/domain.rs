use anyhow::{bail, Result};
use std::fs;
use std::path::PathBuf;

pub fn run(domain: &str, service: &str) -> Result<()> {
    let base = PathBuf::from(".microservice").join(service);

    if !base.is_dir() {
        bail!("error: service '{}' not found", service);
    }

    let file = base.join("domains").join(format!("{domain}.md"));
    if !file.is_file() {
        bail!(
            "error: domain '{}' not found in service '{}'",
            domain,
            service
        );
    }

    let content = fs::read_to_string(&file)?;
    print!("{content}");
    Ok(())
}
