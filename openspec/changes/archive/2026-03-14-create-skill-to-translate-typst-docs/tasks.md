## 1. Helper foundation

- [x] 1.1 Create the `crates/translate` workspace crate and wire in the Rust
  dependencies needed to read translation TOML and compare it with current
  documentation-derived English source text.
- [x] 1.2 Implement the helper’s core comparison model for inline TOML entries,
  split body files, missing `zh` content, outdated `en` content, and structural
  mismatches.

## 2. Helper commands and reporting

- [x] 2.1 Implement a scan command that reports untranslated and outdated
  entries with file paths, keys, and paragraph targets that Codex can patch.
- [x] 2.2 Implement structured helper output that preserves issue type, file,
  key, and paragraph information for machine-readable workflows.
- [x] 2.3 Add representative tests for inline entries, included body files, and
  stale-source detection so helper behavior stays aligned with the repository’s
  translation layout.

## 3. Translation skill

- [x] 3.1 Create the repo-local skill under
  `.codex/skills/translate-typst-docs/` with concise trigger metadata and
  workflow instructions.
- [x] 3.2 Teach the skill to read the repository’s canonical translation rules,
  run the helper before editing, and improve `crates/translate` when the helper
  cannot represent a required translation case.
- [x] 3.3 Teach the skill to finish translation work with git-based patch review
  so suggested and applied changes are easy to audit.

## 4. Validation and handoff

- [x] 4.1 Validate the skill metadata and confirm the helper commands described
  by the skill match the implemented CLI.
- [x] 4.2 Run representative helper checks against the repository’s translation
  files and verify the reported targets produce small, reviewable git diffs
  during translation work.
