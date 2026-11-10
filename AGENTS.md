# Agent guidance

## Website work

Before changing the Cinch website, read [website/AGENTS.md](website/AGENTS.md). It routes to the detailed [design handbook](website/design/design-handbook.md) and [inspiration atlas](website/design/inspiration-atlas.md), including the user's archived images, source links, and design decisions.

Open the relevant visual references before interpreting or extending the aesthetic. Preserve the selected typography, material language, bold composition, and reading-page themes unless the user's current request changes them. Consult the website instructions for implementation ownership and verification.

These website-specific notes apply to site work. Keep unrelated compiler, bootstrap, tooling, and deployment changes intact when working on the site.

## Workspaces and parallel lanes

The repository is a colocated jj and git repository. Parallel work happens in jj workspaces, one per lane, never in the main checkout.

- The main checkout (`designing-a-safe-language`) is the integration point. It holds `main`. Nobody edits there while lanes are in flight; only integration (landing lanes, final full gates, publishing) happens there.
- Every lane gets its own workspace as a sibling directory, made with `jws` (`~/.local/bin/jws`): `jws new <lane>` creates `../dasl-<lane>` on `main`. A lane is one agent, one stage-chain build, or one frozen test snapshot. Use `jws list`, `jws path <lane>`, `jws log <lane>`.
- Build outputs live inside the tree (`compiler/target`, `build/bootstrap`), so each lane has its own and cannot pick up another lane's half-finished edits. A fresh lane needs its own cargo build.
- Inside a lane, finish work with `jj commit -m "..."` (or `jj describe` plus `jj new`). Every jj command snapshots the working copy, so an interrupted lane loses nothing; resume by opening the same workspace.
- Landing: from the main checkout, `jws land <lane>` rebases the lane's committed changes onto `main` and moves `main`. The lane's uncommitted working copy stays in the lane. Then run the integration gates in the main checkout, and `jws rm <lane>` when the lane is done (it refuses while the lane has changes not in `main`).
- Plain git still works for reading history and pushing (`jj git push --bookmark main`, or git). Do not run `jj op restore` or `jj undo` in the main checkout while lanes are running.
