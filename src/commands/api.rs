use anyhow::{bail, Result};
use std::fs;
use std::path::PathBuf;

/// Resolve a JSON Pointer like "#/components/schemas/Foo" against the document root.
fn resolve_ref<'a>(ref_str: &str, doc: &'a serde_json::Value) -> Option<&'a serde_json::Value> {
    let pointer = ref_str.strip_prefix('#')?;
    doc.pointer(pointer)
}

/// Recursively replace every {"$ref": "#/..."} with the referenced value.
fn inline_refs(value: &serde_json::Value, doc: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(ref_str) = map.get("$ref").and_then(|v| v.as_str()) {
                if let Some(resolved) = resolve_ref(ref_str, doc) {
                    return inline_refs(resolved, doc);
                }
            }
            let mut out = serde_json::Map::new();
            for (k, v) in map {
                out.insert(k.clone(), inline_refs(v, doc));
            }
            serde_json::Value::Object(out)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(|v| inline_refs(v, doc)).collect())
        }
        other => other.clone(),
    }
}

pub fn run(api_path: &str, service: &str) -> Result<()> {
    let ref_dir = PathBuf::from(".microservice").join(service).join("reference");

    if !ref_dir.is_dir() {
        bail!(
            "error: reference directory not found for service '{}'",
            service
        );
    }

    let yaml_files: Vec<PathBuf> = fs::read_dir(&ref_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("yaml"))
        .collect();

    if yaml_files.is_empty() {
        bail!(
            "error: no .yaml files found in reference directory for service '{}'",
            service
        );
    }

    for file in &yaml_files {
        let content = fs::read_to_string(file)?;
        let doc: serde_json::Value = match serde_yaml::from_str(&content) {
            Ok(v) => v,
            Err(_) => continue,
        };

        if let Some(entry) = doc.get("paths").and_then(|paths| paths.get(api_path)) {
            let resolved = inline_refs(entry, &doc);
            let mut wrapper = serde_json::Map::new();
            wrapper.insert(api_path.to_string(), resolved);
            println!("{}", serde_json::to_string_pretty(&serde_json::Value::Object(wrapper))?);
            return Ok(());
        }
    }

    bail!(
        "error: path '{}' not found in any .yaml file for service '{}'",
        api_path,
        service
    );
}
