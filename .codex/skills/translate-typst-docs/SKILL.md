---
name: translate-typst-docs
description: Translate or review Typst documentation TOML in this repository using the local helper, canonical repo rules, and git-based patch review.
license: MIT
metadata:
  author: typst-doc-cn/docs
  version: "1.0"
---

Translate or review Typst documentation TOML in this repository.

Use this skill when the user asks to translate, review, or suggest changes for:
- `locales/docs/typst-docs.toml`
- files under `locales/docs/typst-docs/`
- glossary-linked terms in `docs/terms.toml`

## Workflow

1. Read the canonical repo guidance before editing:
   - `.github/copilot-instructions.md`
   - `docs/terms.toml`

2. Run the helper before manually hunting for work whenever it can answer the question:
   ```bash
   cargo run -p translate -- scan
   ```
   The helper caps output at 50 issues by default so the target list stays reviewable.
   Re-run with all results only when you truly need the full backlog:
   ```bash
   cargo run -p translate -- scan --limit 0
   ```
   Use structured output when you need machine-readable targets:
   ```bash
   cargo run -p translate -- scan --format json
   ```
   Treat the helper output as the primary source of files, keys, and `main.<index>` paragraph targets to patch.

3. Resolve where the translation actually lives before editing:
   - Inline entries are edited in `locales/docs/typst-docs.toml`.
   - Entries whose checked-in `en` value looks like `{{typst-docs/<key>.toml}}` are stored in an included body file under `locales/docs/typst-docs/`.
   - For split body files, patch the specific `[[main]]` paragraph reported by the helper.

4. Edit minimally and preserve repository conventions:
   - Keep TOML structure valid.
   - Preserve Markdown, Typst code, links, labels, and placeholders.
   - Follow the terminology and term-link conventions from `docs/terms.toml`.
   - Do not rewrite unrelated paragraphs when only one target needs work.

5. Improve the helper first if it cannot represent the case you need:
   - Update `crates/translate` so the workflow stays repeatable.
   - Add or adjust tests for the new case.
   - Rerun the helper before continuing translation work.

6. Finish with patch-based review:
   - Review applied edits with `git diff -- <files>`.
   - If the user wants a suggestion without applying it, present the change as a unified diff / git-patch-compatible patch instead of prose-only guidance.

## Quick Checks

- If you changed the helper, run:
  ```bash
  cargo test -p translate
  ```
- If you changed translation files, keep the final diff small and easy to audit.
