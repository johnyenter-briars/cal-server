CREATE TABLE IF NOT EXISTS event (
    id INTEGER PRIMARY KEY,
    starttime INTEGER,
    endtime INTEGER,
    name TEXT NOT NULL,
    caluserid INTEGER NOT NULL,
    seriesid INTEGER,

    FOREIGN KEY (caluserid) REFERENCES caluser(id),
    FOREIGN KEY (seriesid) REFERENCES series(id)
)