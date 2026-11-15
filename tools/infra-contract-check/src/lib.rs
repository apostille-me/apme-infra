//! Offline source-contract checks. No network, credentials, Python, or live infrastructure.
#![forbid(unsafe_code)]

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use serde_json::Value;

pub fn validate_catalog_text(input: &str) -> Result<(), String> {
    let document: Value =
        serde_json::from_str(input).map_err(|error| format!("invalid services JSON: {error}"))?;
    let object = document
        .as_object()
        .ok_or_else(|| "catalog must be a JSON object".to_string())?;
    if object.get("schema_version").and_then(Value::as_u64) != Some(1) {
        return Err("catalog schema_version must be 1".into());
    }
    if object.get("organization").and_then(Value::as_str) != Some("apostille-me") {
        return Err("catalog organization must be apostille-me".into());
    }
    let services = object
        .get("services")
        .and_then(Value::as_array)
        .ok_or_else(|| "catalog services must be an array".to_string())?;
    if services.is_empty() {
        return Err("catalog must declare at least one service".into());
    }
    let mut unique = BTreeSet::new();
    for service in services {
        let value = service
            .as_str()
            .ok_or_else(|| "catalog service identifiers must be strings".to_string())?;
        if !value.starts_with("apme-")
            || !value.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        {
            return Err("catalog service identifiers must be lowercase apme-* names".into());
        }
        if !unique.insert(value) {
            return Err(format!("duplicate catalog service: {value}"));
        }
    }
    Ok(())
}

pub fn validate_manifest_text(input: &str) -> Result<(), String> {
    let mut has_api_version = false;
    let mut has_kind = false;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with("apiVersion:") && !line["apiVersion:".len()..].trim().is_empty() {
            has_api_version = true;
        }
        if line.starts_with("kind:") && !line["kind:".len()..].trim().is_empty() {
            has_kind = true;
        }
    }
    if !has_api_version {
        return Err("Kubernetes manifest lacks apiVersion".into());
    }
    if !has_kind {
        return Err("Kubernetes manifest lacks kind".into());
    }
    Ok(())
}

fn collect_yaml_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(root)
        .map_err(|error| format!("cannot read Kubernetes source directory {}: {error}", root.display()))?;
    let mut entries = entries
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("cannot enumerate Kubernetes files: {error}"))?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        let kind = fs::symlink_metadata(&path)
            .map_err(|error| format!("cannot inspect {}: {error}", path.display()))?
            .file_type();
        if kind.is_symlink() {
            return Err(format!("Kubernetes source includes a symlink: {}", path.display()));
        }
        if kind.is_dir() {
            collect_yaml_files(&path, files)?;
        } else if kind.is_file()
            && path.extension().is_some_and(|ext| ext == "yaml")
        {
            files.push(path);
        }
    }
    Ok(())
}

pub fn validate_repository(root: &Path) -> Result<usize, Vec<String>> {
    let mut problems = Vec::new();
    let catalog_path = root.join("catalog/services.json");
    match fs::read_to_string(&catalog_path) {
        Ok(content) => {
            if let Err(problem) = validate_catalog_text(&content) {
                problems.push(format!("catalog/services.json: {problem}"));
            }
        }
        Err(error) => problems.push(format!("cannot read catalog/services.json: {error}")),
    }

    let mut manifests = Vec::new();
    if let Err(error) = collect_yaml_files(&root.join("k8s"), &mut manifests) {
        problems.push(error);
    }
    if manifests.is_empty() {
        problems.push("no Kubernetes YAML files found under k8s/".into());
    }
    for path in &manifests {
        match fs::read_to_string(path) {
            Ok(content) => {
                if let Err(error) = validate_manifest_text(&content) {
                    let name = path.strip_prefix(root).unwrap_or(path).display();
                    problems.push(format!("{name}: {error}"));
                }
            }
            Err(error) => problems.push(format!("cannot read {}: {error}", path.display())),
        }
    }
    if problems.is_empty() {
        Ok(manifests.len())
    } else {
        Err(problems)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD_CATALOG: &str = r#"{
        "schema_version": 1,
        "organization": "apostille-me",
        "services": ["apme-api", "apme-sync"]
    }"#;
    const GOOD_YAML: &str = "apiVersion: apps/v1\nkind: Deployment\nmetadata:\n  name: test\n";

    #[test]
    fn accepts_valid_offline_contracts() {
        assert!(validate_catalog_text(GOOD_CATALOG).is_ok());
        assert!(validate_manifest_text(GOOD_YAML).is_ok());
    }

    #[test]
    fn malformed_or_wrong_organization_catalog_fails_closed() {
        assert!(validate_catalog_text("{broken").is_err());
        assert!(validate_catalog_text("[]").is_err());
        assert!(validate_catalog_text(&GOOD_CATALOG.replace("apostille-me", "another-org")).is_err());
        assert!(validate_catalog_text(&GOOD_CATALOG.replace("schema_version\": 1", "schema_version\": 2")).is_err());
    }

    #[test]
    fn refuses_duplicate_and_noncanonical_services() {
        let duplicated = GOOD_CATALOG.replace("\"apme-api\", \"apme-sync\"", "\"apme-api\", \"apme-api\"");
        assert!(validate_catalog_text(&duplicated).is_err());
        for invalid in ["API", "apme_", "../secrets", "apme-A", ""] {
            let mutated = GOOD_CATALOG.replace("apme-sync", invalid);
            assert!(validate_catalog_text(&mutated).is_err(), "accepted {invalid:?}");
        }
    }

    #[test]
    fn rejects_missing_or_commented_manifest_identity() {
        assert!(validate_manifest_text("# apiVersion: v1\nkind: Service\n").is_err());
        assert!(validate_manifest_text("apiVersion: v1\n# kind: Service\n").is_err());
        assert!(validate_manifest_text("apiVersion:\nkind: Service\n").is_err());
        assert!(validate_manifest_text("apiVersion: v1\nkind:\n").is_err());
        assert!(validate_manifest_text("").is_err());
    }
}
