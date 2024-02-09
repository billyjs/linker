use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{delete, get, put},
    Json, Router,
};
use dotenv::dotenv;
use links::Link;
use serde::Deserialize;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::env;
use tower_http::trace::TraceLayer;

mod links;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let db_connection_str = env::var("DATABASE_URL").expect("no database url found");

    let pool = SqlitePoolOptions::new()
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/:id", get(redirect))
        .route("/:id", put(put_link))
        .route("/:id", delete(delete_link))
        .route("/debug/:id", get(get_link))
        .route("/debug", get(get_all_links))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn redirect(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<String>,
) -> Result<Redirect, (StatusCode, String)> {
    match links::get(pool, id).await.map_err(internal_error)? {
        Some(link) => Ok(Redirect::to(&link.href)),
        None => Err((StatusCode::NOT_FOUND, String::from("Not Found"))),
    }
}

async fn put_link(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<String>,
    Json(payload): Json<Payload>,
) -> Result<(StatusCode, Json<Link>), (StatusCode, String)> {
    let link = links::insert(pool, Link::new(id, payload.href))
        .await
        .map_err(internal_error)?;
    Ok((StatusCode::CREATED, Json(link)))
}

async fn delete_link(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    links::delete(pool, id).await.map_err(internal_error)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn get_link(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<Link>), (StatusCode, String)> {
    match links::get(pool, id).await.map_err(internal_error)? {
        Some(link) => Ok((StatusCode::OK, Json(link))),
        None => Err((StatusCode::NOT_FOUND, String::from("Not Found"))),
    }
}

async fn get_all_links(
    State(pool): State<Pool<Sqlite>>,
) -> Result<(StatusCode, Json<Vec<Link>>), (StatusCode, String)> {
    let links = links::get_all(pool).await.map_err(internal_error)?;
    Ok((StatusCode::OK, Json(links)))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[derive(Deserialize)]
struct Payload {
    href: String,
}
