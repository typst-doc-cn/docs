## Why

The article body files under `locales/docs/typst-docs/` still contain a large
backlog of untranslated `[[main]]` paragraphs. The local helper already limits
scan output, but its current paragraph ordering makes review-sized article
batches awkward because `main.10` appears before `main.2`.

## What Changes

- Modify the `translation-status-helper` capability so limited scan output
  explicitly treats body-file paragraph issues as individual selectable items
  and preserves paragraph order within a file.
- Align the helper implementation and tests with that paragraph-ordered limited
  output behavior.
- Translate the first review-sized batch of article paragraphs selected by the
  helper from `locales/docs/typst-docs/guides.guide-for-latex-users.body.toml`.

## Capabilities

### New Capabilities

### Modified Capabilities
- `translation-status-helper`: Limited scan output must keep split body-file
  paragraph issues in source order so a paragraph limit maps cleanly to a
  review-sized translation batch.

## Impact

- `crates/translate` scan ordering and tests.
- OpenSpec delta for `translation-status-helper`.
- Article translation content in
  `locales/docs/typst-docs/guides.guide-for-latex-users.body.toml`.
