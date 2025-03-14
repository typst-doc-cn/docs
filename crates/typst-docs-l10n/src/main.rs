//! # typst-docs-l10n
//!
//! This is a documentation localization project for the Typst project.

use std::fs;
use std::path::PathBuf;

use clap::Parser;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tinymist_l10n::update_disk_translations;
use typst_docs::provide;
use typst_docs_l10n::generate::CliResolver;
use typst_docs_l10n::translate::check_page;
use typst_docs_l10n::PageMdModel;

/// The main function
fn main() -> anyhow::Result<()> {
    let args = Command::parse();

    match args {
        Command::Generate(args) => generate(args),
        Command::Translate(args) => translate(args),
    }
}

/// The command line arguments.
#[derive(Parser, Debug)]
enum Command {
    /// Generate the JSON representation of the documentation.
    #[clap()]
    Generate(GenerateArgs),
    /// Update the translations of the documentation.
    #[clap()]
    Translate(TranslateArgs),
}

/// Generates the JSON representation of the documentation. This can be used to
/// generate the HTML yourself. Be warned: the JSON structure is not stable and
/// may change at any time.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct GenerateArgs {
    /// The generation process can produce additional assets. Namely images.
    /// This option controls where to spit them out. The HTML generation will
    /// assume that this output directory is served at `${base_url}/assets/*`.
    /// The default is `assets`. For example, if the base URL is `/docs/` then
    /// the generated HTML might look like `<img src="/docs/assets/foo.png">`
    /// even though the `--assets-dir` was set to `/tmp/images` or something.
    #[arg(long, default_value = "dist/assets")]
    assets_dir: PathBuf,

    /// Write the JSON output to this file. The default is `-` which is a
    /// special value that means "write to standard output". If you want to
    /// write to a file named `-` then use `./-`.
    #[arg(long, default_value = "dist/docs.json")]
    out_file: PathBuf,

    /// The base URL for the documentation. This can be an absolute URL like
    /// `https://example.com/docs/` or a relative URL like `/docs/`. This is
    /// used as the base URL for the generated page's `.route` properties as
    /// well as cross-page links. The default is `/`. If a `/` trailing slash is
    /// not present then it will be added. This option also affects the HTML
    /// asset references. For example: `--base /docs/` will generate
    /// `<img src="/docs/assets/foo.png">`.
    #[arg(long, default_value = "/")]
    base: String,

    /// Enable verbose logging. This will print out all the calls to the
    /// resolver and the paths of the generated assets.
    #[arg(long)]
    verbose: bool,
}

/// Generates the JSON representation of the documentation.
fn generate(args: GenerateArgs) -> anyhow::Result<()> {
    let mut base = args.base.clone();
    if !base.ends_with('/') {
        base.push('/');
    }

    let resolver = CliResolver {
        assets_dir: &args.assets_dir,
        verbose: args.verbose,
        base: &base,
    };
    if args.verbose {
        eprintln!("resolver: {resolver:?}");
    }
    let pages = provide(&resolver)
        .into_iter()
        .map(|page| page.into())
        .collect::<Vec<PageMdModel>>();

    eprintln!("Be warned: the JSON structure is not stable and may change at any time.");
    let json = serde_json::to_string_pretty(&pages)?;

    if args.out_file.to_string_lossy() == "-" {
        println!("{json}");
    } else {
        fs::write(&args.out_file, &*json)?;
    }

    Ok(())
}

/// Updates the translations of the documentation.
#[derive(Parser, Debug)]
struct TranslateArgs {
    /// The JSON file containing the documentation.
    #[arg(long, default_value = "dist/docs.json")]
    docs_file: PathBuf,

    /// The output directory for the translated documentation.
    #[arg(long, default_value = "locales/docs")]
    out_dir: PathBuf,
}

/// Updates the translations of the documentation.
fn translate(args: TranslateArgs) -> anyhow::Result<()> {
    let json = fs::read_to_string(&args.docs_file)?;
    let pages: Vec<PageMdModel> = serde_json::from_str(&json)?;

    let doc_translations = pages
        .into_par_iter()
        .flat_map(|page| {
            let mut translations = vec![];
            check_page(page, &mut translations);
            for (_k, v) in translations.iter_mut() {
                *v = serde_json::to_string(v).unwrap();
            }
            translations
        })
        .collect::<Vec<_>>();

    std::fs::create_dir_all(&args.out_dir)?;
    update_disk_translations(doc_translations, &args.out_dir.join("typst-docs.toml"))?;

    Ok(())
}
