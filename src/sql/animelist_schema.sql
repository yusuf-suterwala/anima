CREATE TABLE AnimeList(
    mal_id INTEGER UNSIGNED PRIMARY KEY,
    status TEXT NOT NULL CHECK(status in ('COMPLETED', 'WATCHING', 'PAUSED', 'DROPPED', 'PLANNING')),
    episodes_completed INTEGER UNSIGNED NOT NULL,
    score NUMERIC UNSIGNED, -- this is null when the user does not provide a score
    -- all of these timestamps are in unix epoch
    added_timestamp INTEGER UNSIGNED NOT NULL, -- unsigned because this was made in the 21th century
    started_timestamp INTEGER, -- null when user has not started the anime
    complated_timestamp INTEGER -- null when user has not compated the anime
);
