# Plan
- [x] Add `branch untrack` command wiring in CLI (`BranchCommands`, dispatch, and module exports).
- [x] Implement `/Users/cesarferreira/code/github/stax/src/commands/branch/untrack.rs` to remove only stax metadata for target branch (default: current).
- [x] Add/upgrade tests to verify untrack removes metadata while keeping the Git branch.
- [x] Run targeted tests for untrack and branch help/coverage.
- [x] Document review summary.

# Review
- [x] Added new CLI subcommand `stax branch untrack [branch]` (alias: `ut`) and wired dispatch in `/Users/cesarferreira/code/github/stax/src/main.rs`.
- [x] Added command module export in `/Users/cesarferreira/code/github/stax/src/commands/branch/mod.rs` and new implementation in `/Users/cesarferreira/code/github/stax/src/commands/branch/untrack.rs`.
- [x] Upgraded `/Users/cesarferreira/code/github/stax/tests/additional_coverage_tests.rs` `test_branch_untrack` to assert metadata deletion and branch retention.
- [x] Updated help/coverage checks in `/Users/cesarferreira/code/github/stax/tests/cli_tests.rs` and `/Users/cesarferreira/code/github/stax/tests/command_coverage_tests.rs`.
- [x] Updated docs references in `/Users/cesarferreira/code/github/stax/README.md` and `/Users/cesarferreira/code/github/stax/skills.md`.
- [x] Validation commands run: `cargo fmt`; `cargo test test_branch_untrack -- --nocapture`; `cargo test test_branch_subcommands -- --nocapture`; `cargo test test_branch_untrack_help -- --nocapture`; `cargo test fp_parity_b_branch -- --nocapture`.
