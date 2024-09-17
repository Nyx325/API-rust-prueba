-- Your SQL goes here
CREATE TABLE clients(
    client_id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    pwd TEXT NOT NULL,
    birth_date DATE NOT NULL
);
