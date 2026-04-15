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

    let resolved = inline_refs(entry, &doc);
    let mut wrapper = serde_json::Map::new();
    wrapper.insert(api_path.to_string(), resolved);
    println!("{}", serde_json::to_string_pretty(&serde_json::Value::Object(wrapper))?);
    Ok(())
}
