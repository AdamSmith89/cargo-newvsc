use std::env;
use std::io::{self, Write};
use std::process::{exit, Command};
use std::fs::{canonicalize, create_dir, copy};

fn main() {
    let args = env::args();
    let args = args.skip(1).collect::<Vec<_>>();

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
                                        cargo_output.find("`").expect("Can't parse target path from cargo new output") + 1..
                                        cargo_output.rfind("`").expect("Can't parse target path from cargo new output")];
                    
                    let target_path = canonicalize(target_path).expect("Failed to resolve path");
                    let vscode_dir = target_path.to_str().unwrap().to_owned() + "\\.vscode";

                    println!("Creating .vscode dir at {}", vscode_dir);
                    create_dir(&vscode_dir).expect("Failed to create .vscode dir");

                    // TODO: Look into rust-embed for deploying the tasks.json file with the crate
                    println!("Deplopying tasks.json to {}", vscode_dir);
                    copy("../../res/tasks.json", vscode_dir + "\\tasks.json").expect("Failed to copy tasks.json");
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
