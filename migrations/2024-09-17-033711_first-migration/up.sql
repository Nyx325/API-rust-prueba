-- Your SQL goes here
CREATE TABLE clients(
    client_id INTEGER PRIMARY KEY AUTOINCREMENT,
    active BOOL NOT NULL,
    username TEXT NOT NULL,
    pwd TEXT NOT NULL,
    birth_date DATE NOT NULL
);
