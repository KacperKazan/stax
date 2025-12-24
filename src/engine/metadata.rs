use crate::git::refs;
use anyhow::Result;
use git2::Repository;
use serde::{Deserialize, Serialize};

/// Metadata stored for each tracked branch
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchMetadata {
    /// Name of the parent branch
    pub parent_branch_name: String,
    /// Commit SHA of parent when this branch was last rebased
    pub parent_branch_revision: String,
    /// PR information (if submitted)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr_info: Option<PrInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrInfo {
    pub number: u64,
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<bool>,
}

impl BranchMetadata {
    /// Create new metadata for a branch
    pub fn new(parent_name: &str, parent_revision: &str) -> Self {
        Self {
            parent_branch_name: parent_name.to_string(),
            parent_branch_revision: parent_revision.to_string(),
            pr_info: None,
        }
    }

    /// Read metadata for a branch from git refs
    pub fn read(repo: &Repository, branch: &str) -> Result<Option<Self>> {
        match refs::read_metadata(repo, branch)? {
            Some(json) => {
                let meta: Self = serde_json::from_str(&json)?;
                Ok(Some(meta))
            }
            None => Ok(None),
        }
    }

    /// Write metadata for a branch to git refs
    pub fn write(&self, repo: &Repository, branch: &str) -> Result<()> {
        let json = serde_json::to_string(self)?;
        refs::write_metadata(repo, branch, &json)
    }

    /// Delete metadata for a branch
    pub fn delete(repo: &Repository, branch: &str) -> Result<()> {
        refs::delete_metadata(repo, branch)
    }

    /// Check if the branch needs restacking (parent has moved)
    pub fn needs_restack(&self, repo: &Repository) -> Result<bool> {
        let parent_ref = repo.find_branch(&self.parent_branch_name, git2::BranchType::Local)?;
        let current_parent_rev = parent_ref.get().peel_to_commit()?.id().to_string();
        Ok(current_parent_rev != self.parent_branch_revision)
    }
}
