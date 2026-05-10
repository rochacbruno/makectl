pub mod rules;

use crate::parser::Makefile;
use rules::{all_rules, LintResult};

pub fn lint(makefile: &Makefile, raw_content: &str) -> Vec<LintResult> {
    all_rules()
        .iter()
        .flat_map(|rule| rule.check(makefile, raw_content))
        .collect()
}
