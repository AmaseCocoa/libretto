use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

mod dir;
mod venv;
mod errors;

fn run_command_in_venv(
    script_path: &str,
    args: &[&str],
    path: std::ffi::OsString,
) -> Result<(), errors::CommandError> {
    let status = Command::new(script_path)
        .args(args)
        .env("PATH", path) 
        .stdout(Stdio::inherit())
        .status()
        .map_err(|e| errors::CommandError::IoError { source: e })?;

    if status.success() {
        Ok(())
    } else {

        match status.code() {
            Some(code) => {
                Err(errors::CommandError::NonZeroExit {
                    code,
                })
            },
            _ => {
                Err(errors::CommandError::NoExitCode {})
            }
        }
    }
}

fn find_and_execute(command: &str, args: &[&str]) -> Result<(), errors::CommandError> {
    let project_root = env::current_dir().map_err(|e| errors::CommandError::IoError { source: e })?;
    let venv_loc = venv::find_venv_path(&project_root);
    match venv_loc {
        Some(p) => {
            let path = build_path(&p);
            run_command_in_venv(command, args, path)
        }
        _ => {
            Err(errors::CommandError::NoVenv {})
        }
    }
}

fn build_path(venv_path: &Path) -> std::ffi::OsString {
    let venv_bin_dir = venv::get_venv_bin_dir(venv_path);
    let current_path = env::var_os("PATH").unwrap_or_default();

    let path_delimiter = if cfg!(target_os = "windows") {
        ";"
    } else {
        ":"
    };

    {
        let mut path_os_string = venv_bin_dir.into_os_string();
        path_os_string.push(path_delimiter);
        path_os_string.push(current_path);
        path_os_string
    }
}

pub fn exec(command: &str, args: &[&str], venv_dir: Option<String>) -> Result<(), errors::CommandError> {
    match venv_dir {
        Some(dir) => {
            let venv_full_path = dir::make_relative_to_current_dir(&dir).map_err(|e| errors::CommandError::IoError { source: e })?;
            let path = build_path(&venv_full_path);
            run_command_in_venv(command, args, path)
        }
        _ => find_and_execute(command, args),
    }
}
