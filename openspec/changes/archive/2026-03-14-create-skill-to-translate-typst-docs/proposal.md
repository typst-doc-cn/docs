## Why

Translating and reviewing Typst documentation in this repository is more than
just filling in `zh` strings: it depends on TOML structure, included body
files, term-link conventions, and issue-specific review constraints. Codex can
do the work, but it currently has to rediscover the workflow and lacks a
reliable local helper for finding missing or stale translations before editing.

## What Changes

- Add a repo-local skill that teaches Codex how to translate and review Typst
  documentation TOML in this repository.
- Add a Rust helper program in `crates/translate` that scans localization files
  and reports untranslated or outdated entries in a structured, patch-oriented
  way.
- Define a workflow where Codex uses the helper first, then updates TOML with
  minimal edits, and reviews or presents the result as git-based patches.
- Teach Codex when to improve the helper itself before continuing translation
  work, so repeated gaps are captured in tooling instead of rediscovered by
  hand.

## Capabilities

### New Capabilities
- `translation-review-skill`: A repo-local skill for translating and reviewing
  Typst documentation TOML using repository-specific conventions, glossary
  rules, and the local helper program.
- `translation-status-helper`: A Rust CLI that detects untranslated entries,
  outdated English source text, and file-level mismatches across
  `locales/docs/typst-docs.toml` and included body files.

### Modified Capabilities
- None.

## Impact

- New skill files under `.codex/skills/translate-typst-docs/`.
- New Rust helper crate under `crates/translate/` and workspace integration.
- Translation workflow centered on `locales/docs/typst-docs.toml`,
  `locales/docs/typst-docs/*.toml`, `docs/terms.toml`, and git diff output for
  review.
