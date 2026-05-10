use std::fs;
use std::path::Path;

use anyhow::{bail, Result};
use colored::Colorize;
use inquire::MultiSelect;

use crate::parser::Makefile;
use crate::templates::TemplateRegistry;

pub fn execute(file: &str, templates: &[String], interactive: bool) -> Result<()> {
    if !Path::new(file).exists() {
        bail!(
            "Makefile not found at {}. Run 'makectl init' to create one.",
            file
        );
    }

    let registry = TemplateRegistry::new();
    let content = fs::read_to_string(file)?;
    let makefile = Makefile::parse(&content)?;

    let template_ids = if interactive && templates.is_empty() {
        prompt_templates(&registry, &makefile)?
    } else if templates.is_empty() {
        bail!("No templates specified. Use -i for interactive mode or provide template names.");
    } else {
        templates.to_vec()
    };

    let mut to_append = String::new();
    let mut added = Vec::new();
    let mut skipped = Vec::new();

    for template_id in &template_ids {
        let template = match registry.get(template_id) {
            Some(t) => t,
            None => bail!(
                "Template '{}' not found. Run 'makectl list' to see available templates.",
                template_id
            ),
        };

        let target_name = &template.name;

        if makefile.has_target(target_name)
            || makefile
                .managed_blocks()
                .iter()
                .any(|mb| mb.template_id == *template_id)
        {
            skipped.push(template_id.clone());
            continue;
        }

        to_append.push_str(&format!(
            "\n# MAKECTL MANAGED START {} {}\n{}\n# MAKECTL MANAGED END {}\n",
            target_name, template_id, template.content, target_name
        ));
        added.push(template_id.clone());
    }

    if !to_append.is_empty() {
        let mut full = content;
        if !full.ends_with('\n') {
            full.push('\n');
        }
        full.push_str(&to_append);
        fs::write(file, full)?;
    }

    for name in &added {
        println!("  {} {}", "+".green().bold(), name);
    }
    for name in &skipped {
        println!("  {} {} (already exists)", "~".yellow().bold(), name);
    }

    if added.is_empty() && skipped.is_empty() {
        println!("Nothing to add.");
    } else if !added.is_empty() {
        println!(
            "{} Added {} template(s) to {}",
            "Done.".green().bold(),
            added.len(),
            file
        );
    }

    Ok(())
}

fn prompt_templates(registry: &TemplateRegistry, makefile: &Makefile) -> Result<Vec<String>> {
    let existing_ids: Vec<String> = makefile
        .managed_blocks()
        .iter()
        .map(|mb| mb.template_id.clone())
        .collect();

    let existing_targets: Vec<String> = makefile.targets().iter().map(|t| t.name.clone()).collect();

    let available: Vec<_> = registry
        .list_all()
        .iter()
        .filter(|t| !existing_ids.contains(&t.id) && !existing_targets.contains(&t.name))
        .collect();

    if available.is_empty() {
        println!("All templates are already added.");
        return Ok(Vec::new());
    }

    let display_items: Vec<String> = available
        .iter()
        .map(|t| format!("{} - {}", t.id, t.description))
        .collect();

    let selected = MultiSelect::new("Select templates to add:", display_items).prompt()?;

    let ids: Vec<String> = selected
        .iter()
        .filter_map(|s| s.split(" - ").next())
        .map(|s| s.to_string())
        .collect();

    Ok(ids)
}
