## Context

The repository already has strong translation conventions in
`.github/copilot-instructions.md`, a glossary in `docs/terms.toml`, and a TOML
layout that spans both `locales/docs/typst-docs.toml` and many included body
files under `locales/docs/typst-docs/`. Translation work is currently possible,
but Codex must rediscover the same repository-specific rules each time and does
not have a dedicated local helper for distinguishing missing translations from
translations that became stale after the English source changed.

This change spans two areas:

- a repo-local skill that teaches the translation workflow, helper usage, and
  patch-review expectations;
- a Rust helper CLI in `crates/translate` that scans TOML files and reports the
  work Codex should perform.

The repository also already contains `crates/typst-docs-l10n`, which generates
and consumes translation data. The new helper should complement that crate
rather than replace its responsibilities.

## Goals / Non-Goals

**Goals:**

- Give Codex a repeatable workflow for translating and reviewing Typst
  documentation TOML in this repository.
- Provide a local Rust helper that can identify missing `zh` entries, outdated
  checked-in English source text, and mismatches between the top-level TOML and
  included body files.
- Make patch-oriented review part of the default workflow so translation
  changes are easy to inspect with git diff.
- Keep the helper easy to improve when a new translation edge case is
  discovered.

**Non-Goals:**

- Automatically generating final Chinese translations without Codex review.
- Replacing the existing `typst-docs-l10n` generation and build workflow.
- Solving localization for languages other than the existing repository
  workflow.
- Building a general-purpose translation platform outside this repository.

## Decisions

### 1. Add a dedicated helper crate at `crates/translate`

The helper will live in its own workspace crate instead of being folded into
`typst-docs-l10n`.

Rationale:

- The helper serves a different operator task: scanning and triaging work,
  rather than generating localized documentation artifacts.
- A separate crate keeps the command surface small and makes the skill’s
  instructions easier to understand.
- It still allows code sharing with the existing localization crate where that
  reduces drift.

Alternatives considered:

- Add more subcommands to `typst-docs-l10n`: rejected because it would mix
  generation and review responsibilities into one CLI.
- Use ad hoc shell or Node scripts: rejected because the repo already has Rust
  workspace tooling and the helper needs stronger parsing guarantees.

### 2. Detect stale translations by comparing current source English to checked-in English

The helper will compare the current documentation-derived English content
against the checked-in `en` values in both `locales/docs/typst-docs.toml` and
included body files. If the checked-in English differs, the translation is
treated as outdated even when `zh` is present.

Rationale:

- The repository already stores source English beside the translation, so that
  text is the most direct freshness signal.
- This works for both inline entries and split body files.
- It avoids relying on git history or manual bookkeeping.

Alternatives considered:

- Use timestamps or commit SHAs: rejected because they add state without
  describing which entry changed.
- Only detect missing `zh`: rejected because it would miss stale translations
  after upstream documentation changes.

### 3. Make the skill helper-first and patch-first

The skill will instruct Codex to:

1. read the canonical repo instructions and glossary;
2. run the helper to find missing or stale work;
3. inspect the relevant TOML entry or included body file;
4. edit minimally;
5. review the result as a git-based patch.

Rationale:

- The helper reduces guesswork about where work is needed.
- The repository’s translation rules still require judgment, so the skill must
  layer human-readable workflow on top of the helper.
- Git diff review keeps the final output easy to audit.

Alternatives considered:

- A generic translation skill without a helper: rejected because it would not
  reliably locate outdated entries.
- A helper that writes translations directly: rejected because translation
  quality and glossary decisions still require Codex reasoning.

### 4. Keep the helper report structured and actionable instead of auto-applying edits

The helper should emit file paths, keys, paragraph indices when applicable,
issue kinds, and review hints that make it straightforward to produce a patch.
It should not auto-translate or auto-apply patches.

Rationale:

- Translation output still needs contextual judgment.
- Structured reports support both human-readable review and future automation.
- Avoiding automatic writes keeps the tool safe for exploratory and review work.

Alternatives considered:

- Generate placeholder patches automatically: rejected because placeholders are
  likely to be low quality and harder to trust.
- Only print raw counts: rejected because Codex also needs precise targets for
  editing.

## Risks / Trade-offs

- Helper logic could drift from the repository’s real translation layout ->
  Mitigation: reuse existing localization logic where practical and add tests
  around split-file behavior.
- Current-source comparison could be slow if it regenerates documentation each
  run -> Mitigation: scope the helper to status scanning and allow future
  optimization or snapshot-based modes if needed.
- The skill could become stale if it copies too much repo guidance ->
  Mitigation: keep the skill procedural and point back to canonical repo files
  for detailed rules.
- Patch-first guidance may feel slower than direct edits -> Mitigation: keep the
  helper output file-scoped so the resulting diffs stay small and easy to
  review.

## Migration Plan

1. Add the new `crates/translate` workspace member and implement scanning
   commands for missing and stale translation work.
2. Add the repo-local skill under `.codex/skills/translate-typst-docs/` with
   instructions that reference the helper and canonical repository rules.
3. Validate the helper against representative translation files and confirm the
   skill’s described workflow matches the actual commands and outputs.
4. Use git diff based review as the standard handoff for translation changes.

Rollback is straightforward: remove the skill and helper crate if the workflow
proves unhelpful or too costly to maintain.

## Open Questions

- Should the helper offer both human-readable and JSON output in the first
  version, or only one format initially?
- How much comparison logic should be shared with `crates/typst-docs-l10n`
  versus duplicated in a focused helper crate?
- Should the first version include a dedicated command for inspecting a single
  key or paragraph in detail, or is a scan report enough?
