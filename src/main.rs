use std::path::Path;
mod sandbox;

fn main() {
    let app_path = Path::new("/home/richard/Downloads/DiscordSetup.exe");

    println!("creation de la sandbox : {:?}", app_path);
    match sandbox::prepare_prefixe(&app_path){
        Ok(prefixe) => {
            println!("🎉 Succès ! Le préfixe Wine est prêt dans :");
            println!("{:?}", prefixe);
        }
        Err(e) => {
            println!("❌ Erreur : {}", e);
        }
    }
}
