#!/usr/bin/env bash
# Run this project in one of three environments: local (cargo run), docker
# (dev-friendly docker-compose.yml), or prod (hardened docker-compose.prod.yml).
# See RUNNING.md for the full manual walkthrough of each of these.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT_DIR"

usage() {
  cat <<'EOF'
Usage:
  scripts/run.sh local [postgres|sqlite]   cargo run against your own database (default: postgres)
  scripts/run.sh docker [up|down|logs]     dev docker-compose.yml, zero external setup (default: up)
  scripts/run.sh prod   [up|down|logs]     hardened docker-compose.prod.yml (default: up)

Examples:
  scripts/run.sh local
  scripts/run.sh local sqlite
  scripts/run.sh docker
  scripts/run.sh docker down
  scripts/run.sh prod
  scripts/run.sh prod logs
EOF
}

run_local() {
  local backend="${1:-postgres}"

  if [ ! -f .env ]; then
    echo "error: .env not found. Copy .env.example to .env and fill in DATABASE_URL/MAGIC_KEY first." >&2
    exit 1
  fi
  set -a
  # shellcheck disable=SC1091
  source .env
  set +a
  : "${DATABASE_URL:?DATABASE_URL must be set in .env}"
  : "${MAGIC_KEY:?MAGIC_KEY must be set in .env}"

  # cors_for_local_development turns off the session cookie's Secure flag —
  # needed for any client (including blog-client/blog-cli) talking plain
  # http:// to a locally-run server. See ARCHITECTURE.md § Authentication.
  case "$backend" in
    postgres)
      echo "==> cargo run (postgres, cors_for_local_development)"
      exec cargo run --features cors_for_local_development
      ;;
    sqlite)
      echo "==> cargo run (sqlite, cors_for_local_development)"
      exec cargo run --no-default-features --features sqlite,cors_for_local_development
      ;;
    *)
      echo "error: unknown backend '$backend' (expected 'postgres' or 'sqlite')" >&2
      exit 1
      ;;
  esac
}

run_docker() {
  local action="${1:-up}"
  case "$action" in
    up)
      docker compose up -d --build
      echo
      echo "Up at http://127.0.0.1:8080 — try: curl http://127.0.0.1:8080/posts/page/1"
      ;;
    down)
      docker compose down
      ;;
    logs)
      docker compose logs -f web
      ;;
    *)
      echo "error: unknown action '$action' (expected 'up', 'down', or 'logs')" >&2
      exit 1
      ;;
  esac
}

run_prod() {
  local action="${1:-up}"

  if [ ! -f .env.prod ]; then
    echo "error: .env.prod not found. Copy .env.prod.example to .env.prod and fill in real secrets first." >&2
    exit 1
  fi

  local compose=(docker compose -f docker-compose.prod.yml --env-file .env.prod)

  case "$action" in
    up)
      "${compose[@]}" up -d --build
      # shellcheck disable=SC1091
      local port
      port="$(source .env.prod && echo "${APP_PORT:-8080}")"
      echo
      echo "Up at http://127.0.0.1:${port} — try: curl http://127.0.0.1:${port}/posts/page/1"
      ;;
    down)
      "${compose[@]}" down
      ;;
    logs)
      "${compose[@]}" logs -f web
      ;;
    *)
      echo "error: unknown action '$action' (expected 'up', 'down', or 'logs')" >&2
      exit 1
      ;;
  esac
}

case "${1:-}" in
  local)
    run_local "${2:-postgres}"
    ;;
  docker)
    run_docker "${2:-up}"
    ;;
  prod)
    run_prod "${2:-up}"
    ;;
  -h|--help|"")
    usage
    ;;
  *)
    echo "error: unknown environment '$1'" >&2
    usage
    exit 1
    ;;
esac
