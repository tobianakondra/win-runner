use std::path::{Path, PathBuf};
use std::env;
use std::fs;

// la fonciton doit etre publique pour etre accessible depuis main
pub fn prepare_prefixe(exe_path: &Path) -> Result<PathBuf, String> {
    //extraire le nom de l'application à partir de exe_path
    let name = exe_path
               .file_stem()
               .and_then(|s| s.to_str()) //la methode file_stem fournit un type special Option<&OsStr> , il faut donc la convertir a un type adapté a rust. and_then est une methode de Option, elle dist a rust, si file_stem a renvoyer du contenu, applique la conversion to_str()
               .unwrap_or("app_inconnue");

    let home = env::var("HOME").expect("impossible de lire la variable HOME");

    //contruire le chemin dynamique  ~/.local/share/win_runner/prefixes/<nom_app>
    let mut path = PathBuf::from(home);
    path.push(".local");
    path.push("share");
    path.push("win_runner");
    path.push("prefixes");
    path.push(name);

    //creer toute l'arborescence si elle n'existe pas déja
    fs::create_dir_all(&path).expect("impossible de creer le dossier de prefixe");

    Ok(path)
}