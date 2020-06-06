CREATE TABLE quoths (
    id INTEGER NOT NULL PRIMARY KEY,
    author INTEGER,
    guild INTEGER,
    starred_by INTEGER,
    message_id INTEGER UNIQUE,
    content TEXT NOT NULL,
    legacy BOOLEAN NOT NULL DEFAULT 'f',
    legacy_author_fallback TEXT
)
