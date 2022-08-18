CREATE TABLE IF NOT EXISTS calendar (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    caluserid TEXT NOT NULL,
    color TEXT NOT NULL,

    FOREIGN KEY (caluserid) REFERENCES caluser(id)
)