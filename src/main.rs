use clap::Parser;
use inquire::Confirm;
use reqwest::StatusCode;
use std::{
    env::current_dir,
    fs::write,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

const BASE_URL: &str = "https://www.toptal.com";

/// A simple cli for .gitignore generator
#[derive(Debug, Parser)]
struct Args {
    /// Template name, e.g node or node,python for more templates
    #[arg(short, long)]
    template: String,

    /// File name for output
    #[arg(short, long, default_value_t = String::from(".gitignore"))]
    filename: String,
}

fn main() {
    let cli_args = Args::parse();
    let templates_content: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handlers = vec![];

    println!("Getting templates...");

    for tpl in cli_args.template.split(',') {
        let template = String::from(tpl);
        let templates_content = Arc::clone(&templates_content);

        let handler = thread::spawn(move || {
            let content = get_template(template).unwrap();
            let mut template_content = templates_content.lock().unwrap();

            template_content.push(content);
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    create_file(
        cli_args.filename,
        templates_content.lock().unwrap().join("\n\n"),
    )
    .unwrap();
}

fn create_file(filename: String, content: String) -> Result<(), &'static str> {
    let mut cwd = current_dir().expect("Error on get current dir");
    cwd.push(&filename);

    if cwd.exists() {
        let confirm_file_overwrite = Confirm::new(
            format!(
                "The '{}' file already exists, do you want to overwrite it?",
                &filename
            )
            .as_str(),
        )
        .with_default(false)
        .prompt();

        match confirm_file_overwrite {
            Ok(true) => {
                write_file(cwd, content)?;
                return Ok(());
            }
            Ok(false) => {
                return Ok(());
            }
            Err(_) => {}
        }
    }

    Ok(())
}

fn write_file(path: PathBuf, content: String) -> Result<(), &'static str> {
    write(path, content).expect("Error on write file");
    Ok(())
}

fn get_template(template: String) -> Result<String, &'static str> {
    let url = format!("{}/developers/gitignore/api/{}", BASE_URL, template);

    match reqwest::blocking::get(url) {
        Ok(response) => {
            if response.status() == StatusCode::NOT_FOUND {
                return Err("Template not found");
            }

            match response.text() {
                Ok(content) => {
                    let lines: Vec<&str> = content.lines().collect();
                    let result: Vec<&str> = lines[3..].to_vec();

                    Ok(result.join("\n"))
                }
                Err(_) => Err("Error on get template text"),
            }
        }
        Err(_) => Err("Error on request template"),
    }
}
