use std::fs;
use std::path::Path;
use std::process;

use anyhow::{bail, Result};
use colored::Colorize;

use crate::linter;
use crate::linter::rules::Severity;
use crate::parser::Makefile;

pub fn execute(file: &str) -> Result<()> {
    if !Path::new(file).exists() {
        bail!("Makefile not found at {}", file);
    }

    let content = fs::read_to_string(file)?;
    let makefile = Makefile::parse(&content)?;
    let results = linter::lint(&makefile, &content);

    if results.is_empty() {
        println!("{} No issues found in {}", "OK".green().bold(), file);
        return Ok(());
    }

    let mut errors = 0;
    let mut warnings = 0;
    let mut infos = 0;

    for r in &results {
        let (prefix, color_msg) = match r.severity {
            Severity::Error => {
                errors += 1;
                ("error".red().bold(), r.message.red())
            }
            Severity::Warning => {
                warnings += 1;
                ("warning".yellow().bold(), r.message.yellow())
            }
            Severity::Info => {
                infos += 1;
                ("info".blue().bold(), r.message.blue())
            }
        };

        let location = match r.line {
            Some(line) => format!("{}:{}", file, line),
            None => file.to_string(),
        };

        println!("{} [{}] {}: {}", prefix, r.rule, location, color_msg);

        if let Some(ref suggestion) = r.suggestion {
            println!("  {} {}", "fix:".dimmed(), suggestion);
        }
    }

    println!(
        "\n{} error(s), {} warning(s), {} info(s)",
        errors, warnings, infos
    );

    if errors > 0 {
        process::exit(1);
    }

    Ok(())
}
