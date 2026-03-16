## MODIFIED Requirements

### Requirement: Helper output supports patch-oriented editing
The helper SHALL emit actionable scan results that help Codex produce minimal
git-based patches rather than forcing manual re-discovery of file targets.
Limited scan output SHALL treat each reported issue as an individual selectable
item, including split body-file paragraph issues, and SHALL preserve source
paragraph order within a file when paragraph targets are present.

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

#### Scenario: Limited output preserves body paragraph order
- **WHEN** Codex runs the helper with `--limit N` and the reported issues
  include split body-file paragraph targets
- **THEN** the helper counts each paragraph issue individually against the limit
- **THEN** the helper preserves source paragraph order within the body file so a
  review-sized batch maps directly to the earliest `main.<index>` targets
