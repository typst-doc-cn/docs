## ADDED Requirements

### Requirement: Helper detects untranslated translation targets
The repository SHALL provide a helper program in `crates/translate` that can
identify translation targets missing required Chinese content across both the
top-level TOML file and included body files.

#### Scenario: Missing top-level Chinese translation
- **WHEN** a key in `locales/docs/typst-docs.toml` has current English content
  but no usable `zh` translation
- **THEN** the helper reports that key as untranslated
- **THEN** the helper includes the file path and key needed to patch the entry

#### Scenario: Missing included body-file Chinese translation
- **WHEN** an included file under `locales/docs/typst-docs/` has a paragraph
  entry without usable `zh` content
- **THEN** the helper reports the included file path and paragraph target
- **THEN** the helper distinguishes that issue from a missing top-level entry

### Requirement: Helper detects outdated checked-in source text
The helper SHALL compare current documentation-derived English source text
against checked-in translation files and report entries whose stored English
content no longer matches the current source.

#### Scenario: Outdated inline entry
- **WHEN** the current English source for an inline TOML entry differs from the
  checked-in `en` text
- **THEN** the helper reports the entry as outdated even if a `zh` translation
  exists
- **THEN** the helper includes both the current source English and checked-in
  English needed for review

#### Scenario: Outdated split body file
- **WHEN** the current English source for a split body file paragraph differs
  from the checked-in paragraph or the paragraph structure no longer matches
- **THEN** the helper reports the included file as outdated
- **THEN** the helper identifies the paragraph target or structural mismatch
  that must be reviewed

### Requirement: Helper output supports patch-oriented editing
The helper SHALL emit actionable scan results that help Codex produce minimal
git-based patches rather than forcing manual re-discovery of file targets.

#### Scenario: Human-readable scan output
- **WHEN** Codex runs the helper in its default mode
- **THEN** the helper reports issue kinds grouped by the file or entry that must
  change
- **THEN** the helper includes enough location detail for Codex to prepare a
  minimal patch

#### Scenario: Structured scan output
- **WHEN** Codex or another tool needs machine-readable helper output
- **THEN** the helper can emit structured results that preserve issue type,
  file, key, and paragraph information
- **THEN** the structured output remains consistent with the human-readable scan
  results
