# stax co Minimal TUI Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace the `stax co` picker tree with a minimalist, stacked, column-aligned list that is faster to scan.

**Architecture:** Extract row-building into pure helpers that compute ordering, depth, and display fields from `Stack`. Render a dense, aligned string per row and feed it to `dialoguer::FuzzySelect`.

**Tech Stack:** Rust, dialoguer, existing `Stack` + `GitRepo` helpers.

### Task 1: Add failing unit tests for checkout row building

**Files:**
- Modify: `src/commands/checkout.rs`

**Step 1: Write failing tests**

```rust
#[test]
fn test_branch_prefix_depths() {
    assert_eq!(branch_prefix(0, false), "• ");
    assert_eq!(branch_prefix(1, false), "│ ○ ");
    assert_eq!(branch_prefix(2, false), "│ │ ○ ");
    assert_eq!(branch_prefix(1, true), "│ ● ");
}

#[test]
fn test_checkout_row_order_current_stack_first() {
    let stack = test_stack();
    let rows = build_checkout_row_data(&stack, "auth-ui", |_p, _b| Some((0, 0)));
    let names: Vec<_> = rows.iter().map(|r| r.branch.as_str()).collect();
    assert_eq!(
        names,
        vec!["auth", "auth-api", "auth-ui", "hotfix", "main"]
    );
}
```

**Step 2: Run tests to verify failure**

Run: `cargo test test_branch_prefix_depths test_checkout_row_order_current_stack_first -v`
Expected: FAIL (functions not yet defined)

### Task 2: Implement minimal checkout row builder and formatter

**Files:**
- Modify: `src/commands/checkout.rs`

**Step 1: Add row data + helpers**

```rust
struct RowData {
    branch: String,
    depth: usize,
    stack_root: String,
    delta: Option<(usize, usize)>,
    pr_number: Option<u64>,
    needs_restack: bool,
    is_current: bool,
    is_trunk: bool,
}

fn branch_prefix(depth: usize, is_current: bool) -> String {
    let mut prefix = String::new();
    if depth > 0 {
        for _ in 0..depth {
            prefix.push_str("│ ");
        }
    }
    let glyph = if is_current { "●" } else if depth == 0 { "•" } else { "○" };
    prefix.push_str(glyph);
    prefix.push(' ');
    prefix
}
```

**Step 2: Build ordered rows (current stack first)**

```rust
fn build_checkout_row_data<F>(
    stack: &Stack,
    current: &str,
    ahead_behind: F,
) -> Vec<RowData>
where
    F: Fn(&str, &str) -> Option<(usize, usize)>,
{
    // roots = trunk children, current root first
    // preorder traversal, depth from root
    // append trunk row last
}
```

**Step 3: Format rows into display strings**

```rust
struct CheckoutRow {
    branch: String,
    display: String,
}

fn format_checkout_rows(rows: &[RowData]) -> Vec<CheckoutRow> {
    // compute column widths, align with double-space separators
}
```

**Step 4: Wire into `run`**

Replace the old tree-building logic with:

```rust
let rows = build_checkout_row_data(&stack, &current, |parent, branch| {
    repo.commits_ahead_behind(parent, branch).ok()
});
let rows = format_checkout_rows(&rows);
let items: Vec<String> = rows.iter().map(|r| r.display.clone()).collect();
let branch_names: Vec<String> = rows.iter().map(|r| r.branch.clone()).collect();
```

Keep the same prompt, and keep the “No branches found” guard.

### Task 3: Verify tests

**Step 1: Run focused tests**

Run: `cargo test test_branch_prefix_depths test_checkout_row_order_current_stack_first -v`
Expected: PASS

**Step 2: (Optional) Full suite**

Run: `cargo test`
Expected: PASS

### Task 4: Commit (optional, user-driven)

```bash
git add src/commands/checkout.rs
# user will commit
```
