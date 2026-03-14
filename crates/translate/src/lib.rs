//! Translation status scanning for Typst documentation files.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tinymist_l10n::{deserialize, TranslationMap};
use typst::layout::PagedDocument;
use typst_docs::{provide, Html, Resolver};
use typst_docs_l10n::{markdown::split_markdown, translate::check_page, PageMdModel};

/// Default base URL used while deriving current English source entries.
const DEFAULT_BASE: &str = "/";
/// Default maximum number of issues shown by the CLI.
pub const DEFAULT_ISSUE_LIMIT: usize = 50;
/// Directory name used inside top-level `{{...}}` body-file markers.
const INCLUDED_DIR_NAME: &str = "typst-docs";
/// Separator used between Markdown paragraphs in translation files.
const MARKDOWN_PAR_SEP: &str = "\n\n";

/// Paths used by the translation scanner.
#[derive(Debug, Clone)]
pub struct RepoPaths {
    /// Path to the top-level translation TOML file.
    pub translations: PathBuf,
    /// Path to the directory that stores split body-file translations.
    pub included_dir: PathBuf,
    /// Base URL used when collecting the current documentation source text.
    pub base: String,
}

impl RepoPaths {
    /// Creates scanner paths and normalizes the base URL.
    pub fn new(
        translations: impl Into<PathBuf>,
        included_dir: impl Into<PathBuf>,
        base: impl Into<String>,
    ) -> Self {
        let mut base = base.into();
        if !base.ends_with('/') {
            base.push('/');
        }

        Self {
            translations: translations.into(),
            included_dir: included_dir.into(),
            base,
        }
    }

    /// Returns the root directory that contains the translation files.
    fn translation_root(&self) -> PathBuf {
        self.translations
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf()
    }
}

impl Default for RepoPaths {
    fn default() -> Self {
        Self::new(
            PathBuf::from("locales/docs/typst-docs.toml"),
            PathBuf::from("locales/docs/typst-docs"),
            DEFAULT_BASE,
        )
    }
}

/// Structured translation scan output.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ScanReport {
    /// The number of source entries scanned from the current documentation.
    pub scanned_entries: usize,
    /// The total number of issues discovered by the scan before output
    /// limiting.
    pub issue_count: usize,
    /// The number of issues included in this report payload.
    pub displayed_issue_count: usize,
    /// The number of issues omitted from this report payload.
    pub omitted_issue_count: usize,
    /// The issues discovered by the scan.
    pub issues: Vec<Issue>,
}

/// A single missing, stale, or mismatched translation target.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Issue {
    /// The issue type.
    pub kind: IssueKind,
    /// The file that should be patched or reviewed.
    pub file: String,
    /// The translation key associated with the issue.
    pub key: String,
    /// The paragraph target for split body files, for example `main.3`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph: Option<String>,
    /// Additional review context for structural mismatches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// The checked-in English content that is currently on disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_in_en: Option<String>,
    /// The current English source text derived from the documentation model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_source_en: Option<String>,
}

/// The supported issue types.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum IssueKind {
    /// A top-level entry is missing usable Chinese content.
    MissingZhInline,
    /// A split body-file paragraph is missing usable Chinese content.
    MissingZhBody,
    /// A top-level entry's stored English text no longer matches the source.
    OutdatedEnInline,
    /// A split body-file paragraph's stored English text no longer matches the
    /// source.
    OutdatedEnBody,
    /// The checked-in translation structure no longer matches the current
    /// source layout.
    StructuralMismatch,
}

impl IssueKind {
    /// Returns the stable machine-readable label for this issue kind.
    fn label(self) -> &'static str {
        match self {
            Self::MissingZhInline => "missing_zh_inline",
            Self::MissingZhBody => "missing_zh_body",
            Self::OutdatedEnInline => "outdated_en_inline",
            Self::OutdatedEnBody => "outdated_en_body",
            Self::StructuralMismatch => "structural_mismatch",
        }
    }
}

/// Scans the repository translation files and returns a structured report.
pub fn scan_repo(paths: &RepoPaths) -> Result<ScanReport> {
    let source_entries = collect_source_entries(&paths.base)?;
    let repo = TranslationRepo::load(paths)?;
    let issues = compare_source_entries(&source_entries, &repo, paths);

    Ok(ScanReport {
        scanned_entries: source_entries.len(),
        issue_count: issues.len(),
        displayed_issue_count: issues.len(),
        omitted_issue_count: 0,
        issues,
    })
}

impl ScanReport {
    /// Limits the number of issues carried in the report payload.
    pub fn with_issue_limit(mut self, limit: Option<usize>) -> Self {
        let Some(limit) = limit else {
            return self;
        };

        if self.issues.len() <= limit {
            self.displayed_issue_count = self.issues.len();
            self.omitted_issue_count = 0;
            return self;
        }

        self.issues.truncate(limit);
        self.displayed_issue_count = self.issues.len();
        self.omitted_issue_count = self.issue_count.saturating_sub(self.displayed_issue_count);
        self
    }
}

/// Renders a human-readable scan report.
pub fn render_text(report: &ScanReport) -> String {
    if report.issues.is_empty() {
        let mut rendered = format!(
            "No translation issues found across {} current documentation entries.",
            report.scanned_entries
        );
        if report.omitted_issue_count > 0 {
            rendered.push_str(&format!(
                "\nShowing 0 of {} issue(s); {} omitted.",
                report.issue_count, report.omitted_issue_count
            ));
        }
        return rendered;
    }

    let mut rendered = format!(
        "Found {} translation issue(s) across {} current documentation entries.\n",
        report.issue_count, report.scanned_entries
    );
    if report.omitted_issue_count > 0 {
        rendered.push_str(&format!(
            "Showing first {} issue(s); {} omitted. Re-run with `--limit 0` to show all results.\n",
            report.displayed_issue_count, report.omitted_issue_count
        ));
    }

    let mut current_file = None::<&str>;
    for issue in &report.issues {
        if current_file != Some(issue.file.as_str()) {
            if current_file.is_some() {
                rendered.push('\n');
            }
            current_file = Some(issue.file.as_str());
            rendered.push_str(&format!("\n{}\n", issue.file));
        }

        rendered.push_str(&format!("  - {} key={}", issue.kind.label(), issue.key));
        if let Some(paragraph) = &issue.paragraph {
            rendered.push_str(&format!(" paragraph={paragraph}"));
        }
        if let Some(detail) = &issue.detail {
            rendered.push_str(&format!(" ({detail})"));
        }
        rendered.push('\n');

        if let Some(checked_in_en) = &issue.checked_in_en {
            rendered.push_str(&format!(
                "    checked-in en: {}\n",
                preview_text(checked_in_en)
            ));
        }
        if let Some(current_source_en) = &issue.current_source_en {
            rendered.push_str(&format!(
                "    current source: {}\n",
                preview_text(current_source_en)
            ));
        }
    }

    rendered
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A current English source entry keyed by its translation path.
struct SourceEntry {
    /// Translation key such as `tutorial.body` or `index.title`.
    key: String,
    /// Source payload, either inline text or a split body-file view.
    content: SourceContent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Current English source content grouped by how it is stored on disk.
enum SourceContent {
    /// A source entry stored directly in the top-level translation TOML file.
    Inline {
        /// Current English source text for the inline entry.
        current_en: String,
    },
    /// A source entry stored in a split included body file.
    Split {
        /// Current English source text before it is split into paragraphs.
        current_en: String,
        /// Included body-file name derived from the translation key.
        file_name: String,
        /// Current English source text split into paragraph targets.
        paragraphs: Vec<String>,
    },
}

#[derive(Debug, Clone, Default)]
/// In-memory view of the checked-in translation repository layout.
struct TranslationRepo {
    /// Parsed top-level entries from `typst-docs.toml`.
    top_level: BTreeMap<String, BTreeMap<String, String>>,
    /// Parsed split body files keyed by file name.
    body_files: BTreeMap<String, BodyFile>,
}

#[derive(Debug, Clone, Default)]
/// Parsed contents of one split body translation file.
struct BodyFile {
    /// Paragraphs stored under repeated `[[main]]` items.
    paragraphs: Vec<BodyParagraph>,
}

#[derive(Debug, Clone, Default)]
/// Checked-in paragraph translations for one `[[main]]` item.
struct BodyParagraph {
    /// Stored English source text for the paragraph.
    en: Option<String>,
    /// Stored Chinese translation for the paragraph.
    zh: Option<String>,
}

#[derive(Debug, Deserialize)]
/// On-disk TOML layout for split body translation files.
struct LargeTranslationFile {
    #[serde(default)]
    /// Paragraph entries stored in repeated `[[main]]` tables.
    main: Vec<TranslationMap>,
}

impl TranslationRepo {
    /// Loads the checked-in translation repository from the configured paths.
    fn load(paths: &RepoPaths) -> Result<Self> {
        let top_level = load_top_level_translations(&paths.translations)?;
        let mut body_files = BTreeMap::new();

        if paths.included_dir.exists() {
            for entry in fs::read_dir(&paths.included_dir).with_context(|| {
                format!(
                    "Failed to read included translation directory: {}",
                    paths.included_dir.display()
                )
            })? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                    continue;
                }

                let file_name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .with_context(|| format!("Invalid UTF-8 body-file name: {}", path.display()))?
                    .to_owned();
                let body = load_body_file(&path)?;
                body_files.insert(file_name, body);
            }
        }

        Ok(Self {
            top_level,
            body_files,
        })
    }
}

/// Loads and normalizes top-level translation entries from `typst-docs.toml`.
fn load_top_level_translations(path: &Path) -> Result<BTreeMap<String, BTreeMap<String, String>>> {
    let input = fs::read_to_string(path)
        .with_context(|| format!("Failed to read translation file: {}", path.display()))?;
    let raw = deserialize(&input, true)
        .with_context(|| format!("Failed to parse translation file: {}", path.display()))?;

    let mut result = BTreeMap::new();
    for (key, languages) in raw {
        let mut normalized = BTreeMap::new();
        for (lang, value) in languages {
            normalized.insert(lang, decode_disk_value(&value)?);
        }
        result.insert(key, normalized);
    }
    Ok(result)
}

/// Loads one split body translation file from disk.
fn load_body_file(path: &Path) -> Result<BodyFile> {
    let input = fs::read_to_string(path)
        .with_context(|| format!("Failed to read body translation file: {}", path.display()))?;
    let parsed = toml::from_str::<LargeTranslationFile>(&input)
        .with_context(|| format!("Failed to parse body translation file: {}", path.display()))?;

    let paragraphs = parsed
        .main
        .into_iter()
        .map(|paragraph| BodyParagraph {
            en: paragraph.get("en").cloned(),
            zh: paragraph.get("zh").cloned(),
        })
        .collect();

    Ok(BodyFile { paragraphs })
}

/// Decodes a checked-in string value from the top-level translation TOML.
fn decode_disk_value(value: &str) -> Result<String> {
    if value.starts_with('"') {
        return serde_json::from_str::<String>(value)
            .with_context(|| format!("Failed to decode string value: {value}"));
    }

    Ok(value.to_owned())
}

/// Collects current English source entries from the generated Typst docs model.
fn collect_source_entries(base: &str) -> Result<Vec<SourceEntry>> {
    let resolver = ScanResolver { base };
    let pages = provide(&resolver)
        .into_iter()
        .map(PageMdModel::from)
        .collect::<Vec<_>>();

    let mut translations = vec![];
    for page in pages {
        check_page(page, &mut translations);
    }

    let mut entries = translations
        .into_iter()
        .map(|(key, current_en)| {
            if should_split(&current_en) {
                let file_name = format!("{key}.toml");
                let paragraphs = split_markdown(&current_en)
                    .into_iter()
                    .map(str::to_owned)
                    .collect();
                SourceEntry {
                    key,
                    content: SourceContent::Split {
                        current_en,
                        file_name,
                        paragraphs,
                    },
                }
            } else {
                SourceEntry {
                    key,
                    content: SourceContent::Inline { current_en },
                }
            }
        })
        .collect::<Vec<_>>();

    entries.sort_by(|left, right| left.key.cmp(&right.key));
    Ok(entries)
}

/// Compares current English source entries against checked-in translations.
fn compare_source_entries(
    source_entries: &[SourceEntry],
    repo: &TranslationRepo,
    paths: &RepoPaths,
) -> Vec<Issue> {
    let top_level_file = paths.translations.display().to_string();
    let translation_root = paths.translation_root();
    let mut issues = vec![];
    let mut expected_body_files = BTreeSet::new();

    for source in source_entries {
        match &source.content {
            SourceContent::Inline { current_en } => {
                compare_inline_entry(
                    &mut issues,
                    repo.top_level.get(&source.key),
                    &source.key,
                    current_en,
                    &top_level_file,
                );
            }
            SourceContent::Split {
                current_en,
                file_name,
                paragraphs,
            } => {
                expected_body_files.insert(file_name.clone());
                compare_split_entry(
                    &mut issues,
                    repo,
                    paths,
                    &source.key,
                    current_en,
                    file_name,
                    paragraphs,
                    &top_level_file,
                    &translation_root,
                );
            }
        }
    }

    for file_name in repo.body_files.keys() {
        if expected_body_files.contains(file_name) {
            continue;
        }

        let file = paths.included_dir.join(file_name).display().to_string();
        issues.push(Issue {
            kind: IssueKind::StructuralMismatch,
            file,
            key: file_name.trim_end_matches(".toml").to_owned(),
            paragraph: None,
            detail: Some("Included body file has no matching current source entry".to_owned()),
            checked_in_en: None,
            current_source_en: None,
        });
    }

    issues.sort_by(|left, right| {
        (
            left.file.as_str(),
            left.key.as_str(),
            left.paragraph.as_deref(),
            left.kind,
        )
            .cmp(&(
                right.file.as_str(),
                right.key.as_str(),
                right.paragraph.as_deref(),
                right.kind,
            ))
    });

    issues
}

/// Compares one inline top-level translation entry.
fn compare_inline_entry(
    issues: &mut Vec<Issue>,
    entry: Option<&BTreeMap<String, String>>,
    key: &str,
    current_en: &str,
    top_level_file: &str,
) {
    let Some(entry) = entry else {
        issues.push(Issue {
            kind: IssueKind::StructuralMismatch,
            file: top_level_file.to_owned(),
            key: key.to_owned(),
            paragraph: None,
            detail: Some("Top-level translation entry is missing".to_owned()),
            checked_in_en: None,
            current_source_en: Some(current_en.to_owned()),
        });
        return;
    };

    match entry.get("en") {
        Some(checked_in_en) if is_body_marker(checked_in_en) => {
            issues.push(Issue {
                kind: IssueKind::StructuralMismatch,
                file: top_level_file.to_owned(),
                key: key.to_owned(),
                paragraph: None,
                detail: Some(
                    "Current source is inline, but the checked-in entry points to a body file"
                        .to_owned(),
                ),
                checked_in_en: Some(checked_in_en.clone()),
                current_source_en: Some(current_en.to_owned()),
            });
        }
        Some(checked_in_en) if checked_in_en != current_en => {
            issues.push(Issue {
                kind: IssueKind::OutdatedEnInline,
                file: top_level_file.to_owned(),
                key: key.to_owned(),
                paragraph: None,
                detail: None,
                checked_in_en: Some(checked_in_en.clone()),
                current_source_en: Some(current_en.to_owned()),
            });
        }
        Some(_) => {}
        None => {
            issues.push(Issue {
                kind: IssueKind::StructuralMismatch,
                file: top_level_file.to_owned(),
                key: key.to_owned(),
                paragraph: None,
                detail: Some("Top-level entry is missing stored English content".to_owned()),
                checked_in_en: None,
                current_source_en: Some(current_en.to_owned()),
            });
        }
    }

    if !has_usable_translation(entry.get("zh")) {
        issues.push(Issue {
            kind: IssueKind::MissingZhInline,
            file: top_level_file.to_owned(),
            key: key.to_owned(),
            paragraph: None,
            detail: None,
            checked_in_en: None,
            current_source_en: None,
        });
    }
}

#[allow(clippy::too_many_arguments)]
/// Compares one split body-file translation entry and its paragraph targets.
fn compare_split_entry(
    issues: &mut Vec<Issue>,
    repo: &TranslationRepo,
    paths: &RepoPaths,
    key: &str,
    current_en: &str,
    file_name: &str,
    current_paragraphs: &[String],
    top_level_file: &str,
    translation_root: &Path,
) {
    let marker = body_marker(file_name);
    let body_file = paths.included_dir.join(file_name).display().to_string();
    let body_reference = translation_root
        .join(INCLUDED_DIR_NAME)
        .join(file_name)
        .display()
        .to_string();

    let Some(entry) = repo.top_level.get(key) else {
        issues.push(Issue {
            kind: IssueKind::StructuralMismatch,
            file: top_level_file.to_owned(),
            key: key.to_owned(),
            paragraph: None,
            detail: Some(format!(
                "Top-level translation entry is missing; expected body-file marker {marker}"
            )),
            checked_in_en: None,
            current_source_en: Some(current_en.to_owned()),
        });
        return;
    };

    match entry.get("en") {
        Some(checked_in_en) if checked_in_en == &marker => {}
        Some(checked_in_en) => {
            issues.push(Issue {
                kind: IssueKind::StructuralMismatch,
                file: top_level_file.to_owned(),
                key: key.to_owned(),
                paragraph: None,
                detail: Some(format!(
                    "Expected body-file marker {marker}, but found a different stored English value"
                )),
                checked_in_en: Some(checked_in_en.clone()),
                current_source_en: Some(current_en.to_owned()),
            });
            return;
        }
        None => {
            issues.push(Issue {
                kind: IssueKind::StructuralMismatch,
                file: top_level_file.to_owned(),
                key: key.to_owned(),
                paragraph: None,
                detail: Some(format!(
                    "Top-level entry is missing stored English content; expected body-file marker {marker}"
                )),
                checked_in_en: None,
                current_source_en: Some(current_en.to_owned()),
            });
            return;
        }
    }

    let Some(body) = repo.body_files.get(file_name) else {
        issues.push(Issue {
            kind: IssueKind::StructuralMismatch,
            file: body_file,
            key: key.to_owned(),
            paragraph: None,
            detail: Some(format!(
                "Missing included body file referenced by {body_reference}"
            )),
            checked_in_en: None,
            current_source_en: Some(current_en.to_owned()),
        });
        return;
    };

    if body.paragraphs.len() != current_paragraphs.len() {
        issues.push(Issue {
            kind: IssueKind::StructuralMismatch,
            file: paths.included_dir.join(file_name).display().to_string(),
            key: key.to_owned(),
            paragraph: None,
            detail: Some(format!(
                "Expected {} paragraph(s), found {} paragraph(s)",
                current_paragraphs.len(),
                body.paragraphs.len()
            )),
            checked_in_en: None,
            current_source_en: None,
        });
    }

    for (index, current_paragraph) in current_paragraphs.iter().enumerate() {
        let paragraph = format!("main.{index}");
        let Some(stored) = body.paragraphs.get(index) else {
            continue;
        };

        match stored.en.as_deref() {
            Some(checked_in_en) if checked_in_en != current_paragraph => {
                issues.push(Issue {
                    kind: IssueKind::OutdatedEnBody,
                    file: paths.included_dir.join(file_name).display().to_string(),
                    key: key.to_owned(),
                    paragraph: Some(paragraph.clone()),
                    detail: None,
                    checked_in_en: Some(checked_in_en.to_owned()),
                    current_source_en: Some(current_paragraph.clone()),
                });
            }
            Some(_) => {}
            None => {
                issues.push(Issue {
                    kind: IssueKind::StructuralMismatch,
                    file: paths.included_dir.join(file_name).display().to_string(),
                    key: key.to_owned(),
                    paragraph: Some(paragraph.clone()),
                    detail: Some("Paragraph is missing stored English content".to_owned()),
                    checked_in_en: None,
                    current_source_en: Some(current_paragraph.clone()),
                });
            }
        }

        if !has_usable_translation(stored.zh.as_ref()) {
            issues.push(Issue {
                kind: IssueKind::MissingZhBody,
                file: paths.included_dir.join(file_name).display().to_string(),
                key: key.to_owned(),
                paragraph: Some(paragraph),
                detail: None,
                checked_in_en: None,
                current_source_en: None,
            });
        }
    }
}

/// Builds the top-level body-file marker for a split translation entry.
fn body_marker(file_name: &str) -> String {
    format!("{{{{{INCLUDED_DIR_NAME}/{file_name}}}}}")
}

/// Returns whether a checked-in translation value is present and non-empty.
fn has_usable_translation(value: Option<&String>) -> bool {
    value.is_some_and(|value| !value.trim().is_empty())
}

/// Returns whether a stored English value points to an included body file.
fn is_body_marker(value: &str) -> bool {
    value.starts_with("{{") && value.ends_with("}}")
}

/// Returns whether a source Markdown entry should live in a split body file.
fn should_split(markdown: &str) -> bool {
    markdown.matches(MARKDOWN_PAR_SEP).take(5).count() >= 5
}

/// Produces a short single-line preview for human-readable reports.
fn preview_text(text: &str) -> String {
    const LIMIT: usize = 120;

    let compact = text.replace('\n', "\\n");
    if compact.chars().count() <= LIMIT {
        return compact;
    }

    let mut preview = compact.chars().take(LIMIT).collect::<String>();
    preview.push_str("...");
    preview
}

#[derive(Debug)]
/// Resolver used to collect current English source entries without writing
/// assets.
struct ScanResolver<'a> {
    /// Base URL used while constructing synthetic asset links.
    base: &'a str,
}

impl Resolver for ScanResolver<'_> {
    fn commits(&self, _from: &str, _to: &str) -> Vec<typst_docs::Commit> {
        vec![]
    }

    fn example(
        &self,
        hash: u128,
        source: Option<Html>,
        _document: &PagedDocument,
    ) -> typst_docs::Html {
        let src = format!("{}assets/{hash:x}.png", self.base);

        if let Some(code) = source {
            let code_safe = code.as_str();
            Html::new(format!(
                r#"<div class="previewed-code"><pre>{code_safe}</pre><div class="preview"><img src="{src}" alt="Preview"></div></div>"#
            ))
        } else {
            Html::new(format!(
                r#"<div class="preview"><img src="{src}" alt="Preview"></div>"#
            ))
        }
    }

    fn image(&self, filename: &str, _data: &[u8]) -> String {
        format!("{}assets/{filename}", self.base)
    }

    fn link(&self, _link: &str) -> Option<String> {
        None
    }

    fn base(&self) -> &str {
        self.base
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn compares_inline_entries() {
        let source_entries = vec![
            SourceEntry {
                key: "index.title".to_owned(),
                content: SourceContent::Inline {
                    current_en: "Overview".to_owned(),
                },
            },
            SourceEntry {
                key: "guides.title".to_owned(),
                content: SourceContent::Inline {
                    current_en: "Guides".to_owned(),
                },
            },
        ];

        let mut repo = TranslationRepo::default();
        repo.top_level.insert(
            "index.title".to_owned(),
            BTreeMap::from([
                ("en".to_owned(), "Overview (old)".to_owned()),
                ("zh".to_owned(), "概述".to_owned()),
            ]),
        );
        repo.top_level.insert(
            "guides.title".to_owned(),
            BTreeMap::from([("en".to_owned(), "Guides".to_owned())]),
        );

        let issues = compare_source_entries(&source_entries, &repo, &RepoPaths::default());
        assert_snapshot!(serde_json::to_string_pretty(&issues).unwrap(), @r###"
        [
          {
            "kind": "missing_zh_inline",
            "file": "locales/docs/typst-docs.toml",
            "key": "guides.title"
          },
          {
            "kind": "outdated_en_inline",
            "file": "locales/docs/typst-docs.toml",
            "key": "index.title",
            "checked_in_en": "Overview (old)",
            "current_source_en": "Overview"
          }
        ]
        "###);
    }

    #[test]
    fn compares_split_body_files() {
        let source_entries = vec![SourceEntry {
            key: "tutorial.body".to_owned(),
            content: SourceContent::Split {
                current_en: "first\n\nsecond\n\nthird\n\nfourth\n\nfifth\n\nsixth".to_owned(),
                file_name: "tutorial.body.toml".to_owned(),
                paragraphs: vec![
                    "first".to_owned(),
                    "second".to_owned(),
                    "third".to_owned(),
                    "fourth".to_owned(),
                    "fifth".to_owned(),
                    "sixth".to_owned(),
                ],
            },
        }];

        let mut repo = TranslationRepo::default();
        repo.top_level.insert(
            "tutorial.body".to_owned(),
            BTreeMap::from([(
                "en".to_owned(),
                "{{typst-docs/tutorial.body.toml}}".to_owned(),
            )]),
        );
        repo.body_files.insert(
            "tutorial.body.toml".to_owned(),
            BodyFile {
                paragraphs: vec![
                    BodyParagraph {
                        en: Some("first".to_owned()),
                        zh: Some("第一".to_owned()),
                    },
                    BodyParagraph {
                        en: Some("second (old)".to_owned()),
                        zh: Some("第二".to_owned()),
                    },
                    BodyParagraph {
                        en: Some("third".to_owned()),
                        zh: None,
                    },
                    BodyParagraph {
                        en: Some("fourth".to_owned()),
                        zh: Some("第四".to_owned()),
                    },
                ],
            },
        );

        let issues = compare_source_entries(&source_entries, &repo, &RepoPaths::default());
        assert_snapshot!(serde_json::to_string_pretty(&issues).unwrap(), @r###"
        [
          {
            "kind": "structural_mismatch",
            "file": "locales/docs/typst-docs/tutorial.body.toml",
            "key": "tutorial.body",
            "detail": "Expected 6 paragraph(s), found 4 paragraph(s)"
          },
          {
            "kind": "outdated_en_body",
            "file": "locales/docs/typst-docs/tutorial.body.toml",
            "key": "tutorial.body",
            "paragraph": "main.1",
            "checked_in_en": "second (old)",
            "current_source_en": "second"
          },
          {
            "kind": "missing_zh_body",
            "file": "locales/docs/typst-docs/tutorial.body.toml",
            "key": "tutorial.body",
            "paragraph": "main.2"
          }
        ]
        "###);
    }

    #[test]
    fn detects_structure_mismatches_for_split_entries() {
        let source_entries = vec![SourceEntry {
            key: "reference.body".to_owned(),
            content: SourceContent::Split {
                current_en: "a\n\nb\n\nc\n\nd\n\ne\n\nf".to_owned(),
                file_name: "reference.body.toml".to_owned(),
                paragraphs: vec![
                    "a".to_owned(),
                    "b".to_owned(),
                    "c".to_owned(),
                    "d".to_owned(),
                    "e".to_owned(),
                    "f".to_owned(),
                ],
            },
        }];

        let mut repo = TranslationRepo::default();
        repo.top_level.insert(
            "reference.body".to_owned(),
            BTreeMap::from([("en".to_owned(), "Reference inline body".to_owned())]),
        );
        repo.body_files.insert(
            "orphan.body.toml".to_owned(),
            BodyFile {
                paragraphs: vec![BodyParagraph {
                    en: Some("orphan".to_owned()),
                    zh: Some("孤儿".to_owned()),
                }],
            },
        );

        let issues = compare_source_entries(&source_entries, &repo, &RepoPaths::default());
        assert_snapshot!(serde_json::to_string_pretty(&issues).unwrap(), @r###"
        [
          {
            "kind": "structural_mismatch",
            "file": "locales/docs/typst-docs.toml",
            "key": "reference.body",
            "detail": "Expected body-file marker {{typst-docs/reference.body.toml}}, but found a different stored English value",
            "checked_in_en": "Reference inline body",
            "current_source_en": "a\n\nb\n\nc\n\nd\n\ne\n\nf"
          },
          {
            "kind": "structural_mismatch",
            "file": "locales/docs/typst-docs/orphan.body.toml",
            "key": "orphan.body",
            "detail": "Included body file has no matching current source entry"
          }
        ]
        "###);
    }

    #[test]
    fn limits_report_payload_but_keeps_total_counts() {
        let report = ScanReport {
            scanned_entries: 4,
            issue_count: 3,
            displayed_issue_count: 3,
            omitted_issue_count: 0,
            issues: vec![
                Issue {
                    kind: IssueKind::MissingZhInline,
                    file: "locales/docs/typst-docs.toml".to_owned(),
                    key: "a".to_owned(),
                    paragraph: None,
                    detail: None,
                    checked_in_en: None,
                    current_source_en: None,
                },
                Issue {
                    kind: IssueKind::MissingZhInline,
                    file: "locales/docs/typst-docs.toml".to_owned(),
                    key: "b".to_owned(),
                    paragraph: None,
                    detail: None,
                    checked_in_en: None,
                    current_source_en: None,
                },
                Issue {
                    kind: IssueKind::MissingZhInline,
                    file: "locales/docs/typst-docs.toml".to_owned(),
                    key: "c".to_owned(),
                    paragraph: None,
                    detail: None,
                    checked_in_en: None,
                    current_source_en: None,
                },
            ],
        }
        .with_issue_limit(Some(2));

        assert_snapshot!(render_text(&report), @r###"
        Found 3 translation issue(s) across 4 current documentation entries.
        Showing first 2 issue(s); 1 omitted. Re-run with `--limit 0` to show all results.
        
        locales/docs/typst-docs.toml
          - missing_zh_inline key=a
          - missing_zh_inline key=b
        "###);
        assert_snapshot!(serde_json::to_string_pretty(&report).unwrap(), @r###"
        {
          "scanned_entries": 4,
          "issue_count": 3,
          "displayed_issue_count": 2,
          "omitted_issue_count": 1,
          "issues": [
            {
              "kind": "missing_zh_inline",
              "file": "locales/docs/typst-docs.toml",
              "key": "a"
            },
            {
              "kind": "missing_zh_inline",
              "file": "locales/docs/typst-docs.toml",
              "key": "b"
            }
          ]
        }
        "###);
    }
}
