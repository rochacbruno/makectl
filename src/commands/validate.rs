use std::fs;
use std::path::Path;
use std::process;

use anyhow::{bail, Result};
use colored::Colorize;

use crate::linter::rules::{DuplicateTargets, LintRule, Severity, SpacesInsteadOfTabs};
use crate::parser::Makefile;

pub fn execute(file: &str) -> Result<()> {
    if !Path::new(file).exists() {
        bail!("Makefile not found at {}", file);
    }

    let content = fs::read_to_string(file)?;
    let makefile = Makefile::parse(&content)?;

    let syntax_rules: Vec<Box<dyn LintRule>> =
        vec![Box::new(SpacesInsteadOfTabs), Box::new(DuplicateTargets)];

    let results: Vec<_> = syntax_rules
        .iter()
        .flat_map(|rule| rule.check(&makefile, &content))
        .collect();

    if results.is_empty() {
        println!("{} {} is valid", "OK".green().bold(), file);
        return Ok(());
    }

    let mut has_errors = false;

    for r in &results {
        let prefix = match r.severity {
            Severity::Error => {
                has_errors = true;
                "error".red().bold()
            }
            Severity::Warning => "warning".yellow().bold(),
            Severity::Info => "info".blue().bold(),
        };

        let location = match r.line {
            Some(line) => format!("{}:{}", file, line),
            None => file.to_string(),
        };

        println!("{}: {} {}", prefix, location, r.message);
    }

    if has_errors {
        process::exit(1);
    }

    Ok(())
}
