# API & HTTP Endpoint Reference

Every route the server exposes: the JSON API (`/api/v1/...`) and the server-rendered HTML surface
(guest pages and the admin app). For the architecture behind these (which `blog-core` function each
one calls, the port pattern, etc.), see [ARCHITECTURE.md](ARCHITECTURE.md); for how to run the
server itself, see [RUNNING.md](RUNNING.md).

The JSON API also has interactive, always-up-to-date documentation generated straight from the
handler code (`utoipa`) — see [Interactive Swagger UI](#interactive-swagger-ui). Everything below
is the hand-written reference, including the HTML routes Swagger doesn't cover.

## Table of contents

1. [Interactive Swagger UI](#interactive-swagger-ui)
2. [Conventions](#conventions)
3. [JSON API — Authentication](#json-api--authentication)
4. [JSON API — Posts](#json-api--posts)
5. [JSON API — Categories](#json-api--categories)
6. [Wire types](#wire-types)
7. [HTML routes — guest](#html-routes--guest)
8. [HTML routes — admin](#html-routes--admin)
9. [Full route table](#full-route-table)

## Interactive Swagger UI

Once the server is running (see [RUNNING.md](RUNNING.md)), the full JSON API spec is browsable
and callable at:

- **Swagger UI**: `http://127.0.0.1:8080/swagger-ui/`
- **Raw OpenAPI 3.0 document**: `http://127.0.0.1:8080/api-docs/openapi.json`

The spec is generated from the same `#[utoipa::path(...)]` annotations on the actual handler
functions in [src/adapters/http/api/](blog-server/src/adapters/http/api/) — it can't drift out of sync with
the code the way a hand-maintained spec can. The schema types themselves live in
[src/adapters/http/api/openapi.rs](blog-server/src/adapters/http/api/openapi.rs), defined independently of
`blog-storage`'s domain types for the same reason `blog-client`'s wire types are (see
[ARCHITECTURE.md § The port pattern](ARCHITECTURE.md#the-port-pattern)) — this also keeps
`utoipa` out of `blog-core`/`blog-storage` entirely.

Swagger UI doesn't cover the server-rendered HTML routes (guest pages, the admin app) — those are
only in the [reference below](#html-routes--guest).

## Conventions

- **Base URL**: `http://127.0.0.1:8080` by default (see [RUNNING.md § Environment variables reference](RUNNING.md#environment-variables-reference) for `BIND_ADDR`).
- **Auth**: a single cookie-based session, shared between the JSON API and the HTML app — logging
  in through either surface authenticates both. There is no bearer token / API key scheme.
- **JSON error shape**, returned by every JSON API error response:
  ```json
  { "error": "human-readable message" }
  ```
- **Validation**: `NewPost.title`/`description` must be non-empty; `NewCategory.name` must be at
  least 2 characters. Violations return `400 Bad Request` with the message above. `Credentials`
  (username/password) has no server-side validation — any non-null strings are accepted, including
  empty ones — and **no uniqueness constraint exists on username**: registering an already-used
  username succeeds silently rather than erroring (see [ARCHITECTURE.md § Known limitations](ARCHITECTURE.md#known-limitations--technical-debt)).
- **Pagination**: list endpoints take `?page=N` (1-indexed, default `1`) and return `404` with
  `{"error": "page out of range"}` if `N` is out of range. Page size is fixed at 3 items.

## JSON API — Authentication

### `POST /api/v1/register`
Creates an account. Does **not** log the new user in.

- **Auth required**: no
- **Body**: `Credentials`
  ```json
  { "username": "alice", "password": "hunter2" }
  ```
- **Responses**:
  - `201 Created` — no body
  - `500` — storage error

### `POST /api/v1/login`
- **Auth required**: no
- **Body**: `Credentials` (same shape as register)
- **Responses**:
  - `200 OK` — sets the session cookie via `Set-Cookie`, body `{"username": "alice"}`
  - `401 Unauthorized` — `{"error": "invalid username or password"}` (wrong password *or* unknown username — the two aren't distinguished)

### `POST /api/v1/logout`
- **Auth required**: no (a no-op if not logged in)
- **Responses**: `200 OK`, empty body

### `GET /api/v1/me`
Returns the logged-in username.

- **Auth required**: yes
- **Responses**:
  - `200 OK` — `{"username": "alice"}`
  - `401 Unauthorized` — `{"error": "authentication required"}`

## JSON API — Posts

### `GET /api/v1/posts?page=N`
- **Auth required**: no
- **Responses**:
  - `200 OK` — a [`Page<Post>`](#wire-types)
  - `404 Not Found` — `{"error": "page out of range"}`

### `GET /api/v1/posts/{id}`
- **Auth required**: no
- **Responses**:
  - `200 OK` — a [`Post`](#wire-types)
  - `404 Not Found` — `{"error": "post not found"}`

### `POST /api/v1/posts`
- **Auth required**: yes (`401` with `{"error": "authentication required"}` if not)
- **Body**: [`NewPost`](#wire-types)
- **Responses**:
  - `201 Created` — no body
  - `400 Bad Request` — validation failure
  - `401 Unauthorized` — not logged in

### `PUT /api/v1/posts/{id}`
- **Auth required**: yes
- **Body**: `NewPost` — full replacement, not a partial patch
- **Responses**: `200 OK` (no body) · `400` · `401`

### `DELETE /api/v1/posts/{id}`
- **Auth required**: yes
- **Responses**: `204 No Content` · `401`

## JSON API — Categories

### `GET /api/v1/categories?page=N`
- **Auth required**: no
- **Responses**: `200 OK` — a [`Page<Category>`](#wire-types) · `404` — page out of range

### `GET /api/v1/categories/{id}`
- **Auth required**: no
- **Responses**: `200 OK` — a `Category` · `404` — `{"error": "category not found"}`

### `POST /api/v1/categories`
- **Auth required**: yes
- **Body**: [`NewCategory`](#wire-types)
- **Responses**: `201 Created` (no body) · `400` · `401`

### `PUT /api/v1/categories/{id}`
- **Auth required**: yes
- **Body**: `NewCategory`
- **Responses**: `200 OK` (no body) · `400` · `401`

### `DELETE /api/v1/categories/{id}`
- **Auth required**: yes
- **Responses**: `204 No Content` · `401`

## Wire types

```rust
struct Credentials { username: String, password: String }

struct Post { id: i32, title: String, description: String }
struct NewPost {
    title: String,        // non-empty
    description: String,  // non-empty
    category_id: i32,     // 0 means "no category"
}

struct Category { id: i32, name: String }
struct NewCategory { name: String }  // >= 2 characters

// Returned by every paginated list endpoint
struct Page<T> {
    items: Vec<T>,
    page: usize,
    total_pages: usize,
    total_items: i64,
}
```

These are defined independently in [`blog-client/src/types.rs`](blog-client/src/types.rs) (the
client's view) and `blog-server`/[`src/adapters/http/api`](blog-server/src/adapters/http/api) (the server's) — the JSON payload is the
actual contract, not a shared Rust type. See
[ARCHITECTURE.md § Known limitations](ARCHITECTURE.md#known-limitations--technical-debt) re: this
being one of three near-duplicate pagination type definitions in the codebase.

## HTML routes — guest

No login required. Server-rendered HTML (Handlebars), not JSON.

| Method | Path | Purpose |
|---|---|---|
| GET | `/` | Redirects to `/posts/page/1` |
| GET | `/posts` | Redirects to `/posts/page/1` |
| GET | `/posts/page/{page_number}` | Paginated post listing |
| GET | `/posts/{post_id}` | Single post |
| GET | `/posts/category/{category_id}/page/{page_number}` | Posts filtered by category |
| GET | `/login` | Login form |
| POST | `/login` | Submits `username`/`password` form fields; on success sets the session cookie and redirects (`303`) to `/admin/posts/page/1`; on failure, redirects back to `/login` with a flash error |
| GET | `/register` | Registration form |
| POST | `/register` | Submits `username`/`password`; redirects (`303`) to `/login` |
| GET or POST | `/logout` | Clears the session, redirects to `/` |
| GET or POST | `/check` | Redirects to `/admin/posts/page/1` if logged in, `/` otherwise |
| GET | `/assets/...` (static files) | Served from `blog-views`' bundled `templates/assets` directory |

## HTML routes — admin

**All of these require login** — an unauthenticated request gets `303 See Other` to `/`, no
exceptions. (This wasn't always true for the mutating routes — see
[ARCHITECTURE.md § Authentication & session model](ARCHITECTURE.md#authentication--session-model)
for the gap that was found and fixed.)

| Method | Path | Purpose |
|---|---|---|
| GET | `/admin/posts/page/{page_number}` | Paginated post listing (admin view) |
| GET | `/admin/posts/new` | New-post form |
| POST | `/admin/posts` | Submits `title`/`description`/`category_id` form fields; creates a post |
| GET | `/admin/posts/{post_id}` | Single post (admin view) |
| GET | `/admin/posts/{post_id}/edit` | Edit-post form |
| POST | `/admin/posts/{post_id}/edit` | Submits the same fields; updates the post |
| GET | `/admin/post/{post_id}/delete` | Deletes the post, redirects to `/admin/posts/page/1` (note: a `GET` that mutates — a "delete link" pattern, not a form) |
| GET | `/admin/categories/new` | New-category form |
| POST | `/admin/categories/new` | Submits `name`; creates a category |
| GET | `/admin/categories/page/{page_number}` | Paginated category listing |
| GET | `/admin/categories/{category_id}/page/{page_number}` | Posts within a category (admin view) |
| GET | `/admin/category/{category_id}/edit` | Edit-category form |
| POST | `/admin/category/{category_id}/edit` | Submits `name`; renames the category |
| GET | `/admin/category/{category_id}/delete` | Deletes the category, redirects to `/admin/categories/page/1` |

## Full route table

Everything in one place, JSON and HTML together:

| Method | Path | Auth | Kind |
|---|---|---|---|
| POST | `/api/v1/register` | no | JSON |
| POST | `/api/v1/login` | no | JSON |
| POST | `/api/v1/logout` | no | JSON |
| GET | `/api/v1/me` | yes | JSON |
| GET | `/api/v1/posts` | no | JSON |
| POST | `/api/v1/posts` | yes | JSON |
| GET | `/api/v1/posts/{id}` | no | JSON |
| PUT | `/api/v1/posts/{id}` | yes | JSON |
| DELETE | `/api/v1/posts/{id}` | yes | JSON |
| GET | `/api/v1/categories` | no | JSON |
| POST | `/api/v1/categories` | yes | JSON |
| GET | `/api/v1/categories/{id}` | no | JSON |
| PUT | `/api/v1/categories/{id}` | yes | JSON |
| DELETE | `/api/v1/categories/{id}` | yes | JSON |
| GET | `/`, `/posts` | no | HTML (redirect) |
| GET | `/posts/page/{n}` | no | HTML |
| GET | `/posts/{id}` | no | HTML |
| GET | `/posts/category/{id}/page/{n}` | no | HTML |
| GET, POST | `/login` | no | HTML |
| GET, POST | `/register` | no | HTML |
| GET, POST | `/logout`, `/check` | no | HTML |
| GET | `/admin/posts/page/{n}` | yes | HTML |
| GET | `/admin/posts/new` | yes | HTML |
| POST | `/admin/posts` | yes | HTML |
| GET | `/admin/posts/{id}` | yes | HTML |
| GET, POST | `/admin/posts/{id}/edit` | yes | HTML |
| GET | `/admin/post/{id}/delete` | yes | HTML |
| GET, POST | `/admin/categories/new` | yes | HTML |
| GET | `/admin/categories/page/{n}` | yes | HTML |
| GET | `/admin/categories/{id}/page/{n}` | yes | HTML |
| GET, POST | `/admin/category/{id}/edit` | yes | HTML |
| GET | `/admin/category/{id}/delete` | yes | HTML |
