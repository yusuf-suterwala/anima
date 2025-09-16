-- TODO: find out whats the best table to repersent history
CREATE TABLE AnimeListHistory(
    timestamp INTEGER,   
    mal_id INTEGER UNSIGNED,
    status TEXT CHECK(status in ('COMPLETED', 'WATCHING', 'PAUSED', 'DROPPED', 'PLANNING')),
    episodes_completed INTEGER UNSIGNED  ,
    score NUMERIC UNSIGNED,
    added_timestamp INTEGER UNSIGNED  ,
    started_timestamp INTEGER,
    complated_timestamp INTEGER
)
