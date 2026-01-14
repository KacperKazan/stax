use anyhow::Result;
use std::fs;
use std::path::Path;

/// Represents a discovered PR template
#[derive(Debug, Clone)]
#[allow(dead_code)] // Will be used in future tasks
pub struct PrTemplate {
    /// Display name (e.g., "feature", "bugfix", "Default")
    pub name: String,
    /// Full file path
    pub path: std::path::PathBuf,
    /// Template content (loaded lazily in real usage, but eager in tests)
    pub content: String,
}

/// Discover all PR templates in standard GitHub locations
///
/// Priority order:
/// 1. .github/PULL_REQUEST_TEMPLATE/ directory - scan for all .md files
/// 2. .github/PULL_REQUEST_TEMPLATE.md - single template (named "Default")
/// 3. .github/pull_request_template.md - lowercase variant
/// 4. docs/PULL_REQUEST_TEMPLATE.md
/// 5. docs/pull_request_template.md
#[allow(dead_code)] // Will be used in future tasks
pub fn discover_pr_templates(workdir: &Path) -> Result<Vec<PrTemplate>> {
    let mut templates = Vec::new();

    // Check directory first (multiple templates)
    let template_dir = workdir.join(".github/PULL_REQUEST_TEMPLATE");
    if template_dir.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(&template_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .map(|ext| ext == "md")
                    .unwrap_or(false)
            })
            .collect();

        entries.sort_by_key(|entry| entry.path());

        for entry in entries {
            let path = entry.path();
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("template")
                .to_string();

            let content = fs::read_to_string(&path)?;

            templates.push(PrTemplate {
                name,
                path,
                content,
            });
        }

        if !templates.is_empty() {
            return Ok(templates);
        }
    }

    // Check single template locations
    let single_template_candidates = [
        ".github/PULL_REQUEST_TEMPLATE.md",
        ".github/pull_request_template.md",
        "docs/PULL_REQUEST_TEMPLATE.md",
        "docs/pull_request_template.md",
    ];

    for candidate in &single_template_candidates {
        let path = workdir.join(candidate);
        if path.is_file() {
            let content = fs::read_to_string(&path)?;
            templates.push(PrTemplate {
                name: "Default".to_string(),
                path,
                content,
            });
            return Ok(templates);
        }
    }

    Ok(templates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_discover_single_template() {
        let dir = TempDir::new().unwrap();
        let github_dir = dir.path().join(".github");
        fs::create_dir(&github_dir).unwrap();
        fs::write(
            github_dir.join("PULL_REQUEST_TEMPLATE.md"),
            "# Single template"
        ).unwrap();

        let templates = discover_pr_templates(dir.path()).unwrap();
        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0].name, "Default");
        assert!(templates[0].content.contains("Single template"));
    }

    #[test]
    fn test_discover_multiple_templates() {
        let dir = TempDir::new().unwrap();
        let template_dir = dir.path().join(".github/PULL_REQUEST_TEMPLATE");
        fs::create_dir_all(&template_dir).unwrap();

        fs::write(template_dir.join("feature.md"), "# Feature").unwrap();
        fs::write(template_dir.join("bugfix.md"), "# Bugfix").unwrap();

        let templates = discover_pr_templates(dir.path()).unwrap();
        assert_eq!(templates.len(), 2);

        let names: Vec<_> = templates.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"bugfix"));
        assert!(names.contains(&"feature"));
    }

    #[test]
    fn test_discover_no_templates() {
        let dir = TempDir::new().unwrap();
        let templates = discover_pr_templates(dir.path()).unwrap();
        assert_eq!(templates.len(), 0);
    }
}
