# Translation Principles

This document is the shared translation policy for human contributors and
machine translators working in this repository. Use it together with
`CONTRIBUTING.md` for the console-first workflow and `docs/terms.toml` for
canonical terminology.

## Keep each iteration review-sized

- Start with the helper instead of manually hunting through the translation
  files:

  ```bash
  cargo run -p translate -- scan
  ```

- Treat one translation iteration as a review-sized patch. Prefer one entry,
  one included body file, or one small cluster of closely related keys over a
  broad backlog sweep.
- Re-run the helper with `--limit 0` only when you need the full backlog, and
  use `--format json` when structured output is easier to work with.

## Edit the correct storage location

- Inline translations live directly in `locales/docs/typst-docs.toml`.
- When a top-level `en` value looks like `{{typst-docs/<file>.toml}}`, the
  translation is stored in `locales/docs/typst-docs/<file>.toml`.
- Split body files store paragraphs in repeated `[[main]]` tables. When the
  helper reports a paragraph target such as `main.3`, edit only that paragraph
  entry unless the task explicitly requires a broader sync.

## Preserve structure and source markers

- Keep TOML tables, keys, repeated `[[main]]` blocks, and surrounding structure
  intact.
- Preserve Markdown, Typst code, code fences, links, labels, anchors,
  placeholders, and line breaks that are part of the stored source.
- Do not translate link targets, section keys, placeholders, or checked-in
  English source text unless the task is explicitly about source synchronization.

## Prefer minimal, auditable edits

- Change only the targeted entry or paragraph and the minimum nearby content
  needed to keep the translation natural and correct.
- Do not rewrite unrelated neighboring paragraphs for style alone.
- Keep patches easy to review with `git diff`.

## Use the glossary deliberately

- `docs/terms.toml` is the repository's canonical glossary for recurring or
  ambiguous terminology.
- Reuse an existing glossary entry when its English term, preferred Chinese
  wording, and description match the current context.
- Add a new glossary entry or refine an existing one when a recurring term lacks
  a stable translation, the current description is too vague, or the repository
  needs a clearer context note to keep wording consistent.
- Reserve explicit glossary-linked wording for terms that benefit from canonical
  anchoring. Common terms such as `function` and `integer` usually do not need a
  glossary link in running prose.

## Keep translations natural and technically accurate

- Preserve the original meaning, constraints, and technical relationships.
- Prefer idiomatic Chinese over literal translation when both convey the same
  meaning.
- Keep terminology consistent across related entries, but adjust phrasing when
  the surrounding grammar requires it.
