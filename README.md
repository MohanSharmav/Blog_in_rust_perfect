# Blog

A small blog engine in Rust: an Actix-web server rendering server-side HTML *and* exposing a
JSON API, a typed async client for that API, and a CLI built on top of the client. The storage
backend (Postgres or SQLite) is a compile-time choice via Cargo features, not a runtime one.

This file used to be a forward-looking refactor plan. All of it has since been built; this
version describes what's actually here.

See [ARCHITECTURE.md](ARCHITECTURE.md) for how the pieces fit together — the hexagonal
architecture / ports-and-adapters design, the port pattern used throughout, and a full
request-flow walkthrough. See [RUNNING.md](RUNNING.md) for how to actually run it — locally, in
Docker, and how to run the test suite. See [API.md](API.md) for every HTTP endpoint the server
exposes — the JSON API and the server-rendered HTML routes, request/response shapes, and auth
requirements. See [DATA_MODEL.md](DATA_MODEL.md) for UML class diagrams of the structs, enums, and
traits and how they relate — domain types and repository ports, the error-type hierarchy, and the
client-side wire types.

## Workspace layout

```
.
├── Cargo.toml            # workspace manifest (members list only)
├── blog-server/          # Controller: HTTP delivery
│   └── src/adapters/
│       ├── http/          #   Actix handlers: HTML admin pages, guest pages, and the /api/v1 JSON API
│       └── crypto/        #   magic-crypt implementation of the PasswordCipher port
├── blog-core/            # use-case services (Model, business-logic half): posts/categories/auth,
│   │                      # pagination rules, Credentials, the PasswordCipher port — no HTTP/SQL deps
├── blog-storage/         # use-case services' other half (Model, data): domain types (Post,
│   │                      # Category, ...), repository ports, Postgres/SQLite implementations
│   ├── src/postgres/     #   `postgres` feature
│   ├── src/sqlite/       #   `sqlite` feature
│   ├── migrations/postgres/
│   ├── migrations/sqlite/
│   └── tests/            #   integration tests against a real database per backend
├── blog-views/           # View: bundles templates/ (Handlebars + static assets) and exposes
│   │                      # a `register()` helper — blog-server never hardcodes their location
│   └── templates/
├── blog-client/          # reqwest-based typed async client for the /api/v1 JSON API
├── blog-cli/             # clap CLI on top of blog-client, with persisted login sessions
└── blog-tests/           # black-box tests: spawns real blog-server/blog-cli binaries and
    └── tests/            #   drives them over HTTP/subprocess, the way a real caller would
```

Dependency direction is one-way: `blog-server` depends on `blog-core`, `blog-storage`, and
`blog-views`; `blog-core` depends on `blog-storage` (for its domain types and repository port
traits, never a concrete backend); `blog-storage` and `blog-views` depend on neither of the
others. `blog-client`/`blog-cli` only ever talk to `blog-server` over HTTP — they don't depend
on any of the above.

## Choosing a database backend

`blog-storage` implements its repository ports (`PostRepository`, `CategoryRepository`,
`UserRepository`) twice — once per backend — gated behind Cargo features:

```toml
# blog-storage/Cargo.toml
[features]
postgres = ["sqlx/postgres"]
sqlite   = ["sqlx/sqlite"]
```

`blog-server`'s own `postgres`/`sqlite` features just forward to these. Exactly one must be
enabled in the final binary — `main.rs` has a `compile_error!` guard that fails the build if both
or neither are selected. `postgres` is the default, matching the existing deployment.

**This is deliberately not a runtime `DbPool` enum** (`enum DbPool { Postgres(..), Sqlite(..) }`).
An enum would mean every call site pattern-matches or forwards through a wrapper, both backends
ship in every binary whether you need them or not, and "which database" becomes a value your
code branches on instead of a build you produce. With features, `AppState` in
`adapters::http::state` resolves to one concrete, monomorphic repository type per backend —
swapping databases means rebuilding with a different `--features` flag, not adding a branch. See
[ARCHITECTURE.md § Key design decisions](ARCHITECTURE.md#key-design-decisions) for the trade-off.

For how to actually set up, run, test, and containerize the project, see **[RUNNING.md](RUNNING.md)**.

## blog-client / blog-cli

`blog-client` is a typed async `reqwest` client for the `/api/v1` JSON API; `blog-cli` is a `clap`
CLI built on top of it, with a persisted login session (the same pattern as `docker login`/`gh auth
login`). See [RUNNING.md § Using the CLI](RUNNING.md#using-the-cli) for usage.

## Testing

Unit tests against in-memory fakes, integration tests against real databases, and black-box tests
that run the actual compiled binaries — see
[ARCHITECTURE.md § Testing strategy](ARCHITECTURE.md#testing-strategy) for what each layer proves,
and [RUNNING.md § Testing](RUNNING.md#testing) for the commands to run them.

## CI

`.github/workflows/rust.yml` runs on every push/PR to `main`:

- `fmt` — `cargo fmt --check`
- `clippy` — across every crate (both DB features for `blog-storage`/`blog-server`), denying warnings
- `test-sqlite` — `blog-storage` and `blog-server` built and tested against SQLite (no external service)
- `test-postgres` — same, against a real Postgres service container
- `test-core-and-views` — `blog-core`/`blog-views` unit + integration tests
- `test-client-cli` — `blog-client`/`blog-cli` unit tests
- `test-e2e` — `blog-tests`' black-box suite (spawns real `blog-server`/`blog-cli` binaries, no
  external services needed)
