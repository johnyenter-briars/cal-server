CREATE TABLE IF NOT EXISTS event (
    id TEXT PRIMARY KEY,
    starttime INTEGER,
    endtime INTEGER,
    name TEXT NOT NULL,
    description TEXT,
    caluserid INTEGER NOT NULL,
    seriesid INTEGER,

    FOREIGN KEY (caluserid) REFERENCES caluser(id),
    FOREIGN KEY (seriesid) REFERENCES series(id)
)