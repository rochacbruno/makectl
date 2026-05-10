use std::fs;
use std::path::Path;
use std::process;

use anyhow::{bail, Result};
use colored::Colorize;

use crate::parser::Makefile;

pub fn execute(file: &str, check: bool) -> Result<()> {
    if !Path::new(file).exists() {
        bail!("Makefile not found at {}", file);
    }

    let content = fs::read_to_string(file)?;
    let makefile = Makefile::parse(&content)?;
    let formatted = format_makefile(&makefile);

    if content == formatted {
        println!("{} {} is already formatted", "OK".green().bold(), file);
        return Ok(());
    }

    if check {
        println!("{} {} needs formatting", "FAIL".red().bold(), file);
        process::exit(1);
    }

    fs::write(file, &formatted)?;
    println!("{} Formatted {}", "OK".green().bold(), file);

    Ok(())
}

fn format_makefile(makefile: &Makefile) -> String {
    let raw = makefile.to_string();
    let mut output = String::new();
    let mut prev_blank = false;
    let mut first = true;

    for line in raw.lines() {
        let trimmed_right = line.trim_end();

        if trimmed_right.is_empty() {
            if prev_blank || first {
                continue;
            }
            output.push('\n');
            prev_blank = true;
            first = false;
            continue;
        }

        prev_blank = false;
        first = false;
        output.push_str(trimmed_right);
        output.push('\n');
    }

    if !output.is_empty() && !output.ends_with('\n') {
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_trailing_whitespace() {
        let input = "build:   \n\tcargo build  \n";
        let mf = Makefile::parse(input).unwrap();
        let formatted = format_makefile(&mf);
        assert!(!formatted.contains("   \n"));
        assert!(formatted.contains("build:\n"));
    }

    #[test]
    fn collapses_multiple_blank_lines() {
        let input = "build:\n\tcargo build\n\n\n\ntest:\n\tcargo test\n";
        let mf = Makefile::parse(input).unwrap();
        let formatted = format_makefile(&mf);
        assert!(!formatted.contains("\n\n\n"));
    }

    #[test]
    fn ends_with_newline() {
        let input = "build:\n\tcargo build";
        let mf = Makefile::parse(input).unwrap();
        let formatted = format_makefile(&mf);
        assert!(formatted.ends_with('\n'));
    }

    #[test]
    fn idempotent() {
        let input = ".PHONY: test\n\ntest:\n\tpytest\n";
        let mf = Makefile::parse(input).unwrap();
        let first = format_makefile(&mf);
        let mf2 = Makefile::parse(&first).unwrap();
        let second = format_makefile(&mf2);
        assert_eq!(first, second);
    }
}
