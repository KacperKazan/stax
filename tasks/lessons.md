# Lessons

- When changing the `stax co` UI, match the `stax ls` visual language (colors, tree/indentation) and confirm it visually. Do not ship a redesign without verifying the output looks like the `ls` tree and that selection emphasis is obvious.
- If interactive lists scroll the terminal on navigation, clear and position the cursor before invoking the dialog to avoid rendering into the lower viewport.
- When adding or changing CLI commands/flags, update both `README.md` and `docs/` command references in the same change and verify parity against `stax --help` before marking docs complete.
