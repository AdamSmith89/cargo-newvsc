use std::env;
use std::io::{self, Write};
use std::process::{exit, Command};
use std::fs::{canonicalize, create_dir, File};

#[macro_use]
extern crate rust_embed;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;

fn main() {
    let args = env::args();
    let args = args.skip(2).collect::<Vec<_>>();

    let mut command: Vec<_> = args.iter().map(|s| s.trim()).collect();
    command.insert(0, "new");

    let output = Command::new("cargo")
        .args(&command)
        .output()
        .expect("Failed to execute cargo new");

    // Cargo seems to output everything to stderr - so print for the user
    io::stderr().write_all(&output.stderr).unwrap();

    match output.status.code() {
        Some(code) => {
            match code {
                0 => {
                    // Kinda dodgy, but parse the project directory from the cargo new output stream
                    // It *should* look like "Created ... `project_dir` package"
                    let cargo_output = String::from_utf8_lossy(&output.stderr).to_owned();
                    let target_path = &cargo_output[
                                        cargo_output.find('`').expect("Failed to parse target path from cargo new output") + 1..
                                        cargo_output.rfind('`').expect("Failed to parse target path from cargo new output")];
                    let target_path = canonicalize(target_path).expect("Failed to resolve path");
                    
                    let vscode_dir = target_path.join(".vscode");
                    println!("Creating .vscode dir at {}", vscode_dir.display());
                    create_dir(&vscode_dir).expect("Failed to create .vscode dir");

                    println!("Deplopying tasks.json to {}", vscode_dir.display());
                    let tasks_asset = Asset::get("tasks.json").expect("Failed to extract tasks.json asset");
                    let tasks_asset = std::str::from_utf8(tasks_asset.as_ref()).expect("Failed to convert tasks.json into utf-8 string");

                    let tasks_json_path = vscode_dir.join("tasks.json");
                    let mut buffer = File::create(tasks_json_path).expect("Failed to create tasks.json");
                    buffer.write_all(tasks_asset.as_bytes()).expect("Failed to write to tasks.json");
                }
                _ => {
                    exit(code);
                }
            }
        }
        None => {
            exit(127);
        }
    }

    exit(0);
}
