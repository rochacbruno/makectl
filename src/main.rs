mod config;

// use serde;
use serde_json;
// use serde_derive;

use structopt::StructOpt;
use std::process::Command;
use std::io::{self, Write};

use crate::config::Command as CmdArgs;
use crate::config::Config;
use std::fs;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;

fn main() {
    let conf = Config::from_args();
    let project_dir = ProjectDirs::from("com", "rochacbruno", "makectl").unwrap();
    let project_dir = project_dir.config_dir();
    match conf.command {
        CmdArgs::Add(ref add_params) => {
            add_template_to_makefile(&add_params.templates, project_dir.join("makectl"));
            println!("Added templates {:?} to Makefile", add_params.templates);
        }
        CmdArgs::Update => {
            let makectl_dir = update_templates_from_github(project_dir)
                .map_err(|e| eprintln!("unable to update templates from github: {:?}", e))
                .unwrap();
            println!("updated templates at {:?}", makectl_dir);
        }
    }
}

const GITHUB_REPO_URL: &str = "https://github.com/rochacbruno/makectl";

fn update_templates_from_github(project_dir: &Path) -> std::io::Result<PathBuf> {
    if project_dir.join("makectl").metadata().is_ok() {
        let output = Command::new("git")
            .arg("pull")
            .current_dir(project_dir.join("makectl"))
            .output()
            .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));
        io::stdout().write_all(&output.stdout)?;
    } else {
        std::fs::create_dir_all(project_dir)?;

        let output = Command::new("git")
            .arg("clone")
            .arg(GITHUB_REPO_URL)
            .current_dir(project_dir)
            .output()?;
        io::stdout().write_all(&output.stdout)?;
    }

    return Ok(project_dir.join("makectl"));
}

fn get_json_from_templates_file(templates_path: &str) -> serde_json::Value {
    let file = fs::File::open(templates_path)
        .expect(format!("Cannot open templates file in path {}", templates_path).as_ref());
    return serde_json::from_reader(file)
        .expect(format!("File {} is not proper JSON format", templates_path).as_ref());
}

fn add_template_to_makefile(templates: &Vec<String>, project_dir: PathBuf) {
    let templates_path = project_dir.join("templates/templates.json");
    let json = get_json_from_templates_file(templates_path.to_str().unwrap());
    let mut file_handler = OpenOptions::new()
        .write(true)
        .append(true)
        .open("Makefile")
        .unwrap();
    let mut data_to_write_to_file = String::new();
    for template_key in templates.iter() {
        let template = json.get(template_key)
            .expect(format!("Templates file does not have key {}", template_key).as_ref());
        if template["template-file"].is_string() {
            let template_file_name = template["template-file"].as_str().unwrap();
            let template_file_path = project_dir.join("templates/").join(template_file_name);
            data_to_write_to_file = fs::read_to_string(&template_file_path)
                .expect(format!("The file {:?} could not be read", &template_file_path).as_ref());
        }
        else if template["template-lines"].is_array() {
            let template_lines_array = template["template-lines"].as_array().unwrap();
            for template_line in template_lines_array.iter() {
                let template_line_as_string = format!("{}\n", template_line.as_str().unwrap());
                data_to_write_to_file.push_str(&template_line_as_string);
            }
        }

        data_to_write_to_file = format!("\n{}\n", data_to_write_to_file);

        if let Err(e) = writeln!(file_handler, "{}", &data_to_write_to_file) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
