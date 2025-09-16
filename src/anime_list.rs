use rusqlite::{self, Connection};
use std::path::{Path, PathBuf};

enum AnimeInfoStatus {
    Complated,
    Watching,
    Paused,
    Droped,
    Planning,
}

// why does rust not have something like this by defulat?
type Decimal = (u32, u32);

pub struct AnimeInfo {
    status: Option<AnimeInfoStatus>,
    episodes_completed: Option<u32>,
    score: Option<Decimal>,
    started_timestamp: Option<u32>,
    complated_timestamp: Option<u32>,
}

pub fn update_anime_entry(
    anime_db_path: &Path,
    mal_id: u32,
    updated_info: Option<AnimeInfo>,
) -> rusqlite::Result<()> {
    let anime_list_path = anime_db_path.join("anime_list.db");
    let conn = Connection::open(anime_list_path)?;
    if !conn.table_exists(None, "AnimeList")? {
        conn.execute(include_str!("./sql/anime_list_schema.sql"), ())?;
    }
    Ok(())
}
