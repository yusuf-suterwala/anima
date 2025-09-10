use std::env;
use std::error::Error;
use std::path::Path;

mod anime_db;

fn main() -> Result<(), Box<dyn Error>> {
    let anidb_foldername = env::var("ANIDB_FOLDER")
        .expect("set ANIDB_FOLDER env var")
        .to_string();
    let anidb_folder = Path::new(&anidb_foldername);

    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(command) => match command.as_str() {
            "build" => {
                anime_db::make_db(anidb_folder)?;
            }
            _ => {
                eprintln!("unknown command entered")
            }
        },
        None => {
            eprintln!("no command entered")
        }
    }

    Ok(())
}
