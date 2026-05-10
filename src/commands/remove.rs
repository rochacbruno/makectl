use std::fs;
use std::path::Path;

use anyhow::{bail, Result};
use colored::Colorize;
use inquire::MultiSelect;

use crate::parser::Makefile;

pub fn execute(file: &str, targets: &[String], interactive: bool) -> Result<()> {
    if !Path::new(file).exists() {
        bail!("Makefile not found at {}", file);
    }

    let content = fs::read_to_string(file)?;
    let mut makefile = Makefile::parse(&content)?;

    let target_names = if interactive && targets.is_empty() {
        prompt_targets(&makefile)?
    } else if targets.is_empty() {
        bail!("No targets specified. Use -i for interactive mode or provide target names.");
    } else {
        targets.to_vec()
    };

    let mut removed = Vec::new();
    let mut not_found = Vec::new();

    for target in &target_names {
        match makefile.remove_managed_block(target) {
            Ok(()) => removed.push(target.clone()),
            Err(_) => not_found.push(target.clone()),
        }
    }

    if !removed.is_empty() {
        fs::write(file, makefile.to_string())?;
    }

    for name in &removed {
        println!("  {} {}", "-".red().bold(), name);
    }
    for name in &not_found {
        println!("  {} {} (not a managed target)", "?".yellow().bold(), name);
    }

    if !removed.is_empty() {
        println!(
            "{} Removed {} target(s) from {}",
            "Done.".green().bold(),
            removed.len(),
            file
        );
    }

    Ok(())
}

fn prompt_targets(makefile: &Makefile) -> Result<Vec<String>> {
    let blocks = makefile.managed_blocks();

    if blocks.is_empty() {
        println!("No managed targets to remove.");
        return Ok(Vec::new());
    }

    let display_items: Vec<String> = blocks
        .iter()
        .map(|mb| format!("{} (from {})", mb.target_name, mb.template_id))
        .collect();

    let selected = MultiSelect::new("Select targets to remove:", display_items).prompt()?;

    let names: Vec<String> = selected
        .iter()
        .filter_map(|s| s.split(" (from ").next())
        .map(|s| s.to_string())
        .collect();

    Ok(names)
}
