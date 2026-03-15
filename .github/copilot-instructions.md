# Localization Instructions for Claude/Copilot

This document provides comprehensive guidance for maintaining localization (l10n) in the Typst Documentation project. These instructions are specifically designed for AI assistants (Claude/Copilot) to ensure consistent and proper handling of translations.

For translation work, treat `docs/translation-principle.md` as the canonical
shared policy, `CONTRIBUTING.md` as the contributor workflow, and
`docs/terms.toml` as the canonical glossary.

## Keep Good PR Title

Determine a good PR prefix **only** by the PR description before work. Add a prefix to indicate what kind of release this pull request corresponds to. For reference, see https://www.conventionalcommits.org/

Available types:

- dev
- feat
- fix
- docs
- style
- refactor
- perf
- test
- build
- ci
- chore
- revert

## Project Structure

The localization files are stored in the `locales/docs/` directory:

- **Main file**: `locales/docs/typst-docs.toml` - Contains the primary translations
- **Detailed files**: `locales/docs/typst-docs/` - Contains detailed documentation sections
- **Shared policy**: `docs/translation-principle.md` - Shared translation rules for humans and AI assistants
- **Glossary**: `docs/terms.toml` - Canonical terminology for recurring or ambiguous terms

## Current Language Support

- `en` - English (source language)
- `zh` - Chinese (target language)

## Translation File Format

The project uses TOML format for translations. Each translatable string follows this pattern:

```toml
[section.subsection.key]
en = "English text"
zh = "Chinese translation"
```

The keys are ISO 639-1 language codes, and the values are messages in Markdown format.

### Example Entry Structure

**Example 1: Basic Entry**

```toml
[index.title]
en = "Overview"
zh = "概述"

[index.description]
en = "Learn how to use Typst to compose documents faster. Get started with the\ntutorial, or dive into the reference.\n"
zh = "了解如何使用 Typst 更快地撰写文档。通过教程入门，或深入参考。\n"
```

**Example 2: Adding French Translation**

```toml
[index.title]
en = "Overview"
zh = "概述"
fr = "Aperçu"  # Add this line
```

**Example 3: Adding Multiple Languages**

```toml
[guides.title]
en = "Guides"
zh = "指南"
fr = "Guides"     # French
de = "Anleitungen"  # German
es = "Guías"       # Spanish
ja = "ガイド"      # Japanese
```

## How to Add Translations

### Adding Chinese Translations to Existing Entries

When you find an entry that doesn't have a translation for the target language, e.g. `zh`, add the translation:

**Before:**

```toml
[Export.part]
en = "Export"
```

**After:**

```toml
[Export.part]
en = "Export"
zh = "导出"
```

## Translation Guidelines

### 1. Maintain TOML Format

- Always preserve the exact TOML structure
- Keep proper indentation and formatting
- Ensure proper escaping of special characters

### 2. Handle Multiline Text

For multiline text, use TOML multiline strings:

```toml
[section.body]
en = "\n# Title\nThis is a paragraph.\n\nThis is another paragraph."
zh = "\n# 标题\n这是一个段落。\n\n这是另一个段落。"
```

### 3. Preserve Markdown and Links

Maintain Markdown syntax and internal links:

```toml
[guides.body]
en = "\n# Guides\nWelcome to the Guides section!\n\n## List of Guides\n- [Guide for LaTeX users]($guides/guide-for-latex-users)\n- [Page setup guide]($guides/page-setup-guide)"
zh = "\n# 指南\n欢迎来到指南部分！\n\n## 指南列表\n- [LaTeX 用户指南]($guides/guide-for-latex-users)\n- [页面设置指南]($guides/page-setup-guide)"
```

### 4. Handle Special References

Some entries reference external files:

```toml
[guides.guide-for-latex-users.body]
en = "{{typst-docs/guides.guide-for-latex-users.body.toml}}"
```

All translations will then be loaded from the `typst-docs/guides.guide-for-latex-users.body.toml` file.

### 5. Use Terms Reference File

- `docs/terms.toml` file contains the terms used in the documentation, for future reference. If the reviewer or issuer gives the reference about translation, you should also add the reference to the `docs/terms.toml` file to help understanding the translation.
- When translating technical terms, include the original term with proper linking.
- Use the format: `「translated_term」（[original_term](#x-term-original_term)）`

**Example of basic format (array conversion):**

```
Converts an array of pairs into a dictionary.
```

should be translated as:

```
将每一项都是键值对的「数组」（[array](#x-term-array)）转换为「字典」（[dictionary](#x-term-dictionary)）。
```

**Example of avoiding nested parentheses, i.e. make fluent translation (exclusive boundary)**

```
end index (exclusive)
```

should be translated as:

```
结束索引且「不包括本身」（[exclusive](#x-term-exclusive)）
```

**Example of avoiding flooding the translation with too many terms (array conversion only first occurrence)**

```
Zips the array with other arrays.
```

should be translated as:

```
将「数组」（[array](#x-term-array)）与其他数组「同时迭代」（[zip](#x-term-zip)）。
```

**Example of common terms that don't need term references**

Very common terms like `function` and `integer` are used frequently and don't need the full term reference format. Instead:

- `function` → `函数` (not `「函数」（[function](#x-term-function)）`)
- `[integer]($int)` → `[整数]($int)` (not `「整数」（[integer]($int)）`)

The term reference format should be reserved for less common technical terms that benefit from explicit reference linking.

### 6. Follow Translation Best Practices

When translating, use this three-step process:

1. **Direct Translation**: Translate content directly while maintaining original format and not omitting any information.

2. **Quality Check**: Based on the first step, identify specific issues:

   - Non-conforming expressions to target language habits
   - Unclear or ambiguous sentences
   - Technical terms that need proper reference

3. **Refined Translation**: Re-translate ensuring original meaning while making it easier to understand and more natural in the target language.

   1. **Consistency**: Use consistent terminology throughout the project

   2. **Technical Accuracy**:

   - Preserve technical terms when appropriate
   - Maintain the meaning and context of the original text

   3. **Cultural Adaptation**:

   - Adapt text to be culturally appropriate for the target language
     - 少用被动句，人造形容词等使人感觉有翻译腔的表达
     - 当术语处于语境下，需要适度改变术语的情态。例如：
       - 原文是标题“Named”：「通过名称传入的」（[Named](#x-term-named)）
       - 原文有固定搭配：「命名参数」（[named parameter](#x-term-named)）
   - Consider regional variations when necessary

4. **Formatting Preservation**:

- Keep all markdown formatting intact
- Preserve line breaks and spacing
- Maintain internal link structure

## Workflow for AI Assistants

Follow the same console-first loop documented in `CONTRIBUTING.md`, and use
`docs/translation-principle.md` as the canonical source when this file and the
shared policy overlap.

### Step 1: Scan for a review-sized target

Start with the local helper instead of manually hunting through the TOML files:

```bash
cargo run -p translate -- scan
```

Choose one review-sized target from the helper output. Prefer a single entry, a
single included body file, or a small cluster of related keys over a broad
backlog sweep.

### Step 2: Locate the correct translation storage

- If the helper reports a top-level key such as `key=index.title`, edit
  `locales/docs/typst-docs.toml`.
- If the matching `en` value in that file is `{{typst-docs/<file>.toml}}`, edit
  the included body file under `locales/docs/typst-docs/`.
- If the helper reports `paragraph=main.<index>`, edit only that `[[main]]`
  entry in the included body file unless the task explicitly requires a wider
  sync.

### Step 3: Edit minimally using the shared policy

- Preserve TOML structure, Markdown, Typst code, links, labels, placeholders,
  and unrelated surrounding content.
- Reuse `docs/terms.toml` for recurring or ambiguous terminology, and add or
  refine glossary entries when the repository needs a canonical wording or a
  clearer usage note.

### Step 4: Validate and inspect the patch

Use the current local validation and review commands:

```
npm run validate
git diff -- <files>
```

Inspect the `npm run validate` output manually. The validator currently logs
parse errors without enforcing a non-zero exit code.

### Step 5: Prepare a pull request when requested

```bash
gh pr create
```

This step requires a working local GitHub CLI authentication setup.

## Common Patterns

### 1. Page Parts

```toml
[SectionName.part]
en = "Section Name"
zh = "部分名称"
```

### 2. Titles

```toml
[section.title]
en = "Title"
zh = "标题"
```

### 3. Descriptions

```toml
[section.description]
en = "Description text."
zh = "描述文本。"
```

### 4. Function Documentation

```toml
[reference.category.function.title]
en = "Function Name"
zh = "函数名称"

[reference.category.function.oneliner]
en = "Brief description of the function."
zh = "函数的简要描述。"

[reference.category.function.details]
en = "Detailed explanation of the function."
zh = "函数的详细说明。"
```

## Best Practices for AI Assistants

1. **Always preserve the TOML structure** - Never modify the section headers or key names
2. **Maintain consistency** - Use the same translation for the same English term throughout
3. **Preserve formatting** - Keep markdown, line breaks, and special characters intact
4. **Add complete translations** - Don't leave partial translations
5. **Follow the existing pattern** - Look at similar entries for guidance
6. **Validate syntax** - Ensure TOML remains parseable after changes

## Error Prevention

### Common Mistakes to Avoid

1. **Breaking TOML syntax**:

   ```toml
   # WRONG - missing quotes
   zh = 这是错误的

   # CORRECT
   zh = "这是正确的"
   ```

2. **Changing section names**:

   ```toml
   # WRONG - modifying section name
   [index.标题]

   # CORRECT - keep section name in English
   [index.title]
   ```

3. **Breaking markdown links**:

   ```toml
   # WRONG - translating link targets
   zh = "[指南]($guides/guide-for-latex-users-zh)"

   # CORRECT - keeping link targets unchanged
   zh = "[指南]($guides/guide-for-latex-users)"
   ```

This document should serve as an AI-assistant supplement to the shared policy in
`docs/translation-principle.md`. Always refer to existing translations for
consistency and keep the workflow aligned with `CONTRIBUTING.md`.
