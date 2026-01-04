use colored::Colorize;
use std::thread;
use std::time::Duration;
use update_informer::{registry, Check};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Spawn a background thread to check for updates.
/// This is non-blocking and won't affect CLI performance.
/// Results are cached by update-informer for 24 hours.
pub fn check_in_background() {
    thread::spawn(|| {
        let informer = update_informer::new(registry::Crates, PKG_NAME, PKG_VERSION)
            .timeout(Duration::from_secs(3))
            .interval(Duration::from_secs(60 * 60 * 24)); // 24 hours

        // This will either use cached result or make a network request
        // The result is cached for the next run
        let _ = informer.check_version();
    });
}

/// Check for cached update info and display if a new version is available.
/// This reads from cache only - it won't make network requests or block.
pub fn show_update_notification() {
    // Use a very short timeout so this never blocks
    // If there's no cached result, this returns quickly
    let informer = update_informer::new(registry::Crates, PKG_NAME, PKG_VERSION)
        .timeout(Duration::from_millis(1))
        .interval(Duration::from_secs(60 * 60 * 24));

    if let Ok(Some(new_version)) = informer.check_version() {
        eprintln!();
        eprintln!(
            "{} {} → {} {}",
            "A new version of stax is available:".yellow(),
            PKG_VERSION.dimmed(),
            new_version.to_string().green().bold(),
            "(cargo install stax)".dimmed()
        );
    }
}
