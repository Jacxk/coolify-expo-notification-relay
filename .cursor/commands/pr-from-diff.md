---
description: Generate a PR title/body from git diff and optionally create it with GitHub CLI.
argument-hint: [base-branch or diff context, GitHub CLI or markdown]
---

Create a pull request title and body from branch changes.

Requirements:
1. If no diff context or base branch is provided in the command args, ask the user which branch to diff against.
2. Ask the user whether they want:
   - Markdown output to copy/paste, or
   - Direct PR creation via GitHub CLI (`gh pr create`).
3. If GitHub CLI is selected, run `gh --version` first:
   - If available, create the PR with generated title/body.
   - If unavailable, tell the user and provide markdown output instead.
4. Analyze the full diff scope and relevant commits (base branch...HEAD) before writing.
5. Do not invent behavior or tests not evidenced by diff/history.
6. Do not append any extra footer, signature, attribution, or marketing text (for example, "Made with Cursor").
7. Output only:
   - The PR title, and
   - The PR body sections defined below.
   No extra sections before or after.
8. The `Summary` must be a single paragraph (not bullets), describing what the changes do and why, with a maximum of 4 sentences.

Output format:

## Summary
...

## Additions
- ...

## Changes
- ...

## Fixes
- ...

## Breaking Changes
- None

Title rules:
- Keep the title concise and action-oriented.
- Prefer a natural prefix when appropriate (`feat:`, `fix:`, `chore:`, `refactor:`).

Command args from user: `{{args}}`
