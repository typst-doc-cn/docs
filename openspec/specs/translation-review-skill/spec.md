# translation-review-skill Specification

## Purpose
This skill defines how Codex should translate and review Typst documentation TOML in this repository. It is used whenever Codex is asked to work on `locales/docs/typst-docs.toml` or related included body files, ensuring that translations follow the repo-specific workflow, use the helper tool to locate and update missing or outdated entries, and present all changes as git-compatible patches for review.
## Requirements
### Requirement: Skill provides the repository-specific TOML translation workflow
The repository SHALL provide a repo-local skill that teaches Codex how to
translate and review Typst documentation TOML using the repository’s canonical
rules, including split body files and glossary-linked terminology.

#### Scenario: Translate a top-level TOML entry
- **WHEN** Codex is asked to translate or review an entry in
  `locales/docs/typst-docs.toml`
- **THEN** the skill guides Codex to read the repository translation
  instructions and glossary before editing
- **THEN** the skill guides Codex to preserve TOML structure, Markdown, and
  link targets while updating the entry

#### Scenario: Translate an included body file entry
- **WHEN** Codex is asked to work on content stored in
  `locales/docs/typst-docs/*.toml`
- **THEN** the skill explains that the translation may be stored in an included
  body file instead of directly in the top-level TOML
- **THEN** the skill guides Codex to inspect and edit the included file with
  the same repository-specific translation rules

### Requirement: Skill uses the helper before manual translation work
The skill SHALL instruct Codex to use the local helper program to identify
untranslated or outdated translation work before making manual edits whenever
that helper can answer the question.

#### Scenario: Helper identifies missing work
- **WHEN** Codex needs to find which entries are still untranslated
- **THEN** the skill directs Codex to run the helper before manually searching
  through translation files
- **THEN** the skill treats the helper report as the primary source of targets
  for translation edits

#### Scenario: Helper needs improvement
- **WHEN** the helper cannot represent a repository translation case that Codex
  must handle
- **THEN** the skill directs Codex to inspect and improve the helper under
  `crates/translate`
- **THEN** the skill directs Codex to rerun the helper after the improvement
  before continuing translation work

### Requirement: Skill requires patch-based review output
The skill SHALL teach Codex to present or verify translation changes as
git-based patches instead of relying on unreviewed file rewrites.

#### Scenario: Translation edit is completed
- **WHEN** Codex finishes updating translation-related files
- **THEN** the skill directs Codex to review the changed files with git-based
  diff output
- **THEN** the skill treats the resulting patch as the review artifact for the
  translation change

#### Scenario: Codex is asked for a suggested change without applying it
- **WHEN** Codex is asked to suggest a translation change without finalizing the
  edit
- **THEN** the skill directs Codex to express the suggestion in a git-patch
  compatible form
- **THEN** the skill avoids presenting the change as an unstructured prose-only
  description

