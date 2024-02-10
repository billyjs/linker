use anyhow::bail;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{delete, get, put},
    Json, Router,
};
use links::Link;
use serde::Deserialize;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePool},
    Pool, Sqlite,
};
use std::{
    borrow::Cow,
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

mod links;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let env = EnvironmentVariables::from_env()?;

    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::from_str(&env.database_url)?.create_if_missing(true),
    )
    .await
    .expect("can't connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("can't run database migrations");

    let app = Router::new()
        .route("/:id", get(redirect))
        .route("/:id", put(put_link))
        .route("/:id", delete(delete_link))
        .route("/debug/:id", get(get_link))
        .route("/debug", get(get_all_links))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let socket = SocketAddr::from((Ipv4Addr::UNSPECIFIED, env.port));
    let listener = TcpListener::bind(socket).await?;

    tracing::debug!("listening on {}:{}", socket.ip(), socket.port());

    axum::serve(listener, app).await?;

    Ok(())
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

#[derive(Clone, Debug)]
pub struct EnvironmentVariables {
    pub database_url: Cow<'static, str>,
    pub port: u16,
}

impl EnvironmentVariables {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        Ok(Self {
            database_url: match dotenv::var("DATABASE_URL") {
                Ok(url) => url.into(),
                Err(err) => bail!("missing DATABASE_URL: {err}"),
            },
            port: match dotenv::var("PORT") {
                Ok(port) => port.parse()?,
                _ => 8000,
            },
        })
    }
}
