# Blog Application Refactoring Plan

This document outlines the architectural improvements and task breakdown for modernizing and modularizing the blog application. 

## 1. Modularization: Cargo Workspace (MVC Separation)
Currently, the application is a single monolithic binary. The goal is to break it down into a **Cargo Workspace** to separate the Model, View, and Controller into distinct packages.

### Proposed Workspace Structure
```
blog_workspace/
├── Cargo.toml               # Workspace root (defines members)
├── blog-core/               # The "Model" - Database schemas, business logic, traits
├── blog-server/             # The "Controller" - Actix-web server, routing, HTTP handlers
├── blog-views/              # The "View" - Handlebars templates and static assets
├── blog-client/             # The Client library
└── blog-cli/                # The CLI application
```

- **blog-core**: Contains all `sqlx` database queries and domain models. It will expose traits (e.g., `PostRepository`, `UserRepository`) so the backend can interact with the data layer without worrying about the underlying DB.
- **blog-server**: Depends on `blog-core` and `blog-views`. It handles HTTP requests, authentication, and session management.
- **blog-views**: Bundles the HTML templates and assets. 

## 2. Client Library & CLI
To interact with the blog without a web browser, we will build a dedicated client library and a command-line interface.

- **blog-client**: A Rust crate utilizing `reqwest` to wrap the HTTP API provided by `blog-server`. 
  - Will handle authentication tokens/cookies.
  - Exposes typed asynchronous methods like `client.create_post(...)` or `client.get_posts(...)`.
- **blog-cli**: A command-line tool built with `clap` that depends on `blog-client`.
  - Commands: `blog-cli login`, `blog-cli post create --title "Hello"`, `blog-cli category list`.
  - Improves administrative workflows and scriptability.

## 3. Integration Testing
With the client library in place, we can write robust, black-box integration tests.

- A dedicated `tests/` directory (or a `blog-tests` crate) will spin up the `blog-server` in a background thread or as a subprocess.
- The tests will use `blog-client` to simulate real API traffic.
- CLI tests can also be added using `assert_cmd` to verify the CLI outputs the correct text when executing commands against the test server.

## 4. Multi-Database Support: SQLite (Testing) & PostgreSQL (Prod)
Currently, the application relies heavily on PostgreSQL. To speed up local development and testing, we will abstract the database layer to support both SQLite and PostgreSQL.

### Implementation Strategy
1. **Repository Pattern**: Instead of passing a `PgPool` directly to controllers, we will pass a dynamic trait object or a generic wrapper, e.g., `Arc<dyn Repository>`.
2. **Conditional Compilation / Generic Pools**: 
   - `blog-core` will include both SQLite and Postgres drivers (`sqlx` features: `sqlite`, `postgres`).
   - We can implement a `DbPool` wrapper enum:
     ```rust
     pub enum DbPool {
         Postgres(sqlx::PgPool),
         Sqlite(sqlx::SqlitePool),
     }
     ```
   - Queries will be standardized where possible, or specific trait implementations will handle SQL dialect differences.
3. **Environment Switch**: The app will check a `DATABASE_URL` at runtime. If it starts with `sqlite://`, it uses the in-memory/local SQLite database (perfect for quick tests). If it starts with `postgres://`, it connects to the production DB.

## 5. Dockerization
To ensure consistent deployments and easy local setups, the application will be Dockerized.

- **Dockerfile**: A multi-stage build file. 
  - *Stage 1 (Builder)*: Uses `rust:latest` to compile the `blog-server` release binary.
  - *Stage 2 (Runtime)*: Uses a minimal image (like `debian:bullseye-slim` or `alpine`), copies the compiled binary and the `blog-views` templates, and exposes the HTTP port (e.g., 8080).
- **docker-compose.yml**: 
  - Defines the `web` service (our blog server).
  - Defines a `db` service (PostgreSQL instance).
  - Handles environment variables so developers can type `docker-compose up -d` and instantly have the entire stack (DB + Web) running locally without installing Rust or PostgreSQL on their host machine.

## Detailed Task Breakdown

| Category | Task ID | Task Title | Detailed Description | Dependencies | Complexity |
|----------|---------|------------|----------------------|--------------|------------|
| Modularization | MOD-1 | Setup Cargo Workspace | Convert the monolithic structure into a Cargo workspace. Create a root Cargo.toml and define members (blog-core, blog-server, blog-views). | None | Low |
| Modularization | MOD-2 | Extract Core (Model) | Create the `blog-core` crate. Move all database structs, queries, and business logic from `src/model` to this crate. Remove web framework dependencies from here. | MOD-1 | High |
| Modularization | MOD-3 | Extract Views | Create the `blog-views` crate. Move the `templates` directory and static assets here. Expose a way to load these into the handlebars registry. | MOD-1 | Low |
| Modularization | MOD-4 | Extract Server (Controller) | Create the `blog-server` crate. Move routing, Actix-web handlers, and authentication here. Configure it to depend on `blog-core` and `blog-views`. | MOD-2, MOD-3 | High |
| Client & CLI | CLI-1 | Create Client Library | Create a new crate `blog-client`. Implement a struct `BlogClient` that uses `reqwest` to make HTTP calls to the server. | MOD-4 | Medium |
| Client & CLI | CLI-2 | Implement Auth in Client | Add login methods to `blog-client` and configure the `reqwest` client to store and send session cookies automatically. | CLI-1 | Medium |
| Client & CLI | CLI-3 | Implement API Wrappers | Add typed methods for fetching posts, creating posts, deleting posts, and managing categories. | CLI-2 | Medium |
| Client & CLI | CLI-4 | Create CLI Application | Create a new crate `blog-cli` using `clap`. Define commands like `login`, `post create`, `post list`. | CLI-1 | Low |
| Client & CLI | CLI-5 | Wire CLI to Client | Implement the execution of the CLI commands by calling the corresponding methods in `blog-client`. Handle output formatting. | CLI-3, CLI-4 | Medium |
| Integration Testing | TST-1 | Test Server Setup | Create a `tests` directory. Write a helper function that spins up the Actix-web server on a random port in a background thread for testing. | MOD-4 | Medium |
| Integration Testing | TST-2 | API Integration Tests | Write tests that instantiate `BlogClient` and test the running server (e.g., successful login, post creation flow). | TST-1, CLI-3 | Medium |
| Integration Testing | TST-3 | CLI Integration Tests | Use `assert_cmd` to test the compiled `blog-cli` binary against the test server, asserting standard output and error codes. | TST-1, CLI-5 | Medium |
| Database Support | DB-1 | Define Repository Trait | In `blog-core`, define a generic `Repository` trait containing async methods for all database operations (e.g., `get_post`, `create_post`). | MOD-2 | Medium |
| Database Support | DB-2 | Implement PostgreSQL Repo | Implement the `Repository` trait for a struct wrapping `sqlx::PgPool`. Port existing queries to this implementation. | DB-1 | High |
| Database Support | DB-3 | Implement SQLite Repo | Implement the `Repository` trait for a struct wrapping `sqlx::SqlitePool`. Adjust SQL queries to be SQLite compatible. | DB-1 | High |
| Database Support | DB-4 | Dynamic DB Initialization | Update the `blog-server` initialization to read the `DATABASE_URL`. If it starts with `sqlite://`, use the SQLite implementation. If `postgres://`, use PostgreSQL. | DB-2, DB-3 | Medium |
| Dockerization | DCK-1 | Write Dockerfile | Create a multi-stage Dockerfile for `blog-server`. Build the binary in a Rust builder image and copy it to a minimal Debian/Alpine runtime image. | MOD-4 | Medium |
| Dockerization | DCK-2 | Docker Compose Setup | Write a `docker-compose.yml` that defines a `db` service (PostgreSQL) and a `web` service (the blog app). Set up network and volumes. | DCK-1 | Low |
| Dockerization | DCK-3 | Configuration for Docker | Provide a `.env.docker` file and update the `docker-compose.yml` to inject the correct `DATABASE_URL` so the app can connect to the postgres container. | DCK-2 | Low |
