use rusqlite::{self, Connection, ToSql, params};
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
            "PLANNING" => Ok(Self::Planning),
            _ => {
                panic!("ERROR: unknown status inputed");
            }
        }
    }
}

impl ToSql for AnimeInfoStatus {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let staus = match &self {
            Self::Complated => "COMPLATED",
            Self::Watching => "WATCHING",
            Self::Paused => "PAUSED",
            Self::Droped => "DROPED",
            Self::Planning => "PLANNING",
        };
        staus.to_sql()
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
    updated_info: AnimeInfo,
) -> rusqlite::Result<()> {
    let anime_list_path = anime_db_folder.join("anime_list.db");
    let conn = Connection::open(anime_list_path)?;
    if !conn.table_exists(None, "AnimeList")? {
        conn.execute(include_str!("./sql/anime_list_schema.sql"), ())?;
    }
    let is_anime_in_list: u32 = conn.query_one(
        "SELECT EXISTS(SELECT 1 FROM AnimeList WHERE mal_id = ?1 LIMIT 1)
",
        [mal_id],
        |row| row.get(0),
    )?;
    if is_anime_in_list != 0 {
        print!("anime exists")
    } else {
        if let (Some(status), Some(episodes_comp)) =
            (updated_info.status, updated_info.episodes_completed)
        {
            conn.execute(
                "INSERT INTO AnimeList (mal_id, status, episodes_completed) VALUES (?1, ?2, ?3)",
                params![mal_id, status, episodes_comp],
            )?;
        } else {
            eprint!("adding a anime requires status and episode complated")
        }
    }
    Ok(())
}
