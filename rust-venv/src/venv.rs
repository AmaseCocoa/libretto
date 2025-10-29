use std::path::{Path, PathBuf};

pub fn find_venv_path(project_root: &Path) -> Option<std::path::PathBuf> {
    let venv_names = ["venv", ".venv"];
    for name in venv_names.iter() {
        let venv_path = project_root.join(name);
        if venv_path.join("bin").exists() || venv_path.join("Scripts").exists() {
            return Some(venv_path);
        }
    }
    None
}

pub fn get_venv_bin_dir(venv_path: &Path) -> PathBuf {
    if cfg!(target_os = "windows") {
        venv_path.join("Scripts")
    } else {
        venv_path.join("bin")
    }
}