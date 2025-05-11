-- Add up migration script here
CREATE TABLE users
(
    username TEXT PRIMARY KEY,
    password TEXT NOT NULL
);
CREATE TABLE syncs
(
    document   TEXT PRIMARY KEY,
    "user"     TEXT NOT NULL REFERENCES users (username),
    sync_time  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    percentage REAL NOT NULL,
    progress   TEXT NOT NULL,
    device     TEXT NOT NULL,
    device_id  TEXT NOT NULL
);
