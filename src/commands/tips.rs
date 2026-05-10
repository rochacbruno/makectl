use anyhow::Result;
use colored::Colorize;

const TIPS: &[(&str, &str)] = &[
    (
        "Use .PHONY for non-file targets",
        "Declare targets that don't produce files as .PHONY to avoid conflicts\nwith files of the same name and ensure they always run.\n  .PHONY: test clean build",
    ),
    (
        "Add self-documenting help",
        "Use ## comments after target names and a help target that greps them:\n  help:\n  \t@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | ...\nRun 'makectl add generic/help' to add one automatically.",
    ),
    (
        "Use ?= for overridable variables",
        "Let users override defaults without editing the Makefile:\n  PYTHON ?= python3\n  CC ?= gcc\nUsers can then run: make PYTHON=python3.12 test",
    ),
    (
        "Use $(MAKE) for recursive calls",
        "Never hardcode 'make' in recipes. Use $(MAKE) so flags and\njob-server state are inherited:\n  build-all:\n  \t$(MAKE) -C subproject build",
    ),
    (
        "Use @ prefix to suppress echo",
        "Prefix recipe lines with @ to suppress printing the command:\n  clean:\n  \t@rm -rf build/\nUseful for cleaner output on simple commands.",
    ),
    (
        "Set .DEFAULT_GOAL",
        "Explicitly set the default target instead of relying on first-target:\n  .DEFAULT_GOAL := help\nThis makes the Makefile self-documenting when run without arguments.",
    ),
    (
        "Use -include for optional files",
        "Use -include instead of include for optional config files.\nIt won't error if the file is missing:\n  -include .env.mk",
    ),
    (
        "Keep recipes simple",
        "If a recipe is more than 5-10 lines, extract it into a script\nfile and call that from the target:\n  deploy:\n  \t./scripts/deploy.sh $(ENV)",
    ),
    (
        "Use tabs, not spaces",
        "GNU make requires literal tab characters for recipe indentation.\nSpaces will cause 'missing separator' errors.\nConfigure your editor to insert tabs in Makefiles.",
    ),
    (
        "Group related targets",
        "Keep related targets together and separate groups with comments:\n  # --- Build ---\n  build: ...\n  build-release: ...\n  # --- Test ---\n  test: ...",
    ),
];

pub fn execute() -> Result<()> {
    println!("{}\n", "Makefile Best Practices".bold().underline());

    for (i, (title, body)) in TIPS.iter().enumerate() {
        println!("{}. {}", (i + 1).to_string().cyan().bold(), title.bold());
        for line in body.lines() {
            println!("   {}", line);
        }
        println!();
    }

    println!(
        "Use {} to check your Makefile against these practices.",
        "makectl lint".cyan()
    );

    Ok(())
}
