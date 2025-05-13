-- Add up migration script here
CREATE TABLE users
(
    username TEXT PRIMARY KEY,
    password TEXT NOT NULL
);
CREATE TABLE syncs
(
    document   TEXT NOT NULL,
    "user"     TEXT NOT NULL REFERENCES users (username),
    timestamp  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    percentage TEXT NOT NULL,
    progress   TEXT NOT NULL,
    device     TEXT NOT NULL,
    device_id  TEXT NOT NULL,
    PRIMARY KEY (document, "user")
);
