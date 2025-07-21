#!/usr/bin/env bash
set -e
echo "Building & deploying dev stack..."
docker compose build
docker compose up -d
echo "VerdeX dev environment running at http://localhost:3000"
