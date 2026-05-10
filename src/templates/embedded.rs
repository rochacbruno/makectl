pub const GENERIC_HELP: &str = include_str!("../../templates/generic/help.mk");
pub const GENERIC_CLEAN: &str = include_str!("../../templates/generic/clean.mk");
pub const GENERIC_DOCKER: &str = include_str!("../../templates/generic/docker.mk");
pub const GENERIC_DOCKER_COMPOSE: &str = include_str!("../../templates/generic/docker-compose.mk");
pub const GENERIC_GIT_HOOKS: &str = include_str!("../../templates/generic/git-hooks.mk");

pub const PYTHON_VENV: &str = include_str!("../../templates/python/venv.mk");
pub const PYTHON_TEST: &str = include_str!("../../templates/python/test.mk");
pub const PYTHON_LINT: &str = include_str!("../../templates/python/lint.mk");
pub const PYTHON_FORMAT: &str = include_str!("../../templates/python/format.mk");
pub const PYTHON_CLEAN: &str = include_str!("../../templates/python/clean.mk");
pub const PYTHON_PUBLISH: &str = include_str!("../../templates/python/publish.mk");

pub const RUST_BUILD: &str = include_str!("../../templates/rust/build.mk");
pub const RUST_TEST: &str = include_str!("../../templates/rust/test.mk");
pub const RUST_FMT: &str = include_str!("../../templates/rust/fmt.mk");
pub const RUST_CLIPPY: &str = include_str!("../../templates/rust/clippy.mk");
pub const RUST_RELEASE: &str = include_str!("../../templates/rust/release.mk");
pub const RUST_DOC: &str = include_str!("../../templates/rust/doc.mk");

pub const GO_BUILD: &str = include_str!("../../templates/go/build.mk");
pub const GO_TEST: &str = include_str!("../../templates/go/test.mk");
pub const GO_FMT: &str = include_str!("../../templates/go/fmt.mk");
pub const GO_VET: &str = include_str!("../../templates/go/vet.mk");
pub const GO_LINT: &str = include_str!("../../templates/go/lint.mk");
pub const GO_CLEAN: &str = include_str!("../../templates/go/clean.mk");

pub const NODE_INSTALL: &str = include_str!("../../templates/node/install.mk");
pub const NODE_BUILD: &str = include_str!("../../templates/node/build.mk");
pub const NODE_TEST: &str = include_str!("../../templates/node/test.mk");
pub const NODE_LINT: &str = include_str!("../../templates/node/lint.mk");
pub const NODE_FORMAT: &str = include_str!("../../templates/node/format.mk");
pub const NODE_DEV: &str = include_str!("../../templates/node/dev.mk");
pub const NODE_CLEAN: &str = include_str!("../../templates/node/clean.mk");

pub fn all_templates() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("generic", "help", GENERIC_HELP),
        ("generic", "clean", GENERIC_CLEAN),
        ("generic", "docker", GENERIC_DOCKER),
        ("generic", "docker-compose", GENERIC_DOCKER_COMPOSE),
        ("generic", "git-hooks", GENERIC_GIT_HOOKS),
        ("python", "venv", PYTHON_VENV),
        ("python", "test", PYTHON_TEST),
        ("python", "lint", PYTHON_LINT),
        ("python", "format", PYTHON_FORMAT),
        ("python", "clean", PYTHON_CLEAN),
        ("python", "publish", PYTHON_PUBLISH),
        ("rust", "build", RUST_BUILD),
        ("rust", "test", RUST_TEST),
        ("rust", "fmt", RUST_FMT),
        ("rust", "clippy", RUST_CLIPPY),
        ("rust", "release", RUST_RELEASE),
        ("rust", "doc", RUST_DOC),
        ("go", "build", GO_BUILD),
        ("go", "test", GO_TEST),
        ("go", "fmt", GO_FMT),
        ("go", "vet", GO_VET),
        ("go", "lint", GO_LINT),
        ("go", "clean", GO_CLEAN),
        ("node", "install", NODE_INSTALL),
        ("node", "build", NODE_BUILD),
        ("node", "test", NODE_TEST),
        ("node", "lint", NODE_LINT),
        ("node", "format", NODE_FORMAT),
        ("node", "dev", NODE_DEV),
        ("node", "clean", NODE_CLEAN),
    ]
}
