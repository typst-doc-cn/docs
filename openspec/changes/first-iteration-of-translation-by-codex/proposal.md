## Why

The repository now has enough local tooling to support a console-first
translation workflow, but that workflow is only implied across the helper,
skill, and scattered instructions. Human translators still lack a clear
single-iteration guide for scanning, editing, reviewing, and opening a pull
request, and Codex does not yet have a shared principles document to follow
alongside humans.

## What Changes

- Add contributor-facing guidance for one translation iteration that can be run
  entirely from the console: scan for work, edit the target entry, review the
  patch, and open a pull request.
- Add a shared `docs/translation-principle.md` document that defines the
  translation rules both human and machine translators should follow.
- Document the role of `docs/terms.toml` as the repository's canonical glossary
  for recurring or ambiguous terminology.
- Update Codex-facing translation guidance so the existing translation skill
  reads and follows the shared principles document instead of relying only on
  scattered repo-local instructions.
- Clarify current workflow prerequisites and limitations for console-only work,
  including local validation behavior and GitHub CLI requirements for PR
  creation.

## Capabilities

### New Capabilities
- `translation-contributor-guidance`: Contributor documentation for a
  review-sized translation iteration, shared translation principles, and
  glossary usage across human and machine workflows.

### Modified Capabilities
- `translation-review-skill`: The Codex translation skill must read the shared
  translation principles document and stay aligned with the documented
  contributor workflow.

## Impact

- Contributor docs in `CONTRIBUTING.md`.
- New shared guidance in `docs/translation-principle.md`.
- Terminology references in `docs/terms.toml`.
- Codex-facing translation guidance in `.github/copilot-instructions.md` and
  `.codex/skills/translate-typst-docs/SKILL.md`.
