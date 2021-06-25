-- Your SQL goes here
CREATE TABLE todo(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task TEXT NOT NULL,
    user TEXT NOT NULL,
    iscompleted BOOLEAN NOT NULL,
)