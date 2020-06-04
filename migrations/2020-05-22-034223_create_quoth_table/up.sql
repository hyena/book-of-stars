CREATE TABLE quoths (
    id INTEGER PRIMARY KEY,
    author INTEGER,
    starred_by INTEGER,
    legacy_author TEXT,
    content TEXT NOT NULL,
    legacy BOOLEAN NOT NULL DEFAULT 'f'
)
