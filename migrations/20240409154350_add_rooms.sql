-- Add migration script here
-- Add migration script here
CREATE TABLE IF NOT EXISTS rooms (
    id INTEGER PRIMARY KEY,
    room_id TEXT NOT NULL,
    available INTEGER NOT NULL DEFAULT 1, 
    created_at DATETIME DEFAULT (datetime('now', 'localtime'))
)