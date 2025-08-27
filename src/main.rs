use serde::Deserialize;
use serde_json;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use ureq;
use zstd;

#[derive(Deserialize)]
struct AnimeEntry {
    sources: Vec<String>,
    title: String,
    episodes: u32,
    synonyms: Vec<String>,
}

fn download_jsonl(output_filename: &Path) -> Result<(), Box<dyn Error>> {
    let mut compressed_database = File::create_new(output_filename)?;
    let mut http_file_reader = ureq::get(
        "https://github.com/manami-project/anime-offline-database/releases/download/latest/anime-offline-database.jsonl.zst",
    ).call()?.into_body().into_reader();

    println!("downloading database...");

    io::copy(&mut http_file_reader, &mut compressed_database)?;

    Ok(())
}

fn extract_jsonl(compressed_filename: &Path, uncompressed_filename: &Path) -> io::Result<()> {
    let compressed_file = File::open(compressed_filename)?;
    let uncompressed_file = File::create_new(uncompressed_filename)?;

    zstd::stream::copy_decode(compressed_file, uncompressed_file)?;

    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let anidb_foldername = env::var("ANIDB_FOLDER")
        .expect("set ANIDB_FOLDER env var")
        .to_string();
    let anidb_folder = Path::new(&anidb_foldername);

    let compressed_filebuf = anidb_folder.join("./anime-offline-database.jsonl.zst");
    let compressed_filepath = compressed_filebuf.as_path();

    let uncompressed_filebuf = anidb_folder.join("./anime-offline-database.jsonl");
    let uncompressed_filepath = uncompressed_filebuf.as_path();

    download_jsonl(compressed_filepath)?;
    extract_jsonl(compressed_filepath, uncompressed_filepath)?;

    let uncompressed_file = File::open(uncompressed_filepath)?;
    let uncompressed_filebuf = BufReader::new(uncompressed_file);

    let mut jsonl_lines = uncompressed_filebuf.lines();

    jsonl_lines.next(); // skips the first line with next bc its metadata line

    for i in jsonl_lines {
        let line = i?;
        let anime_entry: AnimeEntry = serde_json::from_str(line.as_str())?;
        println!("{0}", anime_entry.title);
        break;
    }

    Ok(())
}
