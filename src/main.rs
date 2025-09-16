use std::env;
use std::error::Error;
use std::path::Path;

mod anime_db;
mod anime_list;

fn main() {
    let anidb_foldername = env::var("ANIDB_FOLDER")
        .expect("set ANIDB_FOLDER env var")
        .to_string();
    let anidb_folder = Path::new(&anidb_foldername);

    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(command) => match command.as_str() {
            "create" => {
                anime_db::make_db(&anidb_folder).unwrap();
            }
            "list" => match args.get(2) {
                Some(subcommand) => match subcommand.as_str() {
                    "update" => {
                        anime_list::update_anime_entry(&anidb_folder, 1, None).unwrap();
                    }
                    _ => {
                        eprintln!("subcommand not found")
                    }
                },
                None => {
                    todo!()
                }
            },
            _ => {
                eprintln!("unknown command entered")
            }
        },
        None => {
            eprintln!("no command entered")
        }
    }
}
