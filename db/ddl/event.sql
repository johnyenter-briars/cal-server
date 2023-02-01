CREATE TABLE IF NOT EXISTS event (
    id TEXT PRIMARY KEY,
    starttime INTEGER,
    endtime INTEGER,
    name TEXT NOT NULL,
    description TEXT,
    caluserid TEXT NOT NULL,
    seriesid TEXT,
    calendarid TEXT NOT NULL,
    color TEXT NOT NULL,

    FOREIGN KEY (caluserid) REFERENCES caluser(id),
    FOREIGN KEY (seriesid) REFERENCES series(id)
)