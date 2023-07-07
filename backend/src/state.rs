use clap::Parser;
use models;
use models::{ast::*, change::*};
use std::io::{Read, Write};
use std::{path::PathBuf, sync::Mutex};

const FILE_NAME: &str = "data.json";

#[derive(Parser, Debug, Clone)]
pub struct Args {
    /// Run in development mode
    #[cfg(debug_assertions)]
    #[arg(short, long)]
    pub dev: bool,

    #[arg(short = 'P', long, default_value = "8080")]
    pub port: u16,

    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    pub host: String,

    #[arg(short = 'p', long, default_value = "./")]
    pub project_path: PathBuf,

    /// Run in IO mode (default is HTTP mode)
    #[arg(short, long)]
    pub io_mode: bool,
}

fn load_struct_from_file(path: &PathBuf) -> std::io::Result<Vec<Node>> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let my_struct: Vec<Node> = serde_json::from_str(&contents)?;
    Ok(my_struct)
}

pub struct State {
    pub project_path: Mutex<PathBuf>,
    pub ast: Mutex<Vec<Node>>,
}

impl State {
    pub fn init() -> std::io::Result<(Self, String, u16, bool)> {
        let Args {
            mut project_path,
            host,
            port,
            #[cfg(debug_assertions)]
            dev,
            io_mode,
        } = Args::parse();

        #[cfg(debug_assertions)]
        {
            if dev {
                // println!(
                //     "{}",
                //     serde_json::to_string(&examples::ast_example()).unwrap()
                // );
                // examples::request_json();
                // examples::response_json();
                std::process::exit(0);
            }
        }

        // Ensure the project path is valid and points to an existing file
        Self::prepare_project_path(&mut project_path)?;

        let ast = load_struct_from_file(&project_path)?;

        Ok((
            State {
                project_path: Mutex::new(project_path),
                ast: Mutex::new(ast),
            },
            host,
            port,
            io_mode,
        ))
    }

    fn prepare_project_path(project_path: &mut std::path::PathBuf) -> std::io::Result<()> {
        if project_path.is_relative() {
            *project_path = std::env::current_dir()?.join(project_path.as_path());
            *project_path = std::fs::canonicalize(project_path.clone())?;
        }

        let file_needs_creation = Self::update_project_path_if_needed(project_path)?;

        if file_needs_creation {
            let mut file = std::fs::File::create(project_path)?;
            file.write_all(b"[]")?;
        }

        Ok(())
    }

    fn update_project_path_if_needed(
        project_path: &mut std::path::PathBuf,
    ) -> std::io::Result<bool> {
        if project_path.ends_with(FILE_NAME) {
            Ok(!project_path.exists())
        } else if project_path.is_dir() {
            project_path.push(FILE_NAME);
            Ok(!project_path.exists())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "The path {} is not a valid file or directory",
                    project_path
                        .to_str()
                        .unwrap_or("[The path could not be converted to string]")
                ),
            ))
        }
    }
}
