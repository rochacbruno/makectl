use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

#[derive(Parser)]
#[command(
    name = "makectl",
    version,
    about = "Generate and manage targets in your Makefiles"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to Makefile
    #[arg(short, long, global = true, default_value = "Makefile")]
    pub file: String,

    /// Interactive mode (prompts for selections)
    #[arg(short, long, global = true)]
    pub interactive: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Makefile from scratch
    Init {
        /// Language template to use
        #[arg(short, long)]
        lang: Option<Language>,
        /// Force overwrite if Makefile exists
        #[arg(long)]
        force: bool,
    },
    /// Add template targets to existing Makefile
    Add {
        /// Templates to add (e.g., python/test, rust/build)
        templates: Vec<String>,
    },
    /// Remove a managed target from Makefile
    Remove {
        /// Target names to remove
        targets: Vec<String>,
    },
    /// List available templates (default) or Makefile targets
    List {
        /// Show targets in the Makefile instead of available templates
        #[arg(long)]
        targets: bool,
        /// Filter by language
        #[arg(short, long)]
        lang: Option<Language>,
    },
    /// Lint Makefile for common issues
    Lint,
    /// Format Makefile
    Fmt {
        /// Check only, don't modify
        #[arg(long)]
        check: bool,
    },
    /// Validate Makefile syntax
    Validate,
    /// Show Makefile best practice tips
    Tips,
    /// Generate shell completions
    Completions {
        /// Shell to generate for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Language {
    Generic,
    Python,
    Rust,
    Go,
    Node,
}

impl Language {
    pub fn as_str(&self) -> &str {
        match self {
            Language::Generic => "generic",
            Language::Python => "python",
            Language::Rust => "rust",
            Language::Go => "go",
            Language::Node => "node",
        }
    }
}
