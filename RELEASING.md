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

4. **Commit, tag, push** (this repo):
   ```bash
   git add -A && git commit -m "release: vX.Y.Z — <summary>"
   git tag -a vX.Y.Z -m "egui-shadcn vX.Y.Z"
   git push origin main && git push origin vX.Y.Z
   ```

5. **The external marketplace follows on its own.** In `oposs/claude-plugins`,
   an hourly GitHub Action (`track-versions.yml`) copies each plugin's
   `plugin.json` version into `plugin-versions.json`; that commit is what makes
   Claude re-resolve the plugin. Do not bump versions there by hand. To publish
   without waiting for the hour:
   ```bash
   gh workflow run track-versions.yml -R oposs/claude-plugins
   ```
   The listing's description lives in that repo's
   `.claude-plugin/marketplace.json`; change it there (by PR) only when the
   plugin's purpose changed.

6. **Verify**: this repo is clean and in sync with origin, the tag is on the
   remote (`git ls-remote --tags origin`), and the marketplace records the new
   version:
   ```bash
   gh api repos/oposs/claude-plugins/contents/plugin-versions.json -q .content | base64 -d
   ```

## Notes

- Tags are not required to publish (the plugin tracks `main`), but tag every
  release so `CHANGES.md` compare/release links resolve.
- All commits end with the `Co-Authored-By:` trailer when made by Claude.
- Build constraints when verifying: see `CLAUDE.md`.
