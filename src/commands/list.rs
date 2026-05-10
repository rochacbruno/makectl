use std::fs;
use std::path::Path;

use anyhow::{bail, Result};
use colored::Colorize;

use crate::cli::Language;
use crate::parser::Makefile;
use crate::templates::TemplateRegistry;

pub fn execute(file: &str, show_targets: bool, lang: Option<Language>) -> Result<()> {
    if show_targets {
        list_targets(file)
    } else {
        list_templates(lang)
    }
}

fn list_templates(lang: Option<Language>) -> Result<()> {
    let registry = TemplateRegistry::new();

    let templates: Vec<_> = match lang {
        Some(ref l) => registry
            .list_by_language(l.as_str())
            .into_iter()
            .cloned()
            .collect(),
        None => registry.list_all().to_vec(),
    };

    if templates.is_empty() {
        println!("No templates found.");
        return Ok(());
    }

    let mut current_lang = String::new();
    for t in &templates {
        if t.language != current_lang {
            if !current_lang.is_empty() {
                println!();
            }
            println!("{}", t.language.to_uppercase().cyan().bold());
            current_lang = t.language.clone();
        }
        println!("  {:<25} {}", t.id.green(), t.description);
    }

    println!(
        "\nUse {} to add a template",
        "makectl add <template-id>".cyan()
    );

    Ok(())
}

fn list_targets(file: &str) -> Result<()> {
    if !Path::new(file).exists() {
        bail!("Makefile not found at {}", file);
    }

    let content = fs::read_to_string(file)?;
    let makefile = Makefile::parse(&content)?;

    let targets = makefile.targets();
    let managed = makefile.managed_blocks();

    if targets.is_empty() && managed.is_empty() {
        println!("No targets found in {}", file);
        return Ok(());
    }

    println!("{}", "Targets:".bold());

    for t in &targets {
        let badge = if t.is_phony { " [phony]" } else { "" };
        let help = match &t.help_text {
            Some(h) => format!(" - {}", h),
            None => String::new(),
        };
        println!("  {}{}{}", t.name.green(), badge.dimmed(), help);
    }

    if !managed.is_empty() {
        println!("\n{}", "Managed targets:".bold());
        for mb in &managed {
            println!(
                "  {} {} (from {})",
                mb.target_name.green(),
                "[managed]".cyan(),
                mb.template_id.dimmed()
            );
        }
    }

    Ok(())
}
