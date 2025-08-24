# Localization Instructions for Claude/Copilot

This document provides comprehensive guidance for maintaining localization (l10n) in the Typst Documentation project. These instructions are specifically designed for AI assistants (Claude/Copilot) to ensure consistent and proper handling of translations.

## Project Structure

The localization files are stored in the `locales/docs/` directory:
- **Main file**: `locales/docs/typst-docs.toml` - Contains the primary translations
- **Detailed files**: `locales/docs/typst-docs/` - Contains detailed documentation sections

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

### Example Entry Structure

```toml
[index.title]
en = "Overview"
zh = "概述"

[index.description]
en = "Learn how to use Typst to compose documents faster. Get started with the\ntutorial, or dive into the reference.\n"
zh = "了解如何使用 Typst 更快地撰写文档。通过教程入门，或深入参考。\n"
```

## How to Add Translations

### Adding Chinese Translations to Existing Entries

When you find an entry that has only English text, add the Chinese translation:

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

### Adding Translations for New Languages

To add support for a new language (e.g., French):

**Example 1: Adding French Translation**
```toml
[index.title]
en = "Overview"
zh = "概述"
fr = "Aperçu"  # Add this line
```

**Example 2: Adding Multiple Languages**
```toml
[guides.title]
en = "Guides"
zh = "指南"
fr = "Guides"     # French
de = "Anleitungen"  # German
es = "Guías"       # Spanish
ja = "ガイド"      # Japanese
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
zh = "{{typst-docs/guides.guide-for-latex-users.body.toml}}"
```

**Note**: Keep file references unchanged, as they point to separate detailed translation files.

## Language Codes

Use ISO 639-1 language codes:
- `en` - English
- `zh` - Chinese
- `fr` - French
- `de` - German
- `es` - Spanish
- `ja` - Japanese
- `ko` - Korean
- `pt` - Portuguese
- `ru` - Russian
- `it` - Italian

## Workflow for AI Assistants

### Step 1: Identify Missing Translations
Look for entries that have `en = "..."` but are missing translations for the target language.

### Step 2: Add Translations
Add the appropriate language code with proper translation while maintaining the TOML structure.

### Step 3: Validate Format
Ensure the TOML remains valid and properly formatted.

### Example Workflow

**Find untranslated entry:**
```toml
[Language.part]
en = "Language"
```

**Add Chinese translation:**
```toml
[Language.part]
en = "Language"
zh = "语言"
```

**Add multiple languages:**
```toml
[Language.part]
en = "Language"
zh = "语言"
fr = "Langue"
de = "Sprache"
es = "Idioma"
```

## Translation Quality Guidelines

### 1. Consistency
- Use consistent terminology throughout the project
- Maintain the same translation for repeated terms

### 2. Technical Accuracy
- Preserve technical terms when appropriate
- Maintain the meaning and context of the original text

### 3. Cultural Adaptation
- Adapt text to be culturally appropriate for the target language
- Consider regional variations when necessary

### 4. Formatting Preservation
- Keep all markdown formatting intact
- Preserve line breaks and spacing
- Maintain internal link structure

## Tools and Automation

The project includes a Rust CLI tool for managing translations:

```bash
# Generate translations structure
cargo run --bin typst-docs-l10n -- generate

# Update translations
cargo run --bin typst-docs-l10n -- translate

# Save translations
cargo run --bin typst-docs-l10n -- save
```

**Note**: Translators typically only need to edit the TOML files directly. The CLI tools are used by maintainers for generating and managing the translation structure.

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

This document should serve as a comprehensive reference for AI assistants working on localization tasks in this project. Always refer to existing translations for consistency and follow the established patterns.