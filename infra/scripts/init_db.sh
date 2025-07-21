#!/usr/bin/env bash
set -e
docker compose up -d db
until pg_isready -h localhost -p 5432 -U verdex; do
  echo "Waiting for DB..."
  sleep 2
done
sqlx migrate run --source apps/api/migrations --database-url postgres://verdex:verdex@localhost:5432/verdex
echo "DB ready!"
