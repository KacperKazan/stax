use anyhow::{Context, Result};
use git2::Repository;
use std::process::Command;

const METADATA_REF_PREFIX: &str = "refs/branch-metadata/";

/// Read metadata JSON for a branch from git refs
pub fn read_metadata(repo: &Repository, branch: &str) -> Result<Option<String>> {
    let ref_name = format!("{}{}", METADATA_REF_PREFIX, branch);

    match repo.find_reference(&ref_name) {
        Ok(reference) => {
            let oid = reference.target().context("Reference has no target")?;
            let blob = repo.find_blob(oid)?;
            let content = std::str::from_utf8(blob.content())?;
            Ok(Some(content.to_string()))
        }
        Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Write metadata JSON for a branch to git refs
pub fn write_metadata(repo: &Repository, branch: &str, json: &str) -> Result<()> {
    let workdir = repo
        .workdir()
        .context("Repository has no working directory")?;

    // Create blob with json content
    let mut child = Command::new("git")
        .args(["hash-object", "-w", "--stdin"])
        .current_dir(workdir)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        use std::io::Write;
        stdin.write_all(json.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    let hash = String::from_utf8(output.stdout)?.trim().to_string();

    // Update the ref to point to the blob
    let ref_name = format!("{}{}", METADATA_REF_PREFIX, branch);
    Command::new("git")
        .args(["update-ref", &ref_name, &hash])
        .current_dir(workdir)
        .status()
        .context("Failed to update ref")?;

    Ok(())
}

/// Delete metadata ref for a branch
pub fn delete_metadata(repo: &Repository, branch: &str) -> Result<()> {
    let ref_name = format!("{}{}", METADATA_REF_PREFIX, branch);
    let workdir = repo
        .workdir()
        .context("Repository has no working directory")?;

    Command::new("git")
        .args(["update-ref", "-d", &ref_name])
        .current_dir(workdir)
        .status()
        .context("Failed to delete ref")?;

    Ok(())
}

/// List all branches that have metadata
pub fn list_metadata_branches(repo: &Repository) -> Result<Vec<String>> {
    let mut branches = Vec::new();

    for reference in repo.references_glob(&format!("{}*", METADATA_REF_PREFIX))? {
        let reference = reference?;
        if let Some(name) = reference.name() {
            let branch = name.strip_prefix(METADATA_REF_PREFIX).unwrap_or(name);
            branches.push(branch.to_string());
        }
    }

    Ok(branches)
}
