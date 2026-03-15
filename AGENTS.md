# Agent Guide

This repository already keeps detailed translation guidance in
`.github/copilot-instructions.md`. Treat this file as the repo-local operating
guide for coding agents working in this repository.

## Primary References

- Read `.github/copilot-instructions.md` before translation or localization
  edits.
- Use `docs/terms.toml` for glossary-linked terminology and consistent Chinese
  wording.
- Use the repo-local skills under `.codex/skills/` when the task matches them,
  especially the OpenSpec workflow and `translate-typst-docs`.

## Rust Workflow

- Keep Rust edits focused and idiomatic for the existing workspace.
- Run `cargo fmt --all` after Rust code changes.
- Run `cargo clippy` for the Rust code you touched before handoff.
- Prefer the narrowest useful clippy command, for example
  `cargo clippy -p <crate> --all-targets -- -D warnings`.
- If a task spans multiple crates or shared code, run clippy for each affected
  crate or the full workspace when practical.

## Clippy Policy

- Treat clippy warnings in touched Rust crates as blocking for the current
  task, not as follow-up cleanup.
- Fix newly introduced warnings before responding to the user.
- If touched code already triggers clippy warnings, clean them up as part of
  the same task when the scope is reasonable.
- If you cannot run the relevant clippy command, or if a warning must be left
  in place for a documented reason, say so clearly in the final handoff.

## Handoff

- Summarize the validation you actually ran.
- For translation edits, keep the final review patch-oriented and easy to
  audit with `git diff`.
