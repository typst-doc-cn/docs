//! Generates Typst Documentation

use core::fmt;

use tinymist_l10n::TranslationMapSet;

use crate::*;

/// A typed index for a Typst page.
#[derive(Debug, Clone, Copy)]
pub struct TypstPageIdx(usize);

/// This module generates the Typst documentation in a format that can be
/// used by the Typst documentation generator.
pub struct TypstPage {
    /// The children pages.
    children: Vec<TypstPageIdx>,
    /// The body of the page.
    body: TypstContent,
}

impl TypstPage {
    /// Writes to the output.
    pub fn write(&self, ctx: &GenContext, result: &mut impl Writer) -> anyhow::Result<()> {
        write_pagebreak(result)?;

        self.body.write(result)?;

        for child in &self.children {
            write_pagebreak(result)?;
            let child = ctx.get_page(*child);
            child.write(ctx, result)?;
        }
        Ok(())
    }
}

/// Writes a page break to the output.
fn write_pagebreak(result: &mut impl Writer) -> anyhow::Result<()> {
    result.push_str("\n\n#pagebreak();\n\n");
    Ok(())
}

/// A context for generating Typst documentation.
pub struct GenContext<'a> {
    // The target for the documentation.
    // target: Target,
    /// The translations for the documentation.
    transations: &'a TranslationMapSet,
    /// The output pages.
    pages: Vec<TypstPage>,
}

impl<'a> GenContext<'a> {
    /// Creates a new context for generating Typst documentation.
    pub fn new(transations: &'a TranslationMapSet) -> Self {
        Self {
            // target: Target::Paged,
            transations,
            pages: vec![],
        }
    }

    /// Gets the translation for a key.1
    fn get_translation<'b: 'a>(&self, key: &str, fallback: &'b str) -> &'a str {
        let dict = self
            .transations
            .get(key)
            .unwrap_or_else(|| panic!("Missing translation for {key}"));
        eprintln!("Getting {key}");
        dict.get("zh")
            .or_else(|| dict.get("en"))
            .map(|s| s.as_str())
            .unwrap_or(fallback)
    }

    /// Gets a page by index.
    pub fn get_page(&self, page: TypstPageIdx) -> &TypstPage {
        self.pages
            .get(page.0)
            .unwrap_or_else(|| panic!("Missing page {page:?}"))
    }

    /// Generates a logic page of Typst documentation.
    pub fn generate_page(&mut self, page: &PageMdModel) -> anyhow::Result<Option<TypstPageIdx>> {
        let mut children = vec![];

        for child in &page.children {
            if let Some(page) = self.generate_page(child)? {
                children.push(page);
            }
        }

        if page.route.contains("changelog") {
            return Ok(None);
        }

        let k = to_dot_path(&page.route);
        let k = if k.is_empty() { "index".to_owned() } else { k };

        let title_k = format!("{k}.title");
        let title = self.get_translation(&title_k, &page.title);
        let description_k = format!("{k}.description");
        let description = self.get_translation(&description_k, &page.description);

        // if let Some(part) = page.part {
        //     translations.push((format!("{part}.part"), part.into()));
        // }

        // check_outline(page.outline, &k, translations);
        let body = self.generate_body(&page.body, &k)?;

        let body = vec![
            TypstContent::Md(title_k, format!("## {title}")),
            TypstContent::Md(description_k, format!("### {description}")),
            body,
        ];
        let body = TypstContent::Seq(body);

        let page = TypstPage { children, body };
        let page_idx = TypstPageIdx(self.pages.len());
        self.pages.push(page);

        Ok(Some(page_idx))
    }

    /// Generates a body of Typst documentation.
    fn generate_body(&mut self, page: &BodyMdModel, k: &str) -> anyhow::Result<TypstContent> {
        let body = match page {
            BodyMdModel::Html(html) => {
                let k = format!("{k}.body");
                self.generate_html(html, &k)?
            }
            BodyMdModel::Category(category) => self.generate_category(category, k)?,
            BodyMdModel::Func(func) => self.generate_func(func, k)?,
            BodyMdModel::Group(group) => self.generate_group(group, k)?,
            BodyMdModel::Type(type_) => self.generate_type(type_, k)?,
            BodyMdModel::Symbols(symbols) => self.generate_symbols(symbols, k)?,
            BodyMdModel::Packages(html) => {
                let k = format!("{k}.packages");
                self.generate_html(html, &k)?
            }
        };
        Ok(body)
    }

    /// Generates a HTML or Markdown content.
    fn generate_html(&mut self, html: &HtmlMd, k: &str) -> anyhow::Result<TypstContent> {
        match html {
            HtmlMd::Html(html) => {
                let content = self.get_translation(k, html);
                Ok(TypstContent::Html(content.to_string()))
            }
            HtmlMd::Md(code) => {
                let content = self.get_translation(k, code);
                Ok(TypstContent::Md(k.to_string(), content.to_string()))
            }
        }
    }

    /// Generates a category content.
    fn generate_category(
        &mut self,
        category: &CategoryMdModel,
        k: &str,
    ) -> anyhow::Result<TypstContent> {
        let k = format!("{k}.{}", category.name);

        let title = self.get_translation(&format!("{k}.title"), &category.title);
        let heading = TypstContent::Typ(format!("== {title}"));

        let details_k = format!("{k}.details");
        let details = self.generate_html(&category.details, &details_k)?;

        let seq = vec![heading, details];
        Ok(TypstContent::Seq(seq))
    }

    /// Generates a function content.
    fn generate_func(&mut self, func: &FuncMdModel, k: &str) -> anyhow::Result<TypstContent> {
        let k = format!("{k}.{}", func.name);
        let title = self.get_translation(&format!("{k}.title"), &func.title);
        let heading = TypstContent::Typ(format!("== {title}"));

        let oneliner_k = format!("{k}.oneliner");
        let oneliner = self.get_translation(&oneliner_k, &func.oneliner);

        let oneliner = TypstContent::Md(oneliner_k, oneliner.to_string());

        let details_k = format!("{k}.details");
        let details = self.generate_html(&func.details, &details_k)?;

        let seq = vec![heading, oneliner, details];

        Ok(TypstContent::Seq(seq))
    }
    /// Generates a group content.
    fn generate_group(&mut self, group: &GroupMdModel, k: &str) -> anyhow::Result<TypstContent> {
        let k = format!("{k}.{}", group.name);
        let title = self.get_translation(&format!("{k}.title"), &group.title);
        let heading = TypstContent::Typ(format!("== {title}"));

        let details_k = format!("{k}.details");
        let details = self.generate_html(&group.details, &details_k)?;

        let seq = vec![heading, details];
        Ok(TypstContent::Seq(seq))
    }

    /// Generates a type content.
    fn generate_type(&mut self, type_: &TypeMdModel, k: &str) -> anyhow::Result<TypstContent> {
        let k = format!("{k}.{}", type_.name);
        let title = self.get_translation(&format!("{k}.title"), &type_.title);
        let heading = TypstContent::Typ(format!("== {title}"));

        let oneliner_k = format!("{k}.oneliner");
        let oneliner = self.get_translation(&oneliner_k, &type_.oneliner);

        let oneliner = TypstContent::Md(oneliner_k, oneliner.to_string());

        let details_k = format!("{k}.details");
        let details = self.generate_html(&type_.details, &details_k)?;

        let seq = vec![heading, oneliner, details];

        Ok(TypstContent::Seq(seq))
    }

    /// Generates a symbols content.
    fn generate_symbols(
        &mut self,
        symbols: &SymbolsMdModel,
        k: &str,
    ) -> anyhow::Result<TypstContent> {
        let k = format!("{k}.{}", symbols.name);
        let title = self.get_translation(&format!("{k}.title"), &symbols.title);
        let heading = TypstContent::Typ(format!("== {title}"));

        let details_k = format!("{k}.details");
        let details = self.generate_html(&symbols.details, &details_k)?;

        let seq = vec![heading, details];
        Ok(TypstContent::Seq(seq))
    }
}

/// Represents the content of a Typst page.
enum TypstContent {
    /// HTML content.
    Html(String),
    /// Markdown content.
    Md(String, String),
    /// Markdown content.
    Typ(String),
    /// A Sequence of content.
    Seq(Vec<TypstContent>),
}

impl TypstContent {
    /// Writes the content to the output.
    fn write(&self, result: &mut impl Writer) -> anyhow::Result<()> {
        result.push('\n');
        match self {
            TypstContent::Html(html) => {
                write!(result, "```````html\n{html}\n```````\n")?;
            }
            TypstContent::Md(prefix, md) => {
                write!(
                    result,
                    "#render-md(label-prefix: {prefix:?}, ```````md\n{md}\n```````)\n"
                )?;
            }
            TypstContent::Typ(typ) => result.push_str(typ),
            TypstContent::Seq(seq) => {
                for item in seq {
                    item.write(result)?;
                }
            }
        }
        Ok(())
    }
}

/// A model for writing output.
pub trait Writer: fmt::Write {
    /// Writes a string to the output.
    fn push_str(&mut self, s: &str);
    /// Writes a character to the output.
    fn push(&mut self, c: char);
}

impl Writer for String {
    fn push_str(&mut self, s: &str) {
        self.push_str(s);
    }

    fn push(&mut self, c: char) {
        self.push(c);
    }
}
