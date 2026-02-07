#!/usr/bin/env bash
set -euo pipefail

docker-compose down -v

chmod +x scripts/dev-up.sh scripts/dev-down.sh
