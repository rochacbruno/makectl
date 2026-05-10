use crate::parser::Makefile;

#[derive(Debug)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug)]
pub struct LintResult {
    pub rule: String,
    pub severity: Severity,
    pub line: Option<usize>,
    pub message: String,
    pub suggestion: Option<String>,
}

pub trait LintRule {
    fn name(&self) -> &str;
    fn check(&self, makefile: &Makefile, raw_content: &str) -> Vec<LintResult>;
}

pub struct MissingPhony;
pub struct SpacesInsteadOfTabs;
pub struct DuplicateTargets;
pub struct MissingHelpTarget;
pub struct EmptyRecipes;
pub struct HardcodedPaths;

const LIKELY_PHONY: &[&str] = &[
    "all", "test", "clean", "build", "lint", "fmt", "format", "help", "install", "run", "dev",
    "check", "deploy", "release", "publish", "doc", "docs", "coverage", "bench",
];

impl LintRule for MissingPhony {
    fn name(&self) -> &str {
        "missing-phony"
    }

    fn check(&self, makefile: &Makefile, _raw: &str) -> Vec<LintResult> {
        let phony_names = makefile.phony_targets();
        let mut results = Vec::new();

        for target in makefile.targets() {
            if LIKELY_PHONY.contains(&target.name.as_str()) && !phony_names.contains(&target.name) {
                results.push(LintResult {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    line: Some(target.line_number),
                    message: format!(
                        "Target '{}' is likely phony but not declared as .PHONY",
                        target.name
                    ),
                    suggestion: Some(format!("Add: .PHONY: {}", target.name)),
                });
            }
        }

        results
    }
}

impl LintRule for SpacesInsteadOfTabs {
    fn name(&self) -> &str {
        "spaces-instead-of-tabs"
    }

    fn check(&self, _makefile: &Makefile, raw: &str) -> Vec<LintResult> {
        let mut results = Vec::new();
        let mut in_recipe = false;

        for (i, line) in raw.lines().enumerate() {
            let line_num = i + 1;
            if !line.starts_with('\t')
                && !line.starts_with(' ')
                && line.contains(':')
                && !line.starts_with('#')
                && !line.starts_with('.')
            {
                in_recipe = true;
                continue;
            }
            if in_recipe {
                if (line.starts_with("    ") || line.starts_with("  "))
                    && !line.trim_start().starts_with('#')
                {
                    results.push(LintResult {
                        rule: self.name().to_string(),
                        severity: Severity::Error,
                        line: Some(line_num),
                        message: "Recipe line uses spaces instead of tabs".to_string(),
                        suggestion: Some("Replace leading spaces with a tab character".to_string()),
                    });
                }
                if line.trim().is_empty() || (!line.starts_with('\t') && !line.starts_with(' ')) {
                    in_recipe = false;
                }
            }
        }

        results
    }
}

impl LintRule for DuplicateTargets {
    fn name(&self) -> &str {
        "duplicate-targets"
    }

    fn check(&self, makefile: &Makefile, _raw: &str) -> Vec<LintResult> {
        let mut seen: Vec<(String, usize)> = Vec::new();
        let mut results = Vec::new();

        for target in makefile.targets() {
            if let Some((_, first_line)) = seen.iter().find(|(name, _)| *name == target.name) {
                results.push(LintResult {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    line: Some(target.line_number),
                    message: format!(
                        "Duplicate target '{}' (first defined at line {})",
                        target.name, first_line
                    ),
                    suggestion: Some("Remove or rename one of the duplicate targets".to_string()),
                });
            } else {
                seen.push((target.name.clone(), target.line_number));
            }
        }

        results
    }
}

impl LintRule for MissingHelpTarget {
    fn name(&self) -> &str {
        "missing-help-target"
    }

    fn check(&self, makefile: &Makefile, _raw: &str) -> Vec<LintResult> {
        if makefile.has_target("help") {
            return Vec::new();
        }

        vec![LintResult {
            rule: self.name().to_string(),
            severity: Severity::Info,
            line: None,
            message: "No 'help' target found".to_string(),
            suggestion: Some(
                "Add a self-documenting help target: makectl add generic/help".to_string(),
            ),
        }]
    }
}

impl LintRule for EmptyRecipes {
    fn name(&self) -> &str {
        "empty-recipes"
    }

    fn check(&self, makefile: &Makefile, _raw: &str) -> Vec<LintResult> {
        let mut results = Vec::new();

        for target in makefile.targets() {
            if target.recipe_lines.is_empty() && target.prerequisites.is_empty() {
                results.push(LintResult {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    line: Some(target.line_number),
                    message: format!(
                        "Target '{}' has no recipe and no prerequisites",
                        target.name
                    ),
                    suggestion: Some("Add recipe lines or prerequisites".to_string()),
                });
            }
        }

        results
    }
}

impl LintRule for HardcodedPaths {
    fn name(&self) -> &str {
        "hardcoded-paths"
    }

    fn check(&self, _makefile: &Makefile, raw: &str) -> Vec<LintResult> {
        let prefixes = ["/usr/", "/opt/", "/home/", "/etc/", "/var/"];
        let mut results = Vec::new();

        for (i, line) in raw.lines().enumerate() {
            let line_num = i + 1;
            if line.starts_with('#') {
                continue;
            }
            for prefix in &prefixes {
                if line.contains(prefix) {
                    results.push(LintResult {
                        rule: self.name().to_string(),
                        severity: Severity::Info,
                        line: Some(line_num),
                        message: format!("Hardcoded path '{}' found", prefix),
                        suggestion: Some("Consider using a variable instead".to_string()),
                    });
                    break;
                }
            }
        }

        results
    }
}

pub fn all_rules() -> Vec<Box<dyn LintRule>> {
    vec![
        Box::new(MissingPhony),
        Box::new(SpacesInsteadOfTabs),
        Box::new(DuplicateTargets),
        Box::new(MissingHelpTarget),
        Box::new(EmptyRecipes),
        Box::new(HardcodedPaths),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_phony_detects_unphonied_test() {
        let input = "test:\n\tpytest\n";
        let mf = Makefile::parse(input).unwrap();
        let results = MissingPhony.check(&mf, input);
        assert_eq!(results.len(), 1);
        assert!(results[0].message.contains("test"));
    }

    #[test]
    fn missing_phony_passes_when_declared() {
        let input = ".PHONY: test\ntest:\n\tpytest\n";
        let mf = Makefile::parse(input).unwrap();
        let results = MissingPhony.check(&mf, input);
        assert!(results.is_empty());
    }

    #[test]
    fn spaces_instead_of_tabs_detects_spaces() {
        let input = "build:\n    gcc main.c\n";
        let mf = Makefile::parse(input).unwrap();
        let results = SpacesInsteadOfTabs.check(&mf, input);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn duplicate_targets_detects() {
        let input = "build:\n\tcargo build\nbuild:\n\tcargo build --release\n";
        let mf = Makefile::parse(input).unwrap();
        let results = DuplicateTargets.check(&mf, input);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn empty_recipes_detects() {
        let input = "foo:\n\nbar:\n\techo bar\n";
        let mf = Makefile::parse(input).unwrap();
        let results = EmptyRecipes.check(&mf, input);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn hardcoded_paths_detects() {
        let input = "install:\n\tcp app /usr/local/bin/\n";
        let mf = Makefile::parse(input).unwrap();
        let results = HardcodedPaths.check(&mf, input);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn hardcoded_paths_ignores_comments() {
        let input = "# /usr/local/bin is the target\ninstall:\n\tcp app $(DESTDIR)\n";
        let mf = Makefile::parse(input).unwrap();
        let results = HardcodedPaths.check(&mf, input);
        assert!(results.is_empty());
    }

    #[test]
    fn missing_help_target_reports() {
        let input = "build:\n\tcargo build\n";
        let mf = Makefile::parse(input).unwrap();
        let results = MissingHelpTarget.check(&mf, input);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn missing_help_target_passes() {
        let input = "help:\n\techo help\n";
        let mf = Makefile::parse(input).unwrap();
        let results = MissingHelpTarget.check(&mf, input);
        assert!(results.is_empty());
    }
}
