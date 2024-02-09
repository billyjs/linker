use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/:id", get(redirect))
        .route("/:id", post(create_link))
        .route("/:id", delete(delete_link))
        .route("/debug/:id", get(get_link))
        .route("/debug", get(get_all_links))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn redirect(Path(id): Path<String>) -> String {
    format!("Hello, {}", id)
}

async fn create_link(Path(id): Path<String>, Json(payload): Json<CreateLink>) -> impl IntoResponse {
    let link = Link {
        id,
        href: payload.href,
    };

    (StatusCode::CREATED, Json(link))
}

async fn delete_link(Path(_id): Path<String>) -> impl IntoResponse {
    StatusCode::OK
}

async fn get_link(Path(id): Path<String>) -> impl IntoResponse {
    let link = Link {
        id,
        href: "href".to_string(),
    };

    (StatusCode::OK, Json(link))
}

async fn get_all_links() -> impl IntoResponse {
    let link1 = Link {
        id: "1".to_string(),
        href: "href".to_string(),
    };

    let link2 = Link {
        id: "2".to_string(),
        href: "href".to_string(),
    };

    (StatusCode::OK, Json(vec![link1, link2]))
}

#[derive(Deserialize)]
struct CreateLink {
    href: String,
}

#[derive(Serialize)]
struct Link {
    id: String,
    href: String,
}
