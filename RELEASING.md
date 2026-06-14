# Releasing egui-shadcn

This repo is both a Rust crate (`egui_shadcn`) and a Claude Code plugin. The
plugin is published simply by pushing to `main`: the marketplace entry sources it
via `github: oetiker/egui-shadcn` with **no ref pin**, so it tracks `main` HEAD.
Pushing *is* publishing. The `version` fields are what signal a new release to
users running `/plugin update`.

## Release checklist

1. **Land the changes** on `main` (build + `clippy --all-targets` warning-clean,
   all snapshot tests passing). Keep the canonical `src/` and the vendored
   `skills/egui-shadcn/registry/src/` **byte-identical**.

2. **Update `CHANGES.md`** — add a new `## [X.Y.Z] - YYYY-MM-DD` section
   (Keep a Changelog style: Added / Changed / Fixed). Update the compare links at
   the bottom. Pre-1.0, a breaking change is a **minor** bump.

3. **Bump the version in all four places** (they must agree):
   - `Cargo.toml` → `version`
   - `Cargo.lock` (run any `cargo` command to refresh it)
   - `.claude-plugin/plugin.json` → `version`
   - `.claude-plugin/marketplace.json` → `plugins[0].version`

4. **Bump the external marketplace** in the separate repo
   `oposs/claude-plugins` → `.claude-plugin/marketplace.json`, the `egui-shadcn`
   entry's `version` (and description if it changed). This is the listing users
   actually browse.

5. **Commit, tag, push** (this repo):
   ```bash
   git add -A && git commit -m "release: vX.Y.Z — <summary>"
   git tag -a vX.Y.Z -m "egui-shadcn vX.Y.Z"
   git push origin main && git push origin vX.Y.Z
   ```

6. **Commit + push the marketplace repo** (`oposs/claude-plugins`):
   ```bash
   git -C ../claude-plugins add .claude-plugin/marketplace.json
   git -C ../claude-plugins commit -m "egui-shadcn: bump to vX.Y.Z"
   git -C ../claude-plugins push origin main
   ```

7. **Verify**: both repos clean and in sync with origin; the tag is on the remote
   (`git ls-remote --tags origin`).

## Notes

- Tags are not required to publish (the plugin tracks `main`), but tag every
  release so `CHANGES.md` compare/release links resolve.
- All commits end with the `Co-Authored-By:` trailer when made by Claude.
- Build constraints when verifying: see `CLAUDE.md`.
