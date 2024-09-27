#/bin/bash
docker compose restart
sqlx database drop
sqlx database create
sqlx mig run
