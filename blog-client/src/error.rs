use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("not authenticated — run `login` first")]
    NotAuthenticated,
    #[error("invalid username or password")]
    InvalidCredentials,
    #[error("not found")]
    NotFound,
    #[error("server rejected the request ({status}): {message}")]
    Api {
        status: reqwest::StatusCode,
        message: String,
    },
}

#[derive(Deserialize)]
struct ApiErrorBody {
    error: String,
}

/// Turns a non-2xx response into a [`ClientError`], reading the JSON
/// `{"error": "..."}` body the server sends when present.
pub(crate) async fn from_response(response: reqwest::Response) -> ClientError {
    let status = response.status();
    if status == reqwest::StatusCode::UNAUTHORIZED {
        return ClientError::NotAuthenticated;
    }
    if status == reqwest::StatusCode::NOT_FOUND {
        return ClientError::NotFound;
    }

    let body = response.text().await.unwrap_or_default();
    let message = serde_json::from_str::<ApiErrorBody>(&body)
        .map(|e| e.error)
        .unwrap_or(body);

    ClientError::Api { status, message }
}
