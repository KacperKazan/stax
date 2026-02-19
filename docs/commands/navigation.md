# Navigation and Stack View

## Navigation commands

| Command | What it does |
|---|---|
| `stax u` | Move up to child branch |
| `stax d` | Move down to parent branch |
| `stax u 3` | Move up 3 branches |
| `stax d 2` | Move down 2 branches |
| `stax top` | Jump to stack tip |
| `stax bottom` | Jump to stack base |
| `stax trunk` / `stax t` | Jump to trunk |
| `stax prev` | Toggle to previous branch |
| `stax co` | Interactive branch picker |

## Checkout shortcuts

Use `stax checkout` (or `stax co`) with navigation flags:

- `stax checkout --trunk` jump directly to trunk
- `stax checkout --parent` jump to parent of current branch
- `stax checkout --child 1` jump to first child branch

## Reading `stax ls`

```text
○        feature/validation 1↑
◉        feature/auth 1↓ 2↑ ⟳
│ ○    ☁ feature/payments PR #42
○─┘    ☁ main
```

| Symbol | Meaning |
|---|---|
| `◉` | Current branch |
| `○` | Other tracked branch |
| `☁` | Remote tracking exists |
| `1↑` | Commits ahead of parent |
| `1↓` | Commits behind parent |
| `⟳` | Needs restack |
| `PR #42` | Open pull request |
