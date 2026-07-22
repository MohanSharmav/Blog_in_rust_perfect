use crate::error::{self, ClientError};
use crate::types::{Category, Credentials, NewCategory, NewPost, Page, Post};
use reqwest::header::{HeaderMap, COOKIE, SET_COOKIE};
use reqwest::Method;
use std::sync::RwLock;

/// A typed async client for the blog-server JSON API.
///
/// Authentication reuses the server's existing cookie session: after
/// [`BlogClient::login`] succeeds, the client holds the session cookie in
/// memory and attaches it to subsequent requests. Callers that need the
/// session to survive across process restarts (like a CLI) should persist
/// [`BlogClient::session_cookie`] themselves and restore it via
/// [`BlogClient::with_session`].
pub struct BlogClient {
    http: reqwest::Client,
    base_url: String,
    session_cookie: RwLock<Option<String>>,
}

impl BlogClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.into(),
            session_cookie: RwLock::new(None),
        }
    }

    /// Builds a client that starts out already authenticated with a
    /// previously captured [`session_cookie`](Self::session_cookie).
    pub fn with_session(base_url: impl Into<String>, session_cookie: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.into(),
            session_cookie: RwLock::new(Some(session_cookie.into())),
        }
    }

    /// The current session cookie, if logged in. Persist this to restore the
    /// session in a later process with [`with_session`](Self::with_session).
    pub fn session_cookie(&self) -> Option<String> {
        self.session_cookie.read().unwrap().clone()
    }

    fn url(&self, path: &str) -> String {
        format!("{}{path}", self.base_url)
    }

    fn request(&self, method: Method, path: &str) -> reqwest::RequestBuilder {
        let mut builder = self.http.request(method, self.url(path));
        if let Some(cookie) = self.session_cookie() {
            builder = builder.header(COOKIE, cookie);
        }
        builder
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<(), ClientError> {
        let response = self
            .http
            .post(self.url("/api/v1/login"))
            .json(&Credentials {
                username: username.to_string(),
                password: password.to_string(),
            })
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(ClientError::InvalidCredentials);
        }
        if !response.status().is_success() {
            return Err(error::from_response(response).await);
        }

        let cookie = extract_cookie_header(response.headers());
        *self.session_cookie.write().unwrap() = cookie;
        Ok(())
    }

    pub async fn register(&self, username: &str, password: &str) -> Result<(), ClientError> {
        let response = self
            .http
            .post(self.url("/api/v1/register"))
            .json(&Credentials {
                username: username.to_string(),
                password: password.to_string(),
            })
            .send()
            .await?;

        expect_success(response).await
    }

    pub async fn logout(&self) -> Result<(), ClientError> {
        let response = self.request(Method::POST, "/api/v1/logout").send().await?;
        *self.session_cookie.write().unwrap() = None;
        expect_success(response).await
    }

    /// Returns the logged-in username, or [`ClientError::NotAuthenticated`].
    pub async fn me(&self) -> Result<String, ClientError> {
        let response = self.request(Method::GET, "/api/v1/me").send().await?;
        if !response.status().is_success() {
            return Err(error::from_response(response).await);
        }
        let body: serde_json::Value = response.json().await?;
        Ok(body["username"].as_str().unwrap_or_default().to_string())
    }

    pub async fn list_posts(&self, page: i64) -> Result<Page<Post>, ClientError> {
        let response = self
            .request(Method::GET, "/api/v1/posts")
            .query(&[("page", page)])
            .send()
            .await?;
        parse_json(response).await
    }

    pub async fn get_post(&self, id: i32) -> Result<Post, ClientError> {
        let response = self
            .request(Method::GET, &format!("/api/v1/posts/{id}"))
            .send()
            .await?;
        parse_json(response).await
    }

    pub async fn create_post(&self, new_post: &NewPost) -> Result<(), ClientError> {
        let response = self
            .request(Method::POST, "/api/v1/posts")
            .json(new_post)
            .send()
            .await?;
        expect_success(response).await
    }

    pub async fn update_post(&self, id: i32, updated: &NewPost) -> Result<(), ClientError> {
        let response = self
            .request(Method::PUT, &format!("/api/v1/posts/{id}"))
            .json(updated)
            .send()
            .await?;
        expect_success(response).await
    }

    pub async fn delete_post(&self, id: i32) -> Result<(), ClientError> {
        let response = self
            .request(Method::DELETE, &format!("/api/v1/posts/{id}"))
            .send()
            .await?;
        expect_success(response).await
    }

    pub async fn list_categories(&self, page: i64) -> Result<Page<Category>, ClientError> {
        let response = self
            .request(Method::GET, "/api/v1/categories")
            .query(&[("page", page)])
            .send()
            .await?;
        parse_json(response).await
    }

    pub async fn get_category(&self, id: i32) -> Result<Category, ClientError> {
        let response = self
            .request(Method::GET, &format!("/api/v1/categories/{id}"))
            .send()
            .await?;
        parse_json(response).await
    }

    pub async fn create_category(&self, name: &str) -> Result<(), ClientError> {
        let response = self
            .request(Method::POST, "/api/v1/categories")
            .json(&NewCategory {
                name: name.to_string(),
            })
            .send()
            .await?;
        expect_success(response).await
    }

    pub async fn update_category(&self, id: i32, name: &str) -> Result<(), ClientError> {
        let response = self
            .request(Method::PUT, &format!("/api/v1/categories/{id}"))
            .json(&NewCategory {
                name: name.to_string(),
            })
            .send()
            .await?;
        expect_success(response).await
    }

    pub async fn delete_category(&self, id: i32) -> Result<(), ClientError> {
        let response = self
            .request(Method::DELETE, &format!("/api/v1/categories/{id}"))
            .send()
            .await?;
        expect_success(response).await
    }
}

async fn expect_success(response: reqwest::Response) -> Result<(), ClientError> {
    if response.status().is_success() {
        Ok(())
    } else {
        Err(error::from_response(response).await)
    }
}

async fn parse_json<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, ClientError> {
    if !response.status().is_success() {
        return Err(error::from_response(response).await);
    }
    Ok(response.json().await?)
}

/// Collects every `Set-Cookie` header into a single `name=value; ...` string
/// suitable for a `Cookie` request header, discarding attributes
/// (`Path`, `HttpOnly`, `Secure`, ...) that only make sense to a browser.
fn extract_cookie_header(headers: &HeaderMap) -> Option<String> {
    let pairs: Vec<String> = headers
        .get_all(SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .filter_map(|raw| cookie::Cookie::parse(raw.to_string()).ok())
        .map(|c| format!("{}={}", c.name(), c.value()))
        .collect();

    if pairs.is_empty() {
        None
    } else {
        Some(pairs.join("; "))
    }
}
