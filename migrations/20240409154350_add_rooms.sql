-- Add migration script here
-- Add migration script here
CREATE TABLE rooms (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id TEXT NOT NULL,
    available INTEGER NOT NULL DEFAULT 1,
    created_at DATETIME DEFAULT (datetime('now', 'localtime'))
)