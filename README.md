# VerdeX

> Cross-chain escrow + P2P exchange (BTC, SOL, USDT, KES).

## Quick start
```bash
cp .env.example .env
docker compose up -d
cd apps/mobile && yarn start   # Metro
cd apps/api    && cargo run    # Axum
