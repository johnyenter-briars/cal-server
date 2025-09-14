CREATE TABLE IF NOT EXISTS notification (
    id TEXT PRIMARY KEY,
    eventid TEXT NOT NULL,
    caluserid TEXT NOT NULL,

    FOREIGN KEY (eventid) REFERENCES event(id),
    FOREIGN KEY (caluserid) REFERENCES caluser(id)
)
