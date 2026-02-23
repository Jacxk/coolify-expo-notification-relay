---
description: Bump to the next version from last tag, update Cargo.toml, commit, tag, push, and optionally create a release draft (handsfree except one confirmation).
argument-hint: [patch | minor | major, or leave empty to infer from diff]
---

Create a new release version by comparing the last git tag to the current branch (main or HEAD). The agent performs all steps automatically; the only prompt to the user is whether to create a GitHub release draft.

Requirements:
1. **Resolve the last tag**: Run `git describe --tags --abbrev=0` (or equivalent) to get the most recent tag. If no tag exists, treat the current version in `Cargo.toml` as the baseline and suggest the first tag (e.g. `v0.1.0` or current version).
2. **Diff scope**: Determine the diff from that tag to the current branch (e.g. `git diff <last-tag>..HEAD` or `..main` if user prefers). Use the same branch as in the repo (main/master) unless the user specifies otherwise in the command args.
3. **Version bump**:
   - If the user provided a bump type in the command args (`patch`, `minor`, or `major`), use it.
   - Otherwise, infer the bump from the diff and commit messages (e.g. breaking changes → major, new features → minor, fixes/docs/chore → patch). Prefer conservative (patch) when unclear.
4. **Compute next version**: Apply the bump to the version from the **last tag** (not necessarily the one in `Cargo.toml`). If the tag is `v0.2.0`, next patch is `0.2.1`, next minor is `0.3.0`, next major is `1.0.0`. Use semantic versioning (semver).
5. **Update Cargo.toml and Cargo.lock**: Edit the project's `Cargo.toml` and set `version` in `[package]` to the new version (no `v` prefix; e.g. `0.2.1` not `v0.2.1`). Then run `cargo build` or `cargo check` so Cargo updates `Cargo.lock` with the new package version. Do the edit in the workspace so the files are actually updated.
6. **Run the release steps (handsfree)**. Execute these in order; do not ask for confirmation for any of these:
   - **Commit**: Run `git add Cargo.toml Cargo.lock && git commit -m "chore: bump version to X.Y.Z"`.
   - **Tag**: Run `git tag vX.Y.Z`.
   - **Push**: Run `git push && git push --tags`. If push fails (e.g. auth or network), report the error and tell the user to push manually; still continue to output the rest (changelog, release-draft prompt).
7. **Ask for confirmation only once**: After the steps above, ask the user: "Should a GitHub release draft be created?" If they say yes:
   - Run `gh --version`. If GitHub CLI is available, run `gh release create vX.Y.Z --draft --notes "..."` (use the changelog as the notes; use a temp file or `--notes-file` if escaping is awkward). The tag must exist remotely first (already done by the push step).
   - If GitHub CLI is not available, tell the user clearly that `gh` is not installed and they can create the release manually on GitHub using the copy-paste changelog.
8. **Copy-paste changelog**: Always output a **Copy-paste changelog** section—every time this command runs, regardless of release-draft choice or `gh` availability. Content must be markdown ready to paste into a GitHub release description (version heading + bullet list of changes). Use a fenced code block so the user can copy it in one go.
9. **Output**:
   - The last tag and the chosen bump reason (from args or inferred).
   - The new version number.
   - A short **Release notes** section (bullet list) summarizing changes since the last tag, based only on the diff and commit history. Do not invent items.
   - **Copy-paste changelog** (always): markdown block ready for GitHub release notes.
   - A short **Done** or **Next steps** summary: what was run (e.g. committed, tagged, pushed; release draft created or skipped with reason).
10. Do not run destructive git commands (no force push, no tag overwrite). Do not add any extra footer, signature, or attribution.

Output format:

## Release: X.Y.Z
- **Last tag:** ...
- **Bump:** patch | minor | major (reason: ...)

## Release notes
- ...

## Copy-paste changelog
Always include this section. Fenced markdown block so the user can copy it into a GitHub release description:

```markdown
## What's changed in vX.Y.Z
- ...
```

## Done
- Updated `Cargo.toml` and `Cargo.lock` (via `cargo build`/`cargo check`), committed, tagged `vX.Y.Z`, and pushed (or report any step that failed).
- Release draft: created with `gh`, or skipped (user said no), or not created because `gh` is not available—in that case tell the user to use the copy-paste changelog on GitHub.

Command args from user: `{{args}}`
