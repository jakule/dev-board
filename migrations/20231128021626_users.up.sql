CREATE TABLE IF NOT EXISTS users
(
    id       TEXT PRIMARY KEY NOT NULL,
    username VARCHAR(255)     NOT NULL UNIQUE,
    password VARCHAR(511)     NOT NULL
);-- Add up migration script here
