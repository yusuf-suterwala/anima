use rusqlite::{self, Connection};
use std::{
    error::Error,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug)]
pub enum AnimeInfoStatus {
    Complated,
    Watching,
    Paused,
    Droped,
    Planning,
}

impl FromStr for AnimeInfoStatus {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "COMPLATED" => Ok(Self::Complated),
            "WATCHING" => Ok(Self::Watching),
            "PAUSED" => Ok(Self::Paused),
            "DROPED" => Ok(Self::Droped),
            "PLANING" => Ok(Self::Planning),
            _ => {
                panic!("ERROR: unknown status inputed");
            }
        }
    }
}

// why does rust not have something like this by defulat?
#[derive(Debug)]
pub struct Decimal {
    full: u32,
    frac: u32,
}

impl FromStr for Decimal {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (full, frac) = s.split_once(".").ok_or("unknown value as score")?;
        let num_full = str::parse(full)?;
        let num_frac = str::parse(frac)?;
        Ok(Self {
            full: num_full,
            frac: num_frac,
        })
    }
}

#[derive(Debug)]
pub struct AnimeInfo {
    pub status: Option<AnimeInfoStatus>,
    pub episodes_completed: Option<u32>,
    pub score: Option<Decimal>,
    pub started_timestamp: Option<u32>,
    pub complated_timestamp: Option<u32>,
}

pub fn update_anime_entry(
    anime_db_folder: &Path,
    mal_id: u32,
    updated_info: Option<AnimeInfo>,
) -> rusqlite::Result<()> {
    let anime_list_path = anime_db_folder.join("anime_list.db");
    let conn = Connection::open(anime_list_path)?;
    if !conn.table_exists(None, "AnimeList")? {
        conn.execute(include_str!("./sql/anime_list_schema.sql"), ())?;
    }
    dbg!(mal_id);
    dbg!(updated_info);
    todo!()
}
