use rusqlite::Connection;
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use ureq;
use zstd;

// TODO: use anilist to get synopsis + populerity
#[derive(Deserialize)]
struct AnimeEntry {
    sources: Vec<String>,
    title: String,
    episodes: u32,
    synonyms: Vec<String>,
}

fn download_jsonl(output_filename: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut compressed_database = File::create_new(output_filename)?;
    let mut http_file_reader = ureq::get(
    "https://github.com/manami-project/anime-offline-database/releases/download/latest/anime-offline-database.jsonl.zst",
).call()?.into_body().into_reader();

    println!("downloading database jsonl...");

    io::copy(&mut http_file_reader, &mut compressed_database)?;

    Ok(())
}

fn extract_jsonl(compressed_filename: &PathBuf, uncompressed_filename: &PathBuf) -> io::Result<()> {
    let compressed_file = File::open(compressed_filename)?;
    let uncompressed_file = File::create_new(uncompressed_filename)?;

    println!("extracting database jsonl...");

    zstd::stream::copy_decode(compressed_file, uncompressed_file)?;

    Ok(())
}

fn setup_db(conn: &Connection) -> rusqlite::Result<()> {
    println!("setting up database...");
    let _ = conn.execute_batch(include_str!("./sql/anime_schema.sql"))?;
    let _ = conn.execute_batch(include_str!("./sql/alt_title_schema.sql"))?;

    Ok(())
}

fn populate_db(anime_jsonl: &PathBuf, conn: &mut Connection) -> Result<(), Box<dyn Error>> {
    println!("populating database...");
    let uncompressed_file = File::open(anime_jsonl)?;
    let uncompressed_filebuf = BufReader::new(uncompressed_file);

    let mut jsonl_lines = uncompressed_filebuf.lines();

    jsonl_lines.next(); // skips the first line with next bc its metadata line

    // TODO: - find a way to use prepare and transactions without a brrowing error
    //       - maybe prosess the alt titles so there are no duplecates
    //       - maybe add the title from Anime to AnimeAltTitle so there is one place to look up

    let mut total_anime_added = 0;

    let tx = conn.transaction()?;

    for i in jsonl_lines {
        let line = i?;
        let anime_entry: AnimeEntry = serde_json::from_str(line.as_str())?;
        let maybe_mal_url = anime_entry
            .sources
            .iter()
            .find(|i| i.starts_with("https://myanimelist.net"));

        if let Some(mal_url) = maybe_mal_url {
            let mal_id_str = mal_url.split("/").last().unwrap();
            let mal_id: u32 = mal_id_str.parse()?;

            tx.execute(
                "INSERT INTO Anime (mal_id, title, episodes) VALUES (?1, ?2, ?3)",
                (mal_id, anime_entry.title, anime_entry.episodes),
            )?;

            for title in anime_entry.synonyms {
                tx.execute(
                    "INSERT INTO AnimeAltTitle (alt_title, associated_mal_id) VALUES (?1, ?2)",
                    (title, mal_id),
                )?;
            }

            total_anime_added += 1;
        } else {
            continue;
        }
    }

    tx.commit()?;

    println!("total anime added: {total_anime_added}");

    Ok(())
}

pub fn make_db(anidb_folder: &Path) -> Result<(), Box<dyn Error>> {
    let anidb_temp_folder = anidb_folder.join("temp");
    let compressed_jsonlpath = anidb_temp_folder.join("anime-offline-database.jsonl.zst");
    let uncompressed_jsonpath = anidb_temp_folder.join("anime-offline-database.jsonl");
    let anidb_db_path = anidb_folder.join("anime.db");

    println!("folder cleanup...");
    fs::remove_file(&anidb_db_path)?; // TODO: handle not found eror for this
    fs::create_dir(&anidb_temp_folder)?;

    download_jsonl(&compressed_jsonlpath)?;
    extract_jsonl(&compressed_jsonlpath, &uncompressed_jsonpath)?;

    println!("creating database...");
    let mut conn = Connection::open(&anidb_db_path)?;

    setup_db(&conn)?;
    populate_db(&uncompressed_jsonpath, &mut conn)?;

    println!("temp cleanup...");
    fs::remove_dir_all(&anidb_temp_folder)?;

    Ok(())
}
