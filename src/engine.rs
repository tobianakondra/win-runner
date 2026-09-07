use std::process::Command;
use std::path::{Path, PathBuf};

pub fn lancer_installation(prefix_path: PathBuf, app_path: &Path) -> Result<(), String>{
    Command::new("wine")
        .env("WINEPREFIX", prefix_path)
        .arg(app_path)
        .status() // je prefere utiliser status car il montre les resutat dans le terminal contrairement a output
        .expect("failed to execute the program");
    Ok(())
}