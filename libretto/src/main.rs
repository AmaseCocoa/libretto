mod pyproject;

use clap::Parser;
use pyproject::{PyProjectToml, TaskValue};
use std::collections::{HashMap};
use std::fs;
use std::env::consts;
use rust_venv::exec;

use crate::pyproject::CommandElement;

#[derive(Parser, Debug)]
struct Cli {
    #[arg()]
    pub all_args: Vec<String>,
}

fn vec_string_to_slice_of_str(vec_string: &Vec<String>) -> Vec<&str> {
    vec_string.iter()
              .map(|s| s.as_str())
              .collect()
}

fn main() {
    let cli = Cli::parse();
    let mut args_iter = cli.all_args.into_iter();

    let content = fs::read_to_string("pyproject.toml").expect("could not read file");
    let pyproject = PyProjectToml::new(&content);
    match pyproject {
        Ok(config) => {
            let tool = config.tool.unwrap();
            let tasks: &HashMap<String, TaskValue> = &tool.libretto.unwrap().tasks;
            if let Some(command) = args_iter.next() {
                let cmd_string = command.as_str();
                let cmd = tasks.get(cmd_string);
                match cmd {
                    Some(result) => {
                        match result {
                            TaskValue::String(_s) => {
                                let mut parts = _s.split_whitespace();
                                let command = parts.next().unwrap_or("");
                                let mut command_args: Vec<&str> = parts.collect();

                                let additional_args: Vec<String> = args_iter.collect();
                                let additional_args_slice = vec_string_to_slice_of_str(&additional_args);
                                command_args.extend(additional_args_slice);

                                let args_slice: &[&str] = command_args.as_slice();
                                let res = exec(command, args_slice, None);
                                match res {
                                    Ok(_r) => {},
                                    Err(_r) => {}
                                }
                            },
                            TaskValue::Array(_a) => {
                                for _s in _a {
                                    match _s {
                                        CommandElement::SingleString(s) => {
                                            let mut parts = s.split_whitespace();
                                            let command = parts.next().unwrap_or("");
                                            let command_args: Vec<&str> = parts.collect();
                                            let args_slice: &[&str] = command_args.as_slice();
                                            let res = exec(command, args_slice, None);
                                            match res {
                                                Ok(_r) => {},
                                                Err(_r) => {}
                                            }
                                        },
                                        CommandElement::CommandObject(c) => {
                                            if c.platforms.is_empty() || c.platforms.contains(&consts::OS.to_string()) {
                                                let mut parts = c.cmd.split_whitespace();
                                                let command = parts.next().unwrap_or("");
                                                let command_args: Vec<&str> = parts.collect();
                                                let args_slice: &[&str] = command_args.as_slice();
                                                let res = exec(command, args_slice, None);
                                                match res {
                                                    Ok(_r) => {},
                                                    Err(_r) => {}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    _ => {
                        println!("Libretto v{}", env!("CARGO_PKG_VERSION"));
                        eprintln!("Error: No such task: {}\n", cmd_string);
                        println!("Avaliable tasks:");
                        
                        for key in tasks.keys() {
                            println!("  {}", key);
                        }
                    }
                }

            } else {
                println!("Libretto v{}\n", env!("CARGO_PKG_VERSION"));
                
                println!("Avaliable tasks:");
                
                for key in tasks.keys() {
                    println!("  {}", key);
                }
            }
        },
        Err(e) => {
            eprintln!("❌ Failed to parse pyproject.toml: {}", e);
        }
    }


}
