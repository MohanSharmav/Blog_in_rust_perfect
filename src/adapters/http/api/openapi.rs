//! OpenAPI (Swagger) documentation for the `/api/v1` JSON API.
//!
//! The schema structs below are documentation-only — they mirror the real
//! wire shapes structurally but are defined independently of
//! `blog-storage`'s domain types, the same way `blog-client`'s types are
//! (see `blog-client/src/types.rs`): the JSON payload is the actual
//! contract, not a shared Rust type. This also keeps `utoipa` out of
//! `blog-core`/`blog-storage` entirely, consistent with those crates having
//! no other HTTP-framework dependency (see
//! ARCHITECTURE.md § Architectural style).
//!
//! Every struct below exists only for `#[derive(ToSchema)]` to introspect —
//! none is ever constructed as a real value, hence the blanket `dead_code`
//! allow rather than one per struct.
#![allow(dead_code)]

#[allow(unused_imports)]
use serde_json::json;
use utoipa::{OpenApi, ToSchema};

// `#[schema(as = openapi::X)]` on every struct below pins the *registered*
// component name to match how `#[utoipa::path]` in auth.rs/posts.rs/
// categories.rs references these types — as `openapi::X`, since those files
// already import the real `ApiError`/`Credentials`/`NewPost`/`NewCategory`
// domain/wire types under those same bare names and would collide otherwise.
// Without this, utoipa derives a *different* name at each of the two sites
// (the qualified reference vs. the bare definition here), producing a
// `$ref` that points at a component that was never actually registered —
// verified directly: before adding these, every single schema reference in
// the generated `openapi.json` was broken this way.

#[derive(ToSchema)]
#[schema(as = openapi::Credentials, example = json!({"username": "alice", "password": "hunter2"}))]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// Returned by `POST /login` and `GET /me`.
#[derive(ToSchema)]
#[schema(as = openapi::UsernameResponse)]
pub struct UsernameResponse {
    pub username: String,
}

#[derive(ToSchema)]
#[schema(as = openapi::Post)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(ToSchema)]
#[schema(as = openapi::NewPost, example = json!({"title": "Hello", "description": "World", "category_id": 0}))]
pub struct NewPost {
    pub title: String,
    pub description: String,
    /// `0` means "no category".
    pub category_id: i32,
}

#[derive(ToSchema)]
#[schema(as = openapi::Category)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(ToSchema)]
#[schema(as = openapi::NewCategory, example = json!({"name": "rust"}))]
pub struct NewCategory {
    pub name: String,
}

#[derive(ToSchema)]
#[schema(as = openapi::PageOfPosts)]
pub struct PageOfPosts {
    pub items: Vec<Post>,
    pub page: usize,
    pub total_pages: usize,
    pub total_items: i64,
}

#[derive(ToSchema)]
#[schema(as = openapi::PageOfCategories)]
pub struct PageOfCategories {
    pub items: Vec<Category>,
    pub page: usize,
    pub total_pages: usize,
    pub total_items: i64,
}

#[derive(ToSchema)]
#[schema(as = openapi::ApiError)]
pub struct ApiError {
    pub error: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        super::auth::login,
        super::auth::logout,
        super::auth::register,
        super::auth::me,
        super::posts::list,
        super::posts::get,
        super::posts::create,
        super::posts::update,
        super::posts::delete,
        super::categories::list,
        super::categories::get,
        super::categories::create,
        super::categories::update,
        super::categories::delete,
    ),
    components(schemas(
        Credentials,
        UsernameResponse,
        Post,
        NewPost,
        Category,
        NewCategory,
        PageOfPosts,
        PageOfCategories,
        ApiError
    )),
    tags(
        (name = "auth", description = "Registration, login, and the current session"),
        (name = "posts", description = "Blog posts"),
        (name = "categories", description = "Post categories"),
    ),
    info(
        title = "Blog API",
        description = "The `/api/v1` JSON API — see API.md in the repository for the full \
                        reference, including the server-rendered HTML routes this doesn't cover.",
        version = "1.0.0"
    )
)]
pub struct ApiDoc;
