pub mod embedded;

#[derive(Debug, Clone)]
pub struct Template {
    pub id: String,
    pub language: String,
    pub name: String,
    pub description: String,
    pub content: String,
}

pub struct TemplateRegistry {
    templates: Vec<Template>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        let mut templates = Vec::new();
        for (lang, name, raw_content) in embedded::all_templates() {
            let (description, content) = parse_template_content(raw_content);
            templates.push(Template {
                id: format!("{}/{}", lang, name),
                language: lang.to_string(),
                name: name.to_string(),
                description,
                content,
            });
        }
        TemplateRegistry { templates }
    }

    pub fn get(&self, id: &str) -> Option<&Template> {
        self.templates.iter().find(|t| t.id == id)
    }

    pub fn list_all(&self) -> &[Template] {
        &self.templates
    }

    pub fn list_by_language(&self, lang: &str) -> Vec<&Template> {
        self.templates
            .iter()
            .filter(|t| t.language == lang)
            .collect()
    }

    #[allow(dead_code)]
    pub fn languages(&self) -> Vec<&str> {
        let mut langs: Vec<&str> = self.templates.iter().map(|t| t.language.as_str()).collect();
        langs.sort();
        langs.dedup();
        langs
    }
}

fn parse_template_content(raw: &str) -> (String, String) {
    let mut lines = raw.lines();
    let first_line = lines.next().unwrap_or("");

    if let Some(rest) = first_line.strip_prefix("## makectl: description=\"") {
        let description = rest.trim_end_matches('"').to_string();
        let content: String = lines.collect::<Vec<&str>>().join("\n");
        let content = content.trim_start_matches('\n').to_string();
        (description, content)
    } else {
        (String::new(), raw.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_loads_all_templates() {
        let reg = TemplateRegistry::new();
        assert!(reg.list_all().len() >= 30);
    }

    #[test]
    fn get_python_test() {
        let reg = TemplateRegistry::new();
        let t = reg.get("python/test").unwrap();
        assert_eq!(t.language, "python");
        assert_eq!(t.name, "test");
        assert!(!t.description.is_empty());
        assert!(t.content.contains("pytest"));
    }

    #[test]
    fn get_nonexistent_returns_none() {
        let reg = TemplateRegistry::new();
        assert!(reg.get("nonexistent/foo").is_none());
    }

    #[test]
    fn list_by_language_filters() {
        let reg = TemplateRegistry::new();
        let rust_templates = reg.list_by_language("rust");
        assert!(rust_templates.len() >= 6);
        assert!(rust_templates.iter().all(|t| t.language == "rust"));
    }

    #[test]
    fn languages_returns_all() {
        let reg = TemplateRegistry::new();
        let langs = reg.languages();
        assert!(langs.contains(&"generic"));
        assert!(langs.contains(&"python"));
        assert!(langs.contains(&"rust"));
        assert!(langs.contains(&"go"));
        assert!(langs.contains(&"node"));
    }

    #[test]
    fn all_templates_have_descriptions() {
        let reg = TemplateRegistry::new();
        for t in reg.list_all() {
            assert!(
                !t.description.is_empty(),
                "Template {} has no description",
                t.id
            );
        }
    }

    #[test]
    fn all_templates_have_phony() {
        let reg = TemplateRegistry::new();
        for t in reg.list_all() {
            assert!(
                t.content.contains(".PHONY:"),
                "Template {} missing .PHONY declaration",
                t.id
            );
        }
    }
}
