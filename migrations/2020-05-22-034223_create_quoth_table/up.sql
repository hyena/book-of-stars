CREATE TABLE quoths (
    id INTEGER NOT NULL PRIMARY KEY,
    author INTEGER,
    guild INTEGER,
    starred_by INTEGER,
    content TEXT NOT NULL,
    legacy BOOLEAN NOT NULL DEFAULT 'f',
    legacy_author_fallback TEXT
)
