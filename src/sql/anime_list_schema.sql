CREATE TABLE AnimeList(
    mal_id INTEGER UNSIGNED PRIMARY KEY,
    status TEXT NOT NULL CHECK(status in ('COMPLETED', 'WATCHING', 'PAUSED', 'DROPPED', 'PLANNING')),
    episodes_completed INTEGER UNSIGNED NOT NULL,
    score NUMERIC UNSIGNED CHECK(score >= 0 AND score <= 10), -- this is null when the user does not provide a score
    -- all of these timestamps are in unix epoch
    started_timestamp INTEGER, -- null when user has not started the anime
    complated_timestamp INTEGER -- null when user has not compated the anime
);
