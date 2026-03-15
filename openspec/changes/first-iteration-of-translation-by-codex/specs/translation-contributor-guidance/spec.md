## ADDED Requirements

### Requirement: Contributor guide defines a single console-first translation iteration
The repository SHALL document in `CONTRIBUTING.md` a review-sized translation
workflow that a contributor can perform from the console without relying on a
separate GUI-only process.

#### Scenario: Start one translation iteration
- **WHEN** a translator starts a new documentation translation pass
- **THEN** the guide instructs the translator to use the local scan helper to
  identify candidate targets before manually hunting through translation files
- **THEN** the guide defines one iteration as a review-sized patch rather than
  an unconstrained backlog sweep

#### Scenario: Edit the correct translation target
- **WHEN** the helper reports a top-level key or included body-file paragraph
- **THEN** the guide explains how to locate whether the translation lives in
  `locales/docs/typst-docs.toml` or under `locales/docs/typst-docs/`
- **THEN** the guide instructs the translator to preserve TOML structure,
  Markdown, links, placeholders, and unrelated surrounding content

#### Scenario: Finish review and PR preparation
- **WHEN** a translator completes the targeted edits
- **THEN** the guide instructs the translator to run local validation, inspect
  the validation output, and review the resulting patch with `git diff`
- **THEN** the guide explains that pull request creation may be completed with
  `gh pr create` when GitHub CLI authentication is already configured

### Requirement: Shared translation principles apply to human and machine workflows
The repository SHALL provide `docs/translation-principle.md` as a shared policy
document for both human contributors and machine translators.

#### Scenario: Shared principles document structure-preservation rules
- **WHEN** a translator consults the shared principles document
- **THEN** the document defines requirements to preserve TOML structure,
  Markdown, Typst code, links, labels, and placeholders
- **THEN** the document requires minimal, reviewable edits instead of unrelated
  rewrites

#### Scenario: Shared principles document explains translation storage
- **WHEN** a translator needs to understand where a translation is stored
- **THEN** the document explains the difference between inline entries and
  included body-file translations
- **THEN** the document aligns that explanation with the helper-first workflow
  used during translation

### Requirement: Shared translation principles document the glossary concept
The shared principles SHALL explain `docs/terms.toml` as the repository's
canonical glossary for recurring or ambiguous terminology.

#### Scenario: Translator encounters a recurring technical term
- **WHEN** a translator needs a canonical translation for a recurring or
  ambiguous term
- **THEN** the document explains how `docs/terms.toml` records the English
  term, preferred Chinese wording, and usage context
- **THEN** the document explains when the translator should add or update a
  glossary entry instead of improvising per-file wording

#### Scenario: Translator avoids over-linking common terms
- **WHEN** a translator applies glossary-linked terminology in prose
- **THEN** the document explains that not every common term requires an
  explicit glossary link
- **THEN** the document preserves readability by reserving glossary links for
  terms that benefit from canonical anchoring
