#!/usr/bin/env bash
set -e
flyctl deploy --config infra/fly.toml
flyctl deploy --config infra/fly.mobile.toml
