#!/usr/bin/env bash
set -euo pipefail

docker-compose up -d --build

chmod +x scripts/dev-up.sh scripts/dev-down.sh
