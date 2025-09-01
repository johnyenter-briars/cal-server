CREATE TABLE IF NOT EXISTS sharedcalendar (
    id TEXT PRIMARY KEY,
    calendarid TEXT NOT NULL,
    ownercaluserid TEXT NOT NULL,
    caluserid TEXT NOT NULL,

    FOREIGN KEY (calendarid) REFERENCES calendar(id),
    FOREIGN KEY (ownercaluserid) REFERENCES caluser(id),
    FOREIGN KEY (caluserid) REFERENCES caluser(id)
)
