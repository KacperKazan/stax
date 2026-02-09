# Plan
- [x] Reproduce and codify the `branch delete` non-force failure for an empty child branch whose parent is ahead of trunk.
- [x] Update non-force deletion safety logic to accept branches merged into tracked parent (not only trunk), with trunk fallback.
- [x] Add regression test coverage for: empty child branch deletes without `--force` when merged into tracked parent.
- [x] Run targeted test suite covering branch delete paths and validate no regressions.
- [x] Document review results and behavior change summary.

# Review
- [x] Updated `/Users/cesarferreira/code/github/stax/src/git/repo.rs` non-force deletion logic to consider tracked parent merge status first, then trunk fallback.
- [x] Added regression unit test in `/Users/cesarferreira/code/github/stax/src/git/repo.rs` covering empty child branch deletion when parent is ahead of trunk.
- [x] Verified branch delete coverage with `cargo test test_delete_branch_non_force_allows_empty_branch_merged_into_parent -- --nocapture` and `cargo test test_branch_delete -- --nocapture`.
- [x] Note: command-level non-force confirmation is interactive; regression is validated at `GitRepo::delete_branch` layer where merge safety decision is made.
