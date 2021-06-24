-- Your SQL goes here
CREATE TABLE todo(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task TEXT NOT NULL,
    user TEXT NOT NULL,
    iscompleted TEXT NOT NULL,
)