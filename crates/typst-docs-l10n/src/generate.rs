//! Generates a JSON representation of the documentation. This can be used to
//! generate the HTML yourself. Be warned: the JSON structure is not stable and
//! may change at any time.

use std::fs;
use std::path::Path;

use typst::layout::PagedDocument;
use typst_docs::{Html, Resolver};
use typst_render::render;

/// A resolver that uses the command line arguments to generate the
/// documentation.
#[derive(Debug)]
pub struct CliResolver<'a> {
    /// The directory where the assets are written to.
    pub assets_dir: &'a Path,
    /// Enable verbose logging.
    pub verbose: bool,
    /// The base URL for the documentation.
    pub base: &'a str,
}

impl Resolver for CliResolver<'_> {
    fn commits(&self, from: &str, to: &str) -> Vec<typst_docs::Commit> {
        if self.verbose {
            eprintln!("commits({from}, {to})");
        }
        vec![]
    }

    fn example(
        &self,
        hash: u128,
        source: Option<Html>,
        document: &PagedDocument,
    ) -> typst_docs::Html {
        if self.verbose {
            eprintln!(
                "example(0x{hash:x}, {:?} chars, Document)",
                source.as_ref().map(|s| s.as_str().len())
            );
        }

        let page = document.pages.first().expect("page 0");
        let pixmap = render(page, 2.0);
        let filename = format!("{hash:x}.png");
        let path = self.assets_dir.join(&filename);
        fs::create_dir_all(path.parent().expect("parent")).expect("create dir");
        pixmap.save_png(path.as_path()).expect("save png");
        let src = format!("{}assets/{filename}", self.base);
        eprintln!("Generated example image {path:?}");

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

    fn image(&self, filename: &str, data: &[u8]) -> String {
        if self.verbose {
            eprintln!("image({filename}, {} bytes)", data.len());
        }

        let path = self.assets_dir.join(filename);
        fs::create_dir_all(path.parent().expect("parent")).expect("create dir");
        fs::write(&path, data).expect("write image");
        eprintln!("Created {} byte image at {path:?}", data.len());

        format!("{}assets/{filename}", self.base)
    }

    fn link(&self, link: &str) -> Option<String> {
        if self.verbose {
            eprintln!("link({link})");
        }
        None
    }

    fn base(&self) -> &str {
        self.base
    }
}
