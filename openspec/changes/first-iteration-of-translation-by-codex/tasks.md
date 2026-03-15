## 1. Shared Translation Policy

- [ ] 1.1 Add `docs/translation-principle.md` with the shared rules for human
  and machine translators, including structure preservation, minimal edits,
  helper-first target discovery, and inline versus included translation
  storage.
- [ ] 1.2 Document the `docs/terms.toml` concept inside the shared principles,
  including when translators should reuse an existing glossary entry and when
  they should add or refine one.

## 2. Contributor Workflow

- [ ] 2.1 Expand `CONTRIBUTING.md` with a single console-first translation
  iteration that covers scanning for work, choosing a review-sized target,
  locating the correct translation file, editing minimally, and reviewing the
  resulting patch.
- [ ] 2.2 Document workflow prerequisites and current caveats in
  `CONTRIBUTING.md`, including manual inspection of `npm run validate` output
  and the need for configured GitHub CLI authentication before `gh pr create`.

## 3. Codex Guidance Alignment

- [ ] 3.1 Update `.github/copilot-instructions.md` so Codex reads
  `docs/translation-principle.md` as the shared policy source for translation
  work.
- [ ] 3.2 Update `.codex/skills/translate-typst-docs/SKILL.md` so the skill
  follows the shared principles document and uses the same contributor workflow
  terminology as `CONTRIBUTING.md`.

## 4. Verification

- [ ] 4.1 Re-run the documented helper and validation commands referenced by the
  new docs to confirm the instructions match the current repository behavior.
- [ ] 4.2 Review the final documentation patch with `git diff` to confirm the
  human-facing docs and Codex-facing guidance point to the same shared policy
  without duplicating conflicting rules.
