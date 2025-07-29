//! # typst-docs-l10n
//!
//! This is a documentation localization project for the Typst project.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::Parser;
use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use std::io::{Read, Write};
use tinymist_l10n::{
    deserialize, load_translations, serialize_translations, update_disk_translations,
    TranslationMap, TranslationMapSet,
};
use typst_docs::provide;
use typst_docs_l10n::generate::GenContext;
use typst_docs_l10n::resolve::CliResolver;
use typst_docs_l10n::translate::check_page;
use typst_docs_l10n::PageMdModel;

use crate::split::split_markdown;

mod split;

/// The main function
fn main() -> anyhow::Result<()> {
    let args = Command::parse();

    match args {
        Command::Generate(args) => generate(args),
        Command::Translate(args) => translate(args),
        Command::Make(args) => make(args),
        Command::Save(args) => save(args),
    }
}

/// The command line arguments.
#[derive(Parser, Debug)]
enum Command {
    /// Generates the JSON representation of the documentation.
    #[clap()]
    Generate(GenerateArgs),
    /// Updates the translations of the documentation.
    #[clap()]
    Translate(TranslateArgs),
    /// Makes a typst document.
    #[clap()]
    Make(MakeArgs),
    /// Saves the translations to disk.
    #[clap()]
    Save(SaveArgs),
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

    /// The directory for the translated documentation.
    #[arg(long, default_value = "locales/docs")]
    translation_dir: PathBuf,
}

const MARKDOWN_PAR_SEP: &str = "\n\n";

/// Updates the translations of the documentation.
fn translate(args: TranslateArgs) -> anyhow::Result<()> {
    let json = fs::read_to_string(&args.docs_file)?;
    let pages: Vec<PageMdModel> = serde_json::from_str(&json)?;

    let sub_docs = args.translation_dir.join("typst-docs");
    std::fs::create_dir_all(&sub_docs)
        .with_context(|| format!("Failed to create directory: {}", sub_docs.display()))?;
    let doc_translations = pages
        .into_par_iter()
        .flat_map(|page| {
            let mut translations = vec![];
            check_page(page, &mut translations);

            translations.par_iter_mut().for_each(|(k, v)| {
                let count_pars = v.matches(MARKDOWN_PAR_SEP).take(5).count();

                if count_pars < 5 {
                    *v = serde_json::to_string(v).unwrap();
                } else {
                    write_large_translate(&sub_docs, k, v);
                }
            });

            translations
        })
        .collect::<Vec<_>>();

    std::fs::create_dir_all(&args.translation_dir)?;
    update_disk_translations(
        doc_translations,
        &args.translation_dir.join("typst-docs.toml"),
    )?;

    Ok(())
}

/// Writes a large translation text to a file.
fn write_large_translate(sub_docs: &Path, k: &str, v: &mut String) {
    let k = format!("{k}.toml");
    let path = sub_docs.join(&k);
    let rel_path = Path::new("typst-docs").join(&k);

    let pars = split_markdown(v);

    init_large_translation(&path, &pars)
        .with_context(|| format!("Failed to store large translation file: {path:?}"))
        .unwrap();

    *v = serde_json::to_string(&format!("{{{{{}}}}}", rel_path.display())).unwrap();
}

/// Arguments to make a typst document.
#[derive(Parser, Debug)]
struct MakeArgs {
    /// The JSON file containing the documentation.
    #[arg(long, default_value = "dist/docs.json")]
    docs_file: PathBuf,

    /// The directory for the translated documentation.
    #[arg(long, default_value = "locales/docs")]
    translation_dir: PathBuf,

    /// The output directory for the typst document.
    #[arg(long, short, default_value = "target/typst-docs")]
    output_dir: PathBuf,
}

/// Makes a typst document.
fn make(args: MakeArgs) -> anyhow::Result<()> {
    let json = fs::read_to_string(&args.docs_file)?;
    let pages: Vec<PageMdModel> = serde_json::from_str(&json)?;

    let translations_path = args.translation_dir.join("typst-docs.toml");
    let translations_str = fs::read_to_string(&translations_path)?;
    let raw = load_translations(&translations_str)?;

    // todo: key first
    let mut translations = TranslationMapSet::default();
    for (lang, value) in raw {
        for (key, value) in value {
            translations
                .entry(key)
                .or_default()
                .insert(lang.clone(), value);
        }
    }

    let mut ctx = GenContext::new(&translations);
    let typst_pages = pages
        .into_iter()
        .flat_map(|page| ctx.generate_page(&page).transpose())
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut result = include_str!("template.typ").to_string();
    for page in typst_pages {
        let page = ctx.get_page(page);
        page.write(&ctx, &mut result)?;
    }

    std::fs::create_dir_all(&args.output_dir)?;
    let output_path = args.output_dir.join("docs.zh.typ");
    fs::write(&output_path, &*result)?;

    Ok(())
}

/// Arguments to save changes.
#[derive(Parser, Debug)]
struct SaveArgs {
    /// The directory for the translated documentation.
    #[arg(long, default_value = "locales/docs")]
    translation_dir: PathBuf,
}

fn save(args: SaveArgs) -> anyhow::Result<()> {
    let input = std::io::stdin();
    let mut input = input.lock();
    let mut input_buffer = String::new();
    input
        .read_to_string(&mut input_buffer)
        .with_context(|| "Failed to read from standard input")?;
    let translated: Translated =
        serde_json::from_str(&input_buffer).with_context(|| "Failed to parse input as JSON")?;

    if translated.file == "typst-docs.toml" {
        let translation_file = args.translation_dir.join("typst-docs.toml");

        let existing_translations_str =
            fs::read_to_string(&translation_file).with_context(|| {
                format!(
                    "Failed to read existing translations: {}",
                    translation_file.display()
                )
            })?;

        let mut existing_translations = deserialize(&existing_translations_str, true)?;

        for pair in translated.translates {
            let entry = existing_translations.entry(pair.path).or_default();
            entry.insert(
                "zh".to_owned(),
                serde_json::to_string(&pair.content).unwrap(),
            );
        }

        // Writes translations
        let result = serialize_translations(existing_translations);
        std::fs::write(translation_file, result)?;
    } else {
        let translation_file = args.translation_dir.join(translated.file);

        let existing_translations_str =
            fs::read_to_string(&translation_file).with_context(|| {
                format!(
                    "Failed to read existing translations: {}",
                    translation_file.display()
                )
            })?;

        let mut existing_translations =
            toml::from_str::<LargeTranslationFile>(&existing_translations_str)
                .with_context(|| "Failed to parse existing translations")?;

        for pair in translated.translates {
            if let Some((_sub, maybe_number)) = pair.path.rsplit_once('.') {
                let number = maybe_number.parse::<usize>();
                if let Ok(number) = number {
                    let entry = existing_translations.main.get_mut(number).unwrap();
                    entry.insert("zh".to_owned(), pair.content);
                }
            }
        }

        store_large_translation_file(&translation_file, &existing_translations.main)?;
    }

    Ok(())
}

/// Translated content structure.
#[derive(Debug, serde::Deserialize)]
struct Translated {
    /// The file path of the translation.
    file: String,
    /// The translated content.
    translates: Vec<TranslatedPair>,
}

/// A pair of translated content.
#[derive(Debug, serde::Deserialize)]
struct TranslatedPair {
    /// The path to the translation.
    path: String,
    /// The translated content.
    content: String,
}

/// The large translated file.
#[derive(Debug, serde::Deserialize)]
struct LargeTranslationFile {
    /// The translations.
    main: Vec<TranslationMap>,
}

fn init_large_translation(path: &Path, pars: &[&str]) -> anyhow::Result<()> {
    let mut file = fs::File::create(path)?;
    for par in pars.iter() {
        if par.contains("\"\"\"") {
            let content = serde_json::to_string(par).unwrap();
            write!(file, "\n[[main]]\nen = {content}\n")?;
        } else {
            let content = serde_json::to_string(par).unwrap();
            let content = unescape(content);
            write!(file, "\n[[main]]\nen = \"\"{content}\"\"\n")?;
        }
    }

    Ok(())
}

/// Stores a large translation file.
fn store_large_translation_file(path: &Path, pars: &[TranslationMap]) -> anyhow::Result<()> {
    let mut file = fs::File::create(path)?;
    for par in pars.iter() {
        write!(file, "\n[[main]]\n")?;
        let mut store_one = |lang: &str| {
            let Some(content) = par.get(lang) else {
                return Ok(());
            };
            if content.contains("\"\"\"") {
                let content = serde_json::to_string(content).unwrap();
                writeln!(file, "{lang} = {content:?}")?;
            } else {
                let content = serde_json::to_string(content).unwrap();
                let content = unescape(content);
                writeln!(file, "{lang} = \"\"{content}\"\"")?;
            }
            anyhow::Ok(())
        };

        store_one("en")?;
        store_one("zh")?;
    }

    Ok(())
}

/// Unescapes a string by removing toml-safe escape characters.
fn unescape(s: String) -> String {
    let mut is_escaped = false;
    let mut output = vec![];
    for ch in s.chars() {
        if is_escaped {
            if ch == 'n' {
                output.push('\n');
            } else if ch == '"' {
                output.push('"');
            } else {
                output.push('\\');
                output.push(ch);
            }

            is_escaped = false;
        } else if ch == '\\' {
            is_escaped = true;
        } else {
            output.push(ch);
        }
    }

    output.into_iter().collect::<String>()
}
