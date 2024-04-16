#!/bin/bash

set -x
set -eo pipefail

if ! [ -x "$(command -v sqlite3)" ]; then
  echo >&2 "Error: sqlite3 is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 " cargo install --version=0.5.7 sqlx-cli --no-default-features --features sqlite"
  echo >&2 "to install it."
  exit 1
fi

DB_NAME="${SQLITE_DB:=rooms.db}"
echo "${DB_NAME}"
sqlite3 "${DB_NAME}"

export DATABASE_URL="sqlite:${DB_NAME}"

sqlx database create
sqlx migrate run

echo "SQLite database has been migrated, ready to go!"
