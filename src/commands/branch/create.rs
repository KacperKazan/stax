use crate::engine::BranchMetadata;
use crate::git::GitRepo;
use anyhow::Result;
use colored::Colorize;

pub fn run(name: &str) -> Result<()> {
    let repo = GitRepo::open()?;
    let current = repo.current_branch()?;
    let current_rev = repo.branch_commit(&current)?;

    // Create the branch
    repo.create_branch(name)?;

    // Track it with current branch as parent
    let meta = BranchMetadata::new(&current, &current_rev);
    meta.write(repo.inner(), name)?;

    // Checkout the new branch
    repo.checkout(name)?;

    println!(
        "Created and switched to branch '{}' (stacked on {})",
        name.green(),
        current.blue()
    );

    Ok(())
}
