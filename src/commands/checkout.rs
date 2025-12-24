use crate::engine::Stack;
use crate::git::GitRepo;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

pub fn run(branch: Option<String>) -> Result<()> {
    let repo = GitRepo::open()?;

    let target = match branch {
        Some(b) => b,
        None => {
            // Interactive selection
            let stack = Stack::load(&repo)?;
            let current = repo.current_branch()?;

            // Get all tracked branches (excluding trunk)
            let mut branches: Vec<String> = stack
                .branches
                .keys()
                .filter(|b| *b != &stack.trunk)
                .cloned()
                .collect();
            branches.sort();

            if branches.is_empty() {
                println!("No tracked branches. Use `gt branch track` to track a branch.");
                return Ok(());
            }

            // Find current index
            let default_idx = branches.iter().position(|b| b == &current).unwrap_or(0);

            // Build display items with indicators
            let items: Vec<String> = branches
                .iter()
                .map(|b| {
                    let branch_info = stack.branches.get(b);
                    let mut display = b.clone();
                    if let Some(info) = branch_info {
                        if info.needs_restack {
                            display.push_str(" (needs restack)");
                        }
                        if let Some(pr) = info.pr_number {
                            display.push_str(&format!(" #{}", pr));
                        }
                    }
                    if b == &current {
                        display.push_str(" ◀");
                    }
                    display
                })
                .collect();

            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select branch")
                .items(&items)
                .default(default_idx)
                .interact()?;

            branches[selection].clone()
        }
    };

    repo.checkout(&target)?;
    println!("Switched to branch '{}'", target);

    Ok(())
}
