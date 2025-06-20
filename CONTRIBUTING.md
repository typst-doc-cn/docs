## Contributing to the Project

Thank you for your interest in contributing to the Typst Docs Project! It contains two main components:

1. [`typst-docs-l10n`](./crates/typst-docs-l10n/): This is the localization tool that contains the translations for the Typst documentation.
2. [`locales/docs`](./locales/docs/): This directory contains the localized documentation files in various languages.

### Contributing to `typst-docs-l10n`

Please install the Rust toolchain. Then you can build the `typst-docs-l10n` crate using the following command:

```bash
cargo build crates/typst-docs-l10n
```

See the [README.md](./README.md) for usage of the `typst-docs-l10n` CLI.

### Contributing to `locales/docs`

To contribute to the localized documentation files, you only need to edit the files in the `locales/docs` directory, whose keys are generated by `typst-docs-l10n`, and values are _Markdown_ documentation extracted from [typst](https://github.com/typst/typst). The files are generated suitably for LLM-based translations.
