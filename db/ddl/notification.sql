CREATE TABLE IF NOT EXISTS notification (
    id TEXT PRIMARY KEY,
    calendarid TEXT NOT NULL,
    eventid TEXT NOT NULL,
    caluserid TEXT NOT NULL,

    FOREIGN KEY (calendarid) REFERENCES calendar(id),
    FOREIGN KEY (eventid) REFERENCES event(id),
    FOREIGN KEY (caluserid) REFERENCES caluser(id)
)
