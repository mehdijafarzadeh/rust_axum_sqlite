-- Add migration script here
CREATE TABLE IF NOT EXISTS items (
     id INTEGER PRIMARY KEY AUTOINCREMENT,
     name TEXT NOT NULL
);