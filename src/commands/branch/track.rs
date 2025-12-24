use crate::engine::BranchMetadata;
use crate::git::GitRepo;
use anyhow::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

pub fn run(parent: Option<String>) -> Result<()> {
    let repo = GitRepo::open()?;
    let current = repo.current_branch()?;
    let trunk = repo.trunk_branch()?;

    // Check if already tracked
    if BranchMetadata::read(repo.inner(), &current)?.is_some() {
        println!(
            "Branch '{}' is already tracked. Use restack to update.",
            current.yellow()
        );
        return Ok(());
    }

    // Determine parent
    let parent_branch = match parent {
        Some(p) => p,
        None => {
            // Interactive selection
            let mut branches = repo.list_branches()?;
            branches.retain(|b| b != &current);
            branches.sort();

            // Put trunk first
            if let Some(pos) = branches.iter().position(|b| b == &trunk) {
                branches.remove(pos);
                branches.insert(0, trunk.clone());
            }

            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select parent branch")
                .items(&branches)
                .default(0)
                .interact()?;

            branches[selection].clone()
        }
    };

    let parent_rev = repo.branch_commit(&parent_branch)?;

    // Create metadata
    let meta = BranchMetadata::new(&parent_branch, &parent_rev);
    meta.write(repo.inner(), &current)?;

    println!(
        "Tracking '{}' with parent '{}'",
        current.green(),
        parent_branch.blue()
    );

    Ok(())
}
