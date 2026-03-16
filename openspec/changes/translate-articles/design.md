## Context

The repository already uses `crates/translate` to find missing and outdated
translation work across inline TOML entries and split body files. That helper
accepts `--limit`, which is the right mechanism for choosing a review-sized
translation batch, but the current issue ordering compares paragraph targets as
plain strings. In practice, that means `main.10` sorts before `main.2`, so a
limited article batch does not follow source paragraph order.

The translation backlog is dominated by missing `zh` body paragraphs in
article-like files under `locales/docs/typst-docs/`. We want one small, auditable
translation pass that uses the helper exactly the way the documented workflow
describes.

## Goals / Non-Goals

**Goals:**
- Make limited helper output line up with source paragraph order for split body
  files.
- Capture that behavior in the `translation-status-helper` spec delta.
- Translate one review-sized article batch chosen directly from helper output.

**Non-Goals:**
- Redesign the helper output schema or add new helper commands.
- Translate an entire article or the full body-file backlog in one patch.
- Archive the OpenSpec change in this implementation pass.

## Decisions

### Decision: Sort paragraph targets by numeric `main.<index>` order

Limited scan output should operate on the same unit the helper reports: one
issue per paragraph. For split body files, the order should therefore follow the
paragraph index instead of lexicographic string comparison.

Alternatives considered:
- Keep lexicographic ordering and manually choose paragraph batches.
  Rejected because it defeats the helper-first, review-sized workflow.
- Change the helper to limit by file instead of issue.
  Rejected because article batches need paragraph-level targeting.

### Decision: Use `--limit 10` as the initial article batch size

Ten paragraph issues are small enough to review comfortably while still making
visible progress in an article body file. After paragraph ordering is fixed,
`cargo run -p translate -- scan --limit 10` will select the earliest ten missing
paragraphs from the chosen article body.

Alternatives considered:
- Translate the whole article body file in one pass.
  Rejected because the patch would be too large to review comfortably.
- Start with a tiny reference details file instead of an article body.
  Rejected because this change is specifically about translating article
  sentences stored in split TOML body files.

### Decision: Translate the first batch from `guides.guide-for-latex-users.body.toml`

The LaTeX guide is already wired into the translated guides index and has fully
translated title and description metadata, but its body paragraphs are still
missing `zh`. Translating the first helper-selected batch makes the file usable
incrementally without broadening scope.

Alternatives considered:
- Start with `tutorial.writing-in-typst.body.toml`.
  Rejected for this pass because the helper’s default limited output currently
  points at the LaTeX guide first.

## Risks / Trade-offs

- [Helper output order changes from current snapshots] -> Update tests to pin
  the intended paragraph ordering and limited output behavior.
- [Article prose or Markdown structure gets damaged during translation] ->
  Restrict edits to the selected `[[main]]` entries and preserve Markdown,
  links, and code fences verbatim where required.
- [Ten paragraphs still feel too large for review] -> Keep the batch limited to
  the exact helper-selected paragraphs and stop after the first pass.
