use std::net::SocketAddr;
use std::path::PathBuf;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::key::Key;
use crate::store::Store;

// === Handlers ===

/// Url to shorten, used by the API.
#[derive(Debug, Deserialize)]
struct UrlRequest {
  url: String,
}

/// Existing key-url pair, returned by the API.
#[derive(Debug, Serialize)]
struct UrlResponse {
  key: Key,
  url: String,
}

/// Redirect the key to its associated url.
///
/// We use our own `Result` so we can use `?` in the handler.
async fn redirect(
  Path(key): Path<Key>,
  State(store): State<Store>,
) -> Result<Response> {
  Ok(match store.get(&key)? {
    Some(url) => Redirect::temporary(&url).into_response(),
    None => StatusCode::NOT_FOUND.into_response(),
  })
}

/// Shorten the url and return the key-url pair.
async fn shorten(
  store: State<Store>,
  input: Json<UrlRequest>,
) -> Result<impl IntoResponse> {
  // create a new key
  let key = Key::gen();

  // save it to the store
  store.insert(&key, &input.url)?;

  // return key-url pair
  Ok(Json(UrlResponse {
    key,
    url: input.url.clone(),
  }))
}

// === Server ===

pub struct Server {
  port: u16,
  db_path: PathBuf,
}

impl Server {
  // Create a new server with the given options.
  pub fn new(port: u16, db_path: PathBuf) -> Self {
    Self { port, db_path }
  }

  /// TODO: pass store and logger as arguments
  /// Listen on the configured port.
  pub async fn listen(&self) -> anyhow::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt()
      .with_target(false)
      .compact()
      .init();

    // initialize store
    let store = Store::new(&self.db_path)?;

    // initialize the app
    let app = Router::new()
      .route("/", post(shorten))
      .route("/:key", get(redirect))
      .layer(
        TraceLayer::new_for_http()
          .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
          .on_response(DefaultOnResponse::new().level(Level::INFO)),
      )
      // .layer(logger.layer())
      .with_state(store);

    // run the app
    let addr: SocketAddr = format!("0.0.0.0:{}", self.port).parse().unwrap();
    tracing::info!("listening on {addr}");
    axum::Server::bind(&addr)
      .serve(app.into_make_service())
      .await?;

    Ok(())
  }
}

// == Errors ==

type Result<T> = std::result::Result<T, Error>;

/// Make our own error that wraps `anyhow::Error`.
struct Error(anyhow::Error);

/// Tell axum how to convert `AppError` into a response.
impl IntoResponse for Error {
  fn into_response(self) -> Response {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      format!("Something went wrong: {}", self.0),
    )
      .into_response()
  }
}

/// This enables using `?` on functions that return `Result<_, anyhow::Error>`
/// to turn them into `Result<_, AppError>`. That way you don't need to do that
/// manually.
impl<E> From<E> for Error
where
  E: Into<anyhow::Error>,
{
  fn from(err: E) -> Self {
    Self(err.into())
  }
}
