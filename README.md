## Typst Docs Localization

Localization for [`typst-docs`](https://github.com/typst/typst/tree/main/docs).

## Usage

```bash
cargo run --bin typst-docs-l10n -- generate
```

```bash
cargo run --bin typst-docs-l10n -- translate
```

```bash
cargo run --bin typst-docs-l10n -- make
```

```bash
typst compile ./target/typst-docs/docs.zh.typ "./target/typst-docs/Typst Docs v0.13.1 zh version.pdf"
# Windows
explorer.exe "./target/typst-docs/Typst Docs v0.13.1 zh version.pdf"
```
