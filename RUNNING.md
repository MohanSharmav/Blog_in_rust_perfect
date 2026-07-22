# Running & Testing

Practical instructions for getting this project running locally, running it in Docker, and
running its test suite. For *why* things are built the way they are, see
[ARCHITECTURE.md](ARCHITECTURE.md); for the workspace layout, see
[README.md § Workspace layout](README.md#workspace-layout).

## Table of contents

1. [Prerequisites](#prerequisites)
2. [scripts/run.sh — one entry point for every environment](#scriptsrunsh--one-entry-point-for-every-environment)
3. [Quick start with Docker](#quick-start-with-docker)
4. [Running locally](#running-locally)
5. [Environment variables reference](#environment-variables-reference)
6. [Using the CLI](#using-the-cli)
7. [Testing](#testing)
8. [Docker in depth](#docker-in-depth)
9. [Production deployment](#production-deployment)
10. [Troubleshooting](#troubleshooting)

---

## Prerequisites

| Tool | Needed for |
|---|---|
| Rust (stable toolchain) | Building/running anything outside Docker |
| Docker + Docker Compose | The containerized path — nothing else needs installing this way |
| PostgreSQL (local, or via Docker) | Running against Postgres outside `docker compose` |
| SQLite3 CLI | Optional — only for inspecting a SQLite database by hand |
| `sqlx-cli` | Optional — only if you want to run migrations manually instead of relying on auto-migration |

You do **not** need Postgres, SQLite, or `sqlx-cli` installed to use the Docker path below — the
container brings its own database.

## scripts/run.sh — one entry point for every environment

```bash
scripts/run.sh local [postgres|sqlite]   # cargo run, against your own database
scripts/run.sh docker [up|down|logs]     # dev docker-compose.yml — zero external setup
scripts/run.sh prod   [up|down|logs]     # hardened docker-compose.prod.yml
```

`local` and `docker` are thin wrappers around the commands in the rest of this document.
`prod` is the one with real logic behind it — see
[Production deployment](#production-deployment) for what it does differently and why.

## Quick start with Docker

The fastest way to get a fully working stack running, with zero local Rust/Postgres setup:

```bash
docker compose up -d --build
```

This builds the app image and starts it alongside Postgres. Migrations run automatically on
startup. Once it's up:

```bash
curl http://127.0.0.1:8080/posts/page/1
```

should return HTML with the three seeded categories' posts. Browse and try the JSON API
interactively at `http://127.0.0.1:8080/swagger-ui/` (see [API.md § Interactive Swagger UI](API.md#interactive-swagger-ui)).
See [Docker in depth](#docker-in-depth) for everything else — switching to SQLite, cleaning up,
what the env vars do.

## Running locally

### 1. Configure environment

```bash
cp .env.example .env
```

Fill in `.env`:

| Variable | Postgres example | SQLite example |
|---|---|---|
| `DATABASE_URL` | `postgres://user:password@localhost:5432/dbname` | `sqlite://blog.db` (file created automatically) |
| `MAGIC_KEY` | any string — used to encrypt stored passwords | same |

### 2. Get a database

**Postgres** — point `DATABASE_URL` at any reachable instance, e.g. one you start yourself:

```bash
docker run -d --name blog-pg -e POSTGRES_PASSWORD=password -e POSTGRES_DB=dbname -p 5432:5432 postgres:15
```

**SQLite** — nothing to start; the file is created on first connect.

### 3. Run migrations (optional)

`blog-server` **auto-applies migrations on every startup** (idempotent — safe to run every time),
so this step is optional. To apply them ahead of time anyway:

```bash
# Postgres (needs sqlx-cli built with postgres support: cargo install sqlx-cli --features postgres)
sqlx migrate run --source blog-storage/migrations/postgres

# SQLite (needs sqlx-cli built with sqlite support, or apply the .sql files directly)
sqlite3 blog.db < blog-storage/migrations/sqlite/0001_initial_schema.sql
sqlite3 blog.db < blog-storage/migrations/sqlite/0002_seed_data.sql
```

### 4. Run it

The database backend is a **compile-time** choice (a Cargo feature), not a runtime flag — see
[ARCHITECTURE.md § Database backend strategy](ARCHITECTURE.md#database-backend-strategy) for why.

```bash
# Postgres (the default feature)
cargo run

# SQLite
cargo run --no-default-features --features sqlite
```

If you're testing over plain `http://` (no TLS) — which local development always is — also enable
`cors_for_local_development`, or the session cookie won't be sent back by any spec-compliant HTTP
client (including `blog-client`/`blog-cli`):

```bash
cargo run --features cors_for_local_development
cargo run --no-default-features --features sqlite,cors_for_local_development
```

The app is then at `http://127.0.0.1:8080` — the HTML admin/guest pages at `/`, `/posts/...`,
`/admin/...`, and the JSON API at `/api/v1/...`, sharing the same cookie session.

## Environment variables reference

| Variable | Required | Default | Purpose |
|---|---|---|---|
| `DATABASE_URL` | Yes | — | Connection string; scheme (`postgres://` / `sqlite://`) must match the compiled-in feature |
| `MAGIC_KEY` | Yes | — | Key used to encrypt/compare stored passwords |
| `BIND_ADDR` | No | `127.0.0.1:8080` | Socket the server listens on. Set to `0.0.0.0:8080` in containers — see [Docker in depth](#docker-in-depth) |
| `BLOG_VIEWS_ROOT` | No | compile-time path to `blog-views/templates` | Overrides where templates/assets are loaded from at runtime — needed once the binary is relocated (e.g. into a container) |
| `RUST_LOG` | No | forced to `debug` by `main.rs` regardless of this variable (see [ARCHITECTURE.md § Known limitations](ARCHITECTURE.md#known-limitations--technical-debt)) | Log verbosity |
| `BLOG_CLI_CONFIG_DIR` | No | OS config directory | Overrides where `blog-cli` persists its session file — used by tests to avoid touching a real session |

## Using the CLI

```bash
cargo run -p blog-cli -- --help
cargo run -p blog-cli -- register --username user --password pass
cargo run -p blog-cli -- login --username user --password pass
cargo run -p blog-cli -- post list
cargo run -p blog-cli -- post create --title "Hello" --description "World" --category 1
cargo run -p blog-cli -- category list
```

`blog-cli` persists the session cookie from `login` to a per-server-URL file under the OS config
directory, so later commands (`post create`, etc.) reuse it automatically — the same pattern as
`docker login`/`gh auth login`. Point it at a different server with `--server <url>` (defaults to
`http://127.0.0.1:8080`).

`blog-client` is also usable directly as a library if you want to script against the API:

```rust
let client = blog_client::BlogClient::new("http://127.0.0.1:8080");
client.login("user", "password").await?;
let posts = client.list_posts(1).await?;
```

## Testing

Three layers — see [ARCHITECTURE.md § Testing strategy](ARCHITECTURE.md#testing-strategy) for what
each one is actually proving.

### Everything that needs no database (unit tests)

```bash
cargo test --workspace
```

Covers `blog-core` (business logic against in-memory fakes — sub-millisecond, includes `proptest`
coverage of the pagination edge cases), `blog-views`, `blog-client`, `blog-cli`.

### Storage integration tests (real database, per backend)

```bash
# SQLite — each test gets its own temp file, no setup needed
cargo test -p blog-storage --features sqlite

# Postgres — point at a scratch database; each test gets its own schema,
# so this runs fully in parallel, no --test-threads=1 needed
DATABASE_URL=postgres://postgres:postgres@localhost:5432/blog_test \
  cargo test -p blog-storage --features postgres
```

Don't point `DATABASE_URL` at a real database — these tests don't drop anything they create.

### Black-box tests (real binaries, real HTTP)

```bash
cargo test -p blog-tests -- --test-threads=1
```

This builds and runs the actual `blog-server`/`blog-cli` binaries as subprocesses against a
throwaway SQLite database on a free port, and drives them exactly like a real caller would:

- `tests/api_e2e.rs` — through `blog-client` (register/login/CRUD/logout, auth failure paths)
- `tests/html_e2e.rs` — through raw HTTP against the server-rendered HTML pages, both guest and
  admin, with and without a session
- `tests/cli_e2e.rs` — runs the compiled `blog-cli` binary and asserts on its stdout

`--test-threads=1` here is only to keep log output readable (each test spins up its own server, so
there's no shared state to race on) — expect this to take longer than the other layers since it's
building and booting real binaries.

### Everything, the way CI runs it

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings   # CI actually runs this per-crate/per-feature; see below
cargo test --workspace
cargo test -p blog-storage --features sqlite
cargo test -p blog-server --no-default-features --features sqlite
cargo test -p blog-tests -- --test-threads=1
```

The exact job matrix (every crate × every feature combination) lives in
[.github/workflows/rust.yml](.github/workflows/rust.yml).

## Docker in depth

### Build & run

```bash
docker compose up -d --build
```

Builds the `blog-server` image (multi-stage: compiled in `rust:latest`, run in
`debian:bookworm-slim`) and starts it alongside a `postgres:15` `db` service, gated behind a
healthcheck so `web` doesn't start until Postgres is actually ready. Migrations run automatically.
The app is at `http://127.0.0.1:8080`.

```bash
docker compose logs -f web      # follow the app's logs
docker compose down             # stop and remove containers
docker compose down -v          # ...and also drop the Postgres volume (fresh database next time)
```

### Switching to SQLite in Docker

By default the `web` service builds with the `postgres` feature. To build SQLite instead, edit
`docker-compose.yml`:

```yaml
web:
  build:
    args:
      DB_BACKEND: sqlite
  environment:
    DATABASE_URL: sqlite:///data/blog.db   # bind-mount a path for this if you want it to persist
```

There's no `sqlite` service to add — SQLite needs no server, just a file path reachable from
inside the container.

### The two container-specific env vars

Both exist because a compiled binary's assumptions about its own filesystem/network don't hold
once it's running inside a container — see
[ARCHITECTURE.md § Deployment architecture](ARCHITECTURE.md#deployment-architecture) for the full
reasoning:

- **`BIND_ADDR`** — `docker-compose.yml` sets this to `0.0.0.0:8080`; without it the server
  defaults to `127.0.0.1:8080`, which Docker's port mapping can't reach from outside the container.
- **`BLOG_VIEWS_ROOT`** — set to `/app/templates` in the runtime image, where the Dockerfile
  actually copies the templates. The compile-time default path is only valid inside the builder
  stage.

`MAGIC_KEY` in `docker-compose.yml` ships as a placeholder (`change-me-in-production`) — replace it
before using this configuration anywhere but a local sandbox.

### Verifying the running container

```bash
curl -i http://127.0.0.1:8080/posts/page/1        # guest HTML page
curl -i http://127.0.0.1:8080/api/v1/posts?page=1  # JSON API
```

### Building the image without compose

```bash
docker build -t blog-server:local --build-arg DB_BACKEND=postgres .
docker run --rm -p 8080:8080 \
  -e DATABASE_URL=postgres://user:pass@host.docker.internal:5432/dbname \
  -e MAGIC_KEY=dev-key \
  -e BIND_ADDR=0.0.0.0:8080 \
  blog-server:local
```

(`host.docker.internal` reaches a Postgres instance running on your host machine, on macOS/Windows
Docker Desktop; on Linux, use the host's actual IP or run Postgres in the same Docker network.)

## Production deployment

```bash
cp .env.prod.example .env.prod   # fill in real POSTGRES_PASSWORD and MAGIC_KEY
scripts/run.sh prod
```

This uses **[docker-compose.prod.yml](docker-compose.prod.yml)** — a standalone file, not a merge
with the dev `docker-compose.yml` — with three differences that matter for running this somewhere
real:

- **No baked-in secrets.** Every credential (`POSTGRES_PASSWORD`, `MAGIC_KEY`, etc.) comes from
  `.env.prod` via `${VAR:?message}` syntax, which makes Docker Compose **refuse to start at all**
  if a required variable is missing — verified directly: running it without `MAGIC_KEY` set fails
  with `required variable MAGIC_KEY is missing a value`, rather than silently falling back to the
  dev file's `change-me-in-production` placeholder.
- **The Postgres port isn't published to the host at all.** `web` reaches `db` only over the
  internal Docker network (`db:5432`); there's no `ports:` entry for `db` in this file, confirmed
  with `docker port` after bringing the stack up.
- **`restart: unless-stopped`** on both services, so they come back after a host reboot or crash.

`.env.prod` is gitignored — never commit real production secrets. `APP_PORT` (default `8080`) is
the one thing you can safely leave as-is or override without touching the compose file itself.

```bash
scripts/run.sh prod down    # stop and remove containers
scripts/run.sh prod logs    # follow the app's logs
```

## Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| `enable exactly one of the postgres or sqlite features` (compile error) | Built with both or neither DB feature | `cargo run` (postgres, the default) or `cargo run --no-default-features --features sqlite` |
| Login succeeds but the next request looks logged-out (local `cargo run`, not Docker) | Session cookie has `Secure` set, but you're on plain `http://` | Add `cors_for_local_development` to your feature flags (see [Running locally § step 4](#4-run-it)) |
| `docker compose up` fails to bind port 5432 or 8080 | Something else on your machine already using that port | Stop the other process, or remap the port on the left side of `ports:` in `docker-compose.yml` |
| Container starts but `curl` to `127.0.0.1:8080` fails/connection-reset | `BIND_ADDR` not set to `0.0.0.0:...` | Confirm the `web` service's `environment.BIND_ADDR` in `docker-compose.yml` — this is set by default, only relevant if you copied the Dockerfile elsewhere without it |
| Postgres integration tests fail/hang | `DATABASE_URL` unset, or points at an unreachable database | The tests skip themselves (print `skipping: DATABASE_URL not set`) if unset; verify the database is actually reachable otherwise |
| `blog-cli` `whoami` fails right after `login` | `--server` differs between the two invocations | Session is keyed by server URL — use the same `--server` value (or none, for the default) every time |
