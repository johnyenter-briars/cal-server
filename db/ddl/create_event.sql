CREATE TABLE IF NOT EXISTS event (
    id INTEGER PRIMARY KEY,
    time INTEGER,
    name TEXT NOT NULL,
    caluserid INTEGER NOT NULL,
    FOREIGN KEY (caluserid) REFERENCES caluser(id)
)