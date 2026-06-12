-- Add migration script here

CREATE TABLE users (
    id UUID PRIMARY KEY,
    telegram_id TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);