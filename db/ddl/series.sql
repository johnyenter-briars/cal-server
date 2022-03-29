CREATE TABLE IF NOT EXISTS series (
    id INTEGER PRIMARY KEY,
    repeateveryweek INTEGER NOT NULL,
    repeateonmon BOOLEAN NOT NULL CHECK (repeateonmon IN (0, 1)),
    repeateontues BOOLEAN NOT NULL CHECK (repeateontues IN (0, 1)),
    repeateonwed BOOLEAN NOT NULL CHECK (repeateonwed IN (0, 1)),
    repeateonthurs BOOLEAN NOT NULL CHECK (repeateonthurs IN (0, 1)),
    repeateonfri BOOLEAN NOT NULL CHECK (repeateonfri IN (0, 1)),
    repeateonsat BOOLEAN NOT NULL CHECK (repeateonsat IN (0, 1)),
    repeateonsun BOOLEAN NOT NULL CHECK (repeateonsun IN (0, 1)),
    endson INTEGER
)