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

## Building Pdf Output

```bash
curl -L https://github.com/Myriad-Dreamin/shiroa/releases/download/v0.1.5/source-han-serif-font-assets.tar.gz | tar -xvz -C assets/fonts
```

```bash
typst compile --font-path assets/fonts ./target/typst-docs/docs.zh.typ "./target/typst-docs/Typst Docs v0.13.1 zh version.pdf"
# Windows
explorer.exe "./target/typst-docs/Typst Docs v0.13.1 zh version.pdf"
```

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).
