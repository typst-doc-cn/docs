## 1. Helper Ordering

- [x] 1.1 Update `crates/translate` so limited scan output keeps split body-file
  paragraph issues in source order rather than lexicographic paragraph-label
  order.
- [x] 1.2 Add or adjust tests for limited paragraph selection and keep the
  `translation-status-helper` spec delta aligned with the implemented behavior.

## 2. Article Translation Batch

- [x] 2.1 Use the limited helper output to select one review-sized batch of
  article paragraphs from `locales/docs/typst-docs/guides.guide-for-latex-users.body.toml`.
- [x] 2.2 Translate only the selected `[[main]]` entries, preserving Markdown,
  links, code blocks, and surrounding TOML structure.

## 3. Verification

- [x] 3.1 Run `cargo fmt --all`, `cargo test -p translate`, and
  `cargo clippy -p translate --all-targets -- -D warnings` for the touched Rust
  helper changes.
- [x] 3.2 Run `npm run validate` and inspect `git diff` for the helper, spec,
  and translation patches.
