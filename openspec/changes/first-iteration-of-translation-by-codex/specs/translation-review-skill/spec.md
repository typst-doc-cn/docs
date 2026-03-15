## MODIFIED Requirements

### Requirement: Skill provides the repository-specific TOML translation workflow
The repository SHALL provide a repo-local skill that teaches Codex how to
translate and review Typst documentation TOML using the repository’s canonical
rules, shared translation principles, split body files, and glossary-linked
terminology.

#### Scenario: Translate a top-level TOML entry
- **WHEN** Codex is asked to translate or review an entry in
  `locales/docs/typst-docs.toml`
- **THEN** the skill guides Codex to read the repository translation
  instructions, shared principles document, and glossary before editing
- **THEN** the skill guides Codex to preserve TOML structure, Markdown, and
  link targets while updating the entry

#### Scenario: Translate an included body file entry
- **WHEN** Codex is asked to work on content stored in
  `locales/docs/typst-docs/*.toml`
- **THEN** the skill explains that the translation may be stored in an included
  body file instead of directly in the top-level TOML
- **THEN** the skill guides Codex to inspect and edit the included file with
  the same repository-specific translation rules

#### Scenario: Shared principles stay aligned with contributor workflow
- **WHEN** the repository defines translation policy in
  `docs/translation-principle.md`
- **THEN** the skill reads and follows that document as a canonical source of
  translation rules
- **THEN** the skill stays aligned with the contributor-facing workflow
  documented in `CONTRIBUTING.md`
