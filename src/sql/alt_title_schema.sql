BEGIN;
CREATE TABLE AnimeAltTitle(
    alt_title TEXT NOT NULL,
    associated_mal_id INTEGER UNSIGNED NOT NULL REFERENCES Anime(mal_id) 
);

CREATE INDEX AnimeAltTitleIDX ON AnimeAltTitle(alt_title);
COMMIT;
