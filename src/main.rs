use std::path::Path;
mod sandbox;
mod engine;

fn main() {
    let app_path = Path::new("/home/richard/Downloads/DiscordSetup.exe");

    println!("creation de la sandbox : {:?}", app_path);
    let prefix_path = match sandbox::prepare_prefixe(&app_path){
        Ok(path) => path,
        Err(e) => {
            eprintln!("❌ Erreur : {}", e);
            return;
        }
    };

    match engine::lancer_installation(prefix_path, app_path){
        Ok(()) => {
            println!("programme lancé avec succès");
        }
        Err(e) => {
            println!("❌ Erreur : {}", e);
            return;
        }
    }
}
