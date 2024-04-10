-- Add migration script here
-- Add migration script here
CREATE TABLE rooms (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    room_id TEXT NOT NULL,
    available BOOLEAN NOT NULL, 
    created_at timestamptz NOT NULL
)
