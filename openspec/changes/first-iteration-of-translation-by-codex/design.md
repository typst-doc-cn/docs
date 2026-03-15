## Context

The repository already contains the pieces of a console-first translation
workflow: a local scan helper in `crates/translate`, contributor instructions
in `CONTRIBUTING.md`, machine-oriented translation rules in
`.github/copilot-instructions.md`, a repo-local Codex skill in
`.codex/skills/translate-typst-docs/SKILL.md`, and glossary terms in
`docs/terms.toml`. Today those pieces do not form a single documented loop for
either humans or Codex.

The explored repo state also showed two practical constraints that the new
guidance must acknowledge instead of hiding:
- `npm run validate` currently reports checked-in TOML parse issues and the
  validator script logs errors without enforcing a failing exit code.
- `gh` is installed, but pull request creation is only console-only when local
  GitHub CLI authentication is already configured and working.

This change is documentation-first. It should define a repeatable single
translation iteration without requiring new translation tooling.

## Goals / Non-Goals

**Goals:**
- Define one review-sized translation iteration that a human can execute from
  the console: scan, pick a target, edit, validate, review the patch, and open
  a PR.
- Add a shared `docs/translation-principle.md` document that both humans and
  Codex can treat as the canonical translation policy.
- Explain the role of `docs/terms.toml` as the canonical glossary and when a
  translator should update it.
- Align `.github/copilot-instructions.md` and the repo-local translation skill
  with the same shared principles and contributor workflow.
- Document current workflow prerequisites and limitations where the repository
  cannot yet guarantee a hard gate.

**Non-Goals:**
- Fix existing invalid TOML entries or change the validator implementation in
  this change.
- Redesign the glossary format in `docs/terms.toml`.
- Add new translation automation beyond the existing helper.
- Guarantee GitHub CLI authentication in every environment.

## Decisions

### Decision: Make `docs/translation-principle.md` the shared policy source

The new principles document will hold the rules that apply equally to human and
machine translators: preserve structure, prefer minimal patches, respect
inline-versus-included storage, use glossary-linked terminology when needed,
and avoid rewriting unrelated text.

Alternatives considered:
- Keep rules split between `CONTRIBUTING.md` and
  `.github/copilot-instructions.md`.
  Rejected because human and machine guidance would drift immediately.
- Expand `docs/terms.toml` comments into the policy document.
  Rejected because glossary entries and workflow principles have different
  purposes.

### Decision: Use `CONTRIBUTING.md` for the single-iteration workflow

`CONTRIBUTING.md` will describe the operational loop a contributor follows for
one translation batch. It will stay procedural and task-oriented, while
`docs/translation-principle.md` remains normative and reusable.

Alternatives considered:
- Put the full workflow into the principles document.
  Rejected because it would mix policy and operations, making both harder to
  audit.
- Put the workflow only in `.github/copilot-instructions.md`.
  Rejected because human contributors need a first-class entry point.

### Decision: Document current limitations explicitly

The contributor flow will describe `npm run validate` and `gh pr create`, but
it will also state the present caveats: validation output must be inspected
manually until the validator becomes stricter, and PR creation requires local
GitHub CLI auth.

Alternatives considered:
- Treat validation and PR creation as unconditional guarantees.
  Rejected because the explored repository state does not support that claim.
- Remove validation and PR creation from the documented loop.
  Rejected because they remain important parts of the intended workflow.

### Decision: Keep Codex guidance aligned by reference, not duplication

`.github/copilot-instructions.md` and
`.codex/skills/translate-typst-docs/SKILL.md` will be updated to read the new
principles document and follow the same contributor workflow terminology.

Alternatives considered:
- Copy the full principles into each machine-facing file.
  Rejected because duplicate rule text would drift.

## Risks / Trade-offs

- [Human and machine docs drift over time] -> Make
  `docs/translation-principle.md` the canonical policy source and link to it
  from both `CONTRIBUTING.md` and machine-facing guidance.
- [Contributors assume validation is a hard gate] -> Document the current
  validator limitation explicitly in the workflow steps.
- [Contributors assume PR creation always works from the console] -> State that
  `gh pr create` depends on local GitHub CLI authentication and repository
  access.
- [The workflow becomes too broad for review] -> Define a single iteration as a
  review-sized patch and instruct contributors to avoid sweeping backlog edits
  in one pass.
