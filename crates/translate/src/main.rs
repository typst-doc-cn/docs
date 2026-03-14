//! CLI entry point for the translation status helper.

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use translate::{render_text, scan_repo, RepoPaths, DEFAULT_ISSUE_LIMIT};

/// Scans Typst documentation translations for missing or stale entries.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Selected helper subcommand.
    #[command(subcommand)]
    command: Command,
}

/// Supported commands for the translation helper.
#[derive(Debug, Subcommand)]
enum Command {
    /// Scan the checked-in translation files against the current source text.
    Scan(ScanArgs),
}

/// Command-line arguments for the `scan` subcommand.
#[derive(Debug, Parser)]
struct ScanArgs {
    /// Path to the top-level translation TOML file.
    #[arg(long, default_value = "locales/docs/typst-docs.toml")]
    translations: PathBuf,

    /// Path to the directory that stores split body-file translations.
    #[arg(long, default_value = "locales/docs/typst-docs")]
    included_dir: PathBuf,

    /// Base URL used when collecting the current documentation source text.
    #[arg(long, default_value = "/")]
    base: String,

    /// Output format for scan results.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,

    /// Maximum number of issues to include in the output. Use `0` to disable
    /// the limit.
    #[arg(long, default_value_t = DEFAULT_ISSUE_LIMIT)]
    limit: usize,
}

/// Supported output formats for the scanner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    /// Human-readable grouped text output.
    Text,
    /// Structured JSON output for machine-readable workflows.
    Json,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan(args) => {
            let paths = RepoPaths::new(args.translations, args.included_dir, args.base);
            let limit = (args.limit != 0).then_some(args.limit);
            let report = scan_repo(&paths)?.with_issue_limit(limit);

            match args.format {
                OutputFormat::Text => print!("{}", render_text(&report)),
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
            }
        }
    }

    Ok(())
}
