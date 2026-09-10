use std::fs;
use std::path::{Path, PathBuf};
use std::env;

pub fn creer_raccourci(app_path: &Path, prefix_path: &Path) -> Result<(), String>{
    let mon_app = app_path
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("impossible d'extraire le nom de l'appli");
    let extension = String::from(".desktop"); 
    let file = format!("{}{}", mon_app, extension);

    let home = env::var("HOME").expect("impossible d'extraire le repertoire personnel");
    let mut path = PathBuf::from(home);
    path.push(".local");
    path.push("share");
    path.push("applications");

    //creer le dossier
    fs::create_dir_all(&path);

    //recevoir le nom de l'application

    let full_path = path.join(file);

    let desktop_content = format!(
        "[Desktop Entry]\n\
        Type=Application\n\
        Name={}\n\
        Exec=env WINEPREFIX=\"{}\" wine \"{}\"\n\
        Icon=application-x-executable\n\
        Terminal=false\n\
        Categories=Utility;Wine;\n",
        mon_app,
        prefix_path.display(),
        app_path.display()
    );

    let _ = fs::write(&full_path, desktop_content).expect("impossible d'ecrire le fichier");
    println!("raccourci creer avec succès {}", &full_path.display());

    Ok(())
}