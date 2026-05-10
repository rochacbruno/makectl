use std::fmt;

use crate::error::MakectlError;

#[derive(Debug, Clone)]
pub struct Makefile {
    pub elements: Vec<MakefileElement>,
}

#[derive(Debug, Clone)]
pub enum MakefileElement {
    Comment(String),
    BlankLine,
    Variable(Variable),
    Phony(Vec<String>),
    Target(Target),
    ManagedBlock(ManagedBlock),
    Include(String),
    Other(String),
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Target {
    pub name: String,
    pub prerequisites: Vec<String>,
    pub recipe_lines: Vec<String>,
    pub line_number: usize,
    pub is_phony: bool,
    pub help_text: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ManagedBlock {
    pub target_name: String,
    pub template_id: String,
    pub content: String,
}

const MANAGED_START: &str = "# MAKECTL MANAGED START";
const MANAGED_END: &str = "# MAKECTL MANAGED END";
const LEGACY_START: &str = "# MAKECTL MANAGED BLOCK INIT";
const LEGACY_END: &str = "# MAKECTL MANAGED BLOCK END";

#[derive(Debug, PartialEq)]
enum ParserState {
    Normal,
    InRecipe(usize),
    InManagedBlock,
}

impl Makefile {
    pub fn parse(content: &str) -> Result<Self, MakectlError> {
        let mut elements: Vec<MakefileElement> = Vec::new();
        let mut state = ParserState::Normal;
        let mut phony_names: Vec<String> = Vec::new();
        let mut managed_target = String::new();
        let mut managed_template = String::new();
        let mut managed_content = String::new();
        let mut managed_start_line: usize = 0;

        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            let line_num = i + 1;

            match state {
                ParserState::Normal => {
                    if let Some(rest) = line.strip_prefix(MANAGED_START) {
                        let parts: Vec<&str> = rest.split_whitespace().collect();
                        managed_target = parts.first().unwrap_or(&"unknown").to_string();
                        managed_template = parts.get(1).unwrap_or(&"unknown").to_string();
                        managed_content.clear();
                        managed_start_line = line_num;
                        state = ParserState::InManagedBlock;
                    } else if line.starts_with(LEGACY_START) {
                        managed_target = String::from("legacy");
                        managed_template = String::from("legacy");
                        managed_content.clear();
                        managed_start_line = line_num;
                        state = ParserState::InManagedBlock;
                    } else if line.trim().is_empty() {
                        elements.push(MakefileElement::BlankLine);
                    } else if line.starts_with('#') {
                        elements.push(MakefileElement::Comment(line.to_string()));
                    } else if line.starts_with(".PHONY:") || line.starts_with(".PHONY :") {
                        let after_colon = line.split_once(':').map(|x| x.1).unwrap_or("");
                        let targets: Vec<String> = after_colon
                            .split_whitespace()
                            .map(|s| s.to_string())
                            .collect();
                        phony_names.extend(targets.clone());
                        elements.push(MakefileElement::Phony(targets));
                    } else if line.starts_with("include ") || line.starts_with("-include ") {
                        elements.push(MakefileElement::Include(line.to_string()));
                    } else if let Some(var) = try_parse_variable(line) {
                        elements.push(MakefileElement::Variable(Variable {
                            name: var.0,
                            operator: var.1,
                            value: var.2,
                        }));
                    } else if let Some(target) = try_parse_target_header(line) {
                        let help_text = extract_help_text(line);
                        elements.push(MakefileElement::Target(Target {
                            name: target.0,
                            prerequisites: target.1,
                            recipe_lines: Vec::new(),
                            line_number: line_num,
                            is_phony: false,
                            help_text,
                        }));
                        state = ParserState::InRecipe(elements.len() - 1);
                    } else {
                        elements.push(MakefileElement::Other(line.to_string()));
                    }
                }
                ParserState::InRecipe(target_idx) => {
                    if line.starts_with('\t') {
                        if let MakefileElement::Target(ref mut t) = elements[target_idx] {
                            t.recipe_lines.push(line.to_string());
                        }
                    } else {
                        state = ParserState::Normal;
                        continue;
                    }
                }
                ParserState::InManagedBlock => {
                    if line.starts_with(MANAGED_END) || line.starts_with(LEGACY_END) {
                        elements.push(MakefileElement::ManagedBlock(ManagedBlock {
                            target_name: managed_target.clone(),
                            template_id: managed_template.clone(),
                            content: managed_content.clone(),
                        }));
                        state = ParserState::Normal;
                    } else {
                        if !managed_content.is_empty() {
                            managed_content.push('\n');
                        }
                        managed_content.push_str(line);
                    }
                }
            }
            i += 1;
        }

        if state == ParserState::InManagedBlock {
            return Err(MakectlError::ParseError {
                line: managed_start_line,
                message: "Unclosed managed block".to_string(),
            });
        }

        for element in &mut elements {
            if let MakefileElement::Target(ref mut target) = element {
                if phony_names.contains(&target.name) {
                    target.is_phony = true;
                }
            }
        }

        Ok(Makefile { elements })
    }

    pub fn targets(&self) -> Vec<&Target> {
        self.elements
            .iter()
            .filter_map(|e| match e {
                MakefileElement::Target(t) => Some(t),
                _ => None,
            })
            .collect()
    }

    pub fn managed_blocks(&self) -> Vec<&ManagedBlock> {
        self.elements
            .iter()
            .filter_map(|e| match e {
                MakefileElement::ManagedBlock(mb) => Some(mb),
                _ => None,
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn variables(&self) -> Vec<&Variable> {
        self.elements
            .iter()
            .filter_map(|e| match e {
                MakefileElement::Variable(v) => Some(v),
                _ => None,
            })
            .collect()
    }

    pub fn phony_targets(&self) -> Vec<String> {
        self.elements
            .iter()
            .filter_map(|e| match e {
                MakefileElement::Phony(targets) => Some(targets.clone()),
                _ => None,
            })
            .flatten()
            .collect()
    }

    pub fn has_target(&self, name: &str) -> bool {
        self.targets().iter().any(|t| t.name == name)
            || self
                .managed_blocks()
                .iter()
                .any(|mb| mb.target_name == name)
    }

    pub fn remove_managed_block(&mut self, target_name: &str) -> Result<(), MakectlError> {
        let idx = self.elements.iter().position(
            |e| matches!(e, MakefileElement::ManagedBlock(mb) if mb.target_name == target_name),
        );
        match idx {
            Some(i) => {
                self.elements.remove(i);
                Ok(())
            }
            None => Err(MakectlError::ManagedBlockNotFound {
                name: target_name.to_string(),
            }),
        }
    }
}

impl fmt::Display for Makefile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for element in &self.elements {
            match element {
                MakefileElement::Comment(c) => writeln!(f, "{}", c)?,
                MakefileElement::BlankLine => writeln!(f)?,
                MakefileElement::Variable(v) => {
                    writeln!(f, "{} {} {}", v.name, v.operator, v.value)?;
                }
                MakefileElement::Phony(targets) => {
                    writeln!(f, ".PHONY: {}", targets.join(" "))?;
                }
                MakefileElement::Target(t) => {
                    if t.prerequisites.is_empty() {
                        if let Some(ref help) = t.help_text {
                            writeln!(f, "{}: ## {}", t.name, help)?;
                        } else {
                            writeln!(f, "{}:", t.name)?;
                        }
                    } else if let Some(ref help) = t.help_text {
                        writeln!(f, "{}: {} ## {}", t.name, t.prerequisites.join(" "), help,)?;
                    } else {
                        writeln!(f, "{}: {}", t.name, t.prerequisites.join(" "))?;
                    }
                    for recipe in &t.recipe_lines {
                        writeln!(f, "{}", recipe)?;
                    }
                }
                MakefileElement::ManagedBlock(mb) => {
                    writeln!(f, "{} {} {}", MANAGED_START, mb.target_name, mb.template_id)?;
                    writeln!(f, "{}", mb.content)?;
                    writeln!(f, "{} {}", MANAGED_END, mb.target_name)?;
                }
                MakefileElement::Include(inc) => writeln!(f, "{}", inc)?,
                MakefileElement::Other(o) => writeln!(f, "{}", o)?,
            }
        }
        Ok(())
    }
}

fn try_parse_variable(line: &str) -> Option<(String, String, String)> {
    let operators = ["?=", ":=", "+=", "="];
    for op in &operators {
        if let Some(pos) = line.find(op) {
            let name = line[..pos].trim();
            if name.is_empty() || name.contains(':') || name.contains('\t') {
                continue;
            }
            if !name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
            {
                continue;
            }
            let value = line[pos + op.len()..].trim();
            return Some((name.to_string(), op.to_string(), value.to_string()));
        }
    }
    None
}

fn try_parse_target_header(line: &str) -> Option<(String, Vec<String>)> {
    if line.starts_with('\t') || line.starts_with(' ') {
        return None;
    }
    let line_no_help = if let Some(pos) = line.find("##") {
        &line[..pos]
    } else {
        line
    };
    let colon_pos = line_no_help.find(':')?;
    let name = line_no_help[..colon_pos].trim();
    if name.is_empty() || name.starts_with('.') || name.contains('=') {
        return None;
    }
    let after_colon = line_no_help[colon_pos + 1..].trim();
    let prerequisites: Vec<String> = if after_colon.is_empty() {
        Vec::new()
    } else {
        after_colon
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    };
    Some((name.to_string(), prerequisites))
}

fn extract_help_text(line: &str) -> Option<String> {
    let pos = line.find("## ")?;
    Some(line[pos + 3..].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let mf = Makefile::parse("").unwrap();
        assert!(mf.elements.is_empty());
    }

    #[test]
    fn parse_comment() {
        let mf = Makefile::parse("# This is a comment\n").unwrap();
        assert_eq!(mf.elements.len(), 1);
        assert!(
            matches!(&mf.elements[0], MakefileElement::Comment(c) if c == "# This is a comment")
        );
    }

    #[test]
    fn parse_variable() {
        let mf = Makefile::parse("CC := gcc\nCFLAGS ?= -Wall\n").unwrap();
        let vars = mf.variables();
        assert_eq!(vars.len(), 2);
        assert_eq!(vars[0].name, "CC");
        assert_eq!(vars[0].operator, ":=");
        assert_eq!(vars[0].value, "gcc");
        assert_eq!(vars[1].name, "CFLAGS");
        assert_eq!(vars[1].operator, "?=");
        assert_eq!(vars[1].value, "-Wall");
    }

    #[test]
    fn parse_phony() {
        let mf = Makefile::parse(".PHONY: test clean build\n").unwrap();
        let phonies = mf.phony_targets();
        assert_eq!(phonies, vec!["test", "clean", "build"]);
    }

    #[test]
    fn parse_target_with_recipe() {
        let input = "build: main.o utils.o\n\tgcc -o app main.o utils.o\n\techo done\n";
        let mf = Makefile::parse(input).unwrap();
        let targets = mf.targets();
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].name, "build");
        assert_eq!(targets[0].prerequisites, vec!["main.o", "utils.o"]);
        assert_eq!(targets[0].recipe_lines.len(), 2);
    }

    #[test]
    fn parse_phony_flag_on_targets() {
        let input = ".PHONY: test\ntest:\n\tpytest\n";
        let mf = Makefile::parse(input).unwrap();
        let targets = mf.targets();
        assert_eq!(targets.len(), 1);
        assert!(targets[0].is_phony);
    }

    #[test]
    fn parse_target_no_recipe() {
        let input = "all: build test\n";
        let mf = Makefile::parse(input).unwrap();
        let targets = mf.targets();
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].name, "all");
        assert!(targets[0].recipe_lines.is_empty());
    }

    #[test]
    fn parse_managed_block() {
        let input = "# MAKECTL MANAGED START test python/test\n.PHONY: test\ntest:\n\tpytest\n# MAKECTL MANAGED END test\n";
        let mf = Makefile::parse(input).unwrap();
        let blocks = mf.managed_blocks();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].target_name, "test");
        assert_eq!(blocks[0].template_id, "python/test");
    }

    #[test]
    fn parse_legacy_managed_block() {
        let input =
            "# MAKECTL MANAGED BLOCK INIT\nclean:\n\trm -rf build\n# MAKECTL MANAGED BLOCK END\n";
        let mf = Makefile::parse(input).unwrap();
        let blocks = mf.managed_blocks();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].target_name, "legacy");
    }

    #[test]
    fn unclosed_managed_block_errors() {
        let input = "# MAKECTL MANAGED START test python/test\ntest:\n\tpytest\n";
        let result = Makefile::parse(input);
        assert!(result.is_err());
    }

    #[test]
    fn parse_include() {
        let mf = Makefile::parse("include config.mk\n-include local.mk\n").unwrap();
        assert_eq!(mf.elements.len(), 2);
        assert!(matches!(&mf.elements[0], MakefileElement::Include(s) if s == "include config.mk"));
        assert!(matches!(&mf.elements[1], MakefileElement::Include(s) if s == "-include local.mk"));
    }

    #[test]
    fn parse_help_text() {
        let input = "test: ## Run the test suite\n\tpytest\n";
        let mf = Makefile::parse(input).unwrap();
        let targets = mf.targets();
        assert_eq!(targets[0].help_text.as_deref(), Some("Run the test suite"));
    }

    #[test]
    fn has_target_checks_both() {
        let input = "build:\n\tcargo build\n# MAKECTL MANAGED START test python/test\ntest:\n\tpytest\n# MAKECTL MANAGED END test\n";
        let mf = Makefile::parse(input).unwrap();
        assert!(mf.has_target("build"));
        assert!(mf.has_target("test"));
        assert!(!mf.has_target("clean"));
    }

    #[test]
    fn remove_managed_block_works() {
        let input = "build:\n\tcargo build\n# MAKECTL MANAGED START test python/test\ntest:\n\tpytest\n# MAKECTL MANAGED END test\n";
        let mut mf = Makefile::parse(input).unwrap();
        assert!(mf.remove_managed_block("test").is_ok());
        assert!(!mf.has_target("test"));
    }

    #[test]
    fn remove_nonexistent_block_errors() {
        let input = "build:\n\tcargo build\n";
        let mut mf = Makefile::parse(input).unwrap();
        assert!(mf.remove_managed_block("test").is_err());
    }

    #[test]
    fn multiple_targets_and_blank_lines() {
        let input = "CC := gcc\n\nbuild:\n\t$(CC) -o app main.c\n\nclean:\n\trm -f app\n";
        let mf = Makefile::parse(input).unwrap();
        let targets = mf.targets();
        assert_eq!(targets.len(), 2);
        assert_eq!(targets[0].name, "build");
        assert_eq!(targets[1].name, "clean");
        assert_eq!(mf.variables().len(), 1);
    }

    #[test]
    fn round_trip_simple() {
        let input = ".PHONY: test\n\ntest:\n\tpytest\n";
        let mf = Makefile::parse(input).unwrap();
        let output = mf.to_string();
        assert_eq!(output, input);
    }

    #[test]
    fn variable_not_confused_with_target() {
        let input = "FOO = bar:baz\n";
        let mf = Makefile::parse(input).unwrap();
        assert_eq!(mf.variables().len(), 1);
        assert_eq!(mf.targets().len(), 0);
    }
}
