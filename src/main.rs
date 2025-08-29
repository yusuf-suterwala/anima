use rusqlite::Connection;
use serde::Deserialize;
use serde_json;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use ureq;
use zstd;

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

    println!("downloading database...");

    io::copy(&mut http_file_reader, &mut compressed_database)?;

    Ok(())
}

fn extract_jsonl(compressed_filename: &PathBuf, uncompressed_filename: &PathBuf) -> io::Result<()> {
    let compressed_file = File::open(compressed_filename)?;
    let uncompressed_file = File::create_new(uncompressed_filename)?;

    zstd::stream::copy_decode(compressed_file, uncompressed_file)?;

    Ok(())
}

fn setup_db(conn: &Connection) -> rusqlite::Result<()> {
    let _ = conn.execute_batch(include_str!("./sql/anime_schema.sql"))?;
    let _ = conn.execute_batch(include_str!("./sql/alt_title_schema.sql"))?;

    Ok(())
}

fn populate_db(anime_jsonl: &PathBuf, conn: &Connection) -> Result<(), Box<dyn Error>> {
    let uncompressed_file = File::open(anime_jsonl)?;
    let uncompressed_filebuf = BufReader::new(uncompressed_file);

    let mut jsonl_lines = uncompressed_filebuf.lines();

    jsonl_lines.next(); // skips the first line with next bc its metadata line

    // TODO: use transactions and maybe prepare cached

    for i in jsonl_lines {
        let line = i?;
        let anime_entry: AnimeEntry = serde_json::from_str(line.as_str())?;
        conn.execute("", ())
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let anidb_foldername = env::var("ANIDB_FOLDER")
        .expect("set ANIDB_FOLDER env var")
        .to_string();
    let anidb_folder = Path::new(&anidb_foldername);

    let anidb_temp_folder = anidb_folder.join("temp");

    let compressed_jsonlpath = anidb_temp_folder.join("anime-offline-database.jsonl.zst");

    let uncompressed_jsonpath = anidb_temp_folder.join("anime-offline-database.jsonl");

    download_jsonl(&compressed_jsonlpath)?;
    extract_jsonl(&compressed_jsonlpath, &uncompressed_jsonpath)?;

    let conn = Connection::open(anidb_folder.join("anime.db"))?;

    setup_db(&conn)?;

    Ok(())
}
