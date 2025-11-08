use pico_args;
use std::env;
use std::error::Error;
use std::path::Path;

use crate::anime_list::AnimeInfo;

mod anime_db;
mod anime_list;

fn main() {
    let anidb_foldername = env::var("ANIDB_FOLDER")
        .expect("set ANIDB_FOLDER env var")
        .to_string();
    let anidb_folder = Path::new(&anidb_foldername);

    let mut args = pico_args::Arguments::from_env();

    match args.subcommand().unwrap().as_deref() {
        Some("create") => {
            anime_db::make_db(anidb_folder).unwrap();
        }
        Some("update-list") => {
            let status = args.opt_value_from_str("--status").unwrap();
            let episodes_completed = args.opt_value_from_str("--ep").unwrap();
            let score = args.opt_value_from_str("--score").unwrap();
            let started_timestamp = args.opt_value_from_str("--started-ts").unwrap();
            let complated_timestamp = args.opt_value_from_str("--complated-ts").unwrap();

            let info = AnimeInfo {
                status,
                episodes_completed,
                score,
                started_timestamp,
                complated_timestamp,
            };

            let _ = anime_list::update_anime_entry(anidb_folder, 0, info);
        }
        Some(_) => {
            eprintln!("ERROR: unknown subcommand");
        }
        None => {
            eprintln!("ERROR: no subcommand entered");
        }
    }
}
