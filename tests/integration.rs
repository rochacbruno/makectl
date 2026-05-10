use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn makectl() -> Command {
    Command::cargo_bin("makectl").unwrap()
}

#[test]
fn init_creates_makefile() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");

    makectl()
        .args(["init", "--file", makefile.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));

    let content = fs::read_to_string(&makefile).unwrap();
    assert!(content.contains(".DEFAULT_GOAL := help"));
    assert!(content.contains("MAKECTL MANAGED START help"));
}

#[test]
fn init_with_lang_python() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");

    makectl()
        .args([
            "init",
            "--file",
            makefile.to_str().unwrap(),
            "--lang",
            "python",
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&makefile).unwrap();
    assert!(content.contains("pytest"));
    assert!(content.contains("venv"));
}

#[test]
fn init_with_lang_rust() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");

    makectl()
        .args([
            "init",
            "--file",
            makefile.to_str().unwrap(),
            "--lang",
            "rust",
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&makefile).unwrap();
    assert!(content.contains("cargo build"));
    assert!(content.contains("cargo test"));
    assert!(content.contains("cargo clippy"));
}

#[test]
fn init_refuses_overwrite() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");
    fs::write(&makefile, "existing content").unwrap();

    makectl()
        .args(["init", "--file", makefile.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--force"));
}

#[test]
fn init_force_overwrites() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");
    fs::write(&makefile, "old content").unwrap();

    makectl()
        .args(["init", "--file", makefile.to_str().unwrap(), "--force"])
        .assert()
        .success();

    let content = fs::read_to_string(&makefile).unwrap();
    assert!(content.contains(".DEFAULT_GOAL"));
    assert!(!content.contains("old content"));
}

#[test]
fn add_template_to_makefile() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");
    fs::write(&makefile, ".PHONY: help\nhelp:\n\t@echo help\n").unwrap();

    makectl()
        .args(["add", "python/test", "--file", makefile.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("python/test"));

    let content = fs::read_to_string(&makefile).unwrap();
    assert!(content.contains("MAKECTL MANAGED START test python/test"));
    assert!(content.contains("pytest"));
    assert!(content.contains("MAKECTL MANAGED END test"));
}

#[test]
fn add_duplicate_skips() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");

    makectl()
        .args(["init", "--file", makefile.to_str().unwrap()])
        .assert()
        .success();

    makectl()
        .args(["add", "python/test", "--file", makefile.to_str().unwrap()])
        .assert()
        .success();

    let content_after_first = fs::read_to_string(&makefile).unwrap();

    makectl()
        .args(["add", "python/test", "--file", makefile.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("already exists"));

    let content_after_second = fs::read_to_string(&makefile).unwrap();
    assert_eq!(content_after_first, content_after_second);
}

#[test]
fn add_unknown_template_fails() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");
    fs::write(&makefile, "").unwrap();

    makectl()
        .args([
            "add",
            "nonexistent/foo",
            "--file",
            makefile.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn add_no_makefile_fails() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");

    makectl()
        .args(["add", "python/test", "--file", makefile.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn remove_managed_target() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");

    makectl()
        .args(["init", "--file", makefile.to_str().unwrap()])
        .assert()
        .success();

    makectl()
        .args(["add", "python/test", "--file", makefile.to_str().unwrap()])
        .assert()
        .success();

    makectl()
        .args(["remove", "test", "--file", makefile.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed"));

    let content = fs::read_to_string(&makefile).unwrap();
    assert!(!content.contains("pytest"));
    assert!(content.contains("help"));
}

#[test]
fn remove_nonmanaged_warns() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");
    fs::write(&makefile, "build:\n\tcargo build\n").unwrap();

    makectl()
        .args(["remove", "build", "--file", makefile.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("not a managed target"));
}

#[test]
fn list_templates() {
    makectl()
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("python/test"))
        .stdout(predicate::str::contains("rust/build"))
        .stdout(predicate::str::contains("go/test"))
        .stdout(predicate::str::contains("node/build"));
}

#[test]
fn list_templates_filtered() {
    makectl()
        .args(["list", "--lang", "rust"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rust/build"))
        .stdout(predicate::str::contains("rust/test"));
}

#[test]
fn list_targets_in_makefile() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");

    makectl()
        .args([
            "init",
            "--file",
            makefile.to_str().unwrap(),
            "--lang",
            "python",
        ])
        .assert()
        .success();

    makectl()
        .args(["list", "--targets", "--file", makefile.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("help"))
        .stdout(predicate::str::contains("managed"));
}

#[test]
fn lint_clean_makefile() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");

    makectl()
        .args(["init", "--file", makefile.to_str().unwrap()])
        .assert()
        .success();

    makectl()
        .args(["lint", "--file", makefile.to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn validate_valid_makefile() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");
    fs::write(&makefile, ".PHONY: test\ntest:\n\tpytest\n").unwrap();

    makectl()
        .args(["validate", "--file", makefile.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("valid"));
}

#[test]
fn fmt_check_formatted() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");
    fs::write(&makefile, ".PHONY: test\n\ntest:\n\tpytest\n").unwrap();

    makectl()
        .args(["fmt", "--check", "--file", makefile.to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn tips_outputs_content() {
    makectl()
        .args(["tips"])
        .assert()
        .success()
        .stdout(predicate::str::contains(".PHONY"))
        .stdout(predicate::str::contains("Best Practices"));
}

#[test]
fn completions_bash() {
    makectl()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn completions_zsh() {
    makectl()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn full_workflow() {
    let dir = TempDir::new().unwrap();
    let makefile = dir.path().join("Makefile");

    makectl()
        .args(["init", "--file", makefile.to_str().unwrap()])
        .assert()
        .success();

    makectl()
        .args(["add", "python/test", "--file", makefile.to_str().unwrap()])
        .assert()
        .success();

    makectl()
        .args(["add", "rust/build", "--file", makefile.to_str().unwrap()])
        .assert()
        .success();

    makectl()
        .args(["list", "--file", makefile.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("help"))
        .stdout(predicate::str::contains("test"))
        .stdout(predicate::str::contains("build"));

    makectl()
        .args(["lint", "--file", makefile.to_str().unwrap()])
        .assert()
        .success();

    makectl()
        .args(["remove", "test", "--file", makefile.to_str().unwrap()])
        .assert()
        .success();

    let content = fs::read_to_string(&makefile).unwrap();
    assert!(!content.contains("pytest"));
    assert!(content.contains("cargo build"));
}

#[test]
fn help_flag_works() {
    makectl()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Generate and manage targets in your Makefiles",
        ));
}

#[test]
fn version_flag_works() {
    makectl()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("makectl"));
}
