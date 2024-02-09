use serde::Serialize;
use sqlx::{Error, Pool, Sqlite};

#[derive(Serialize, sqlx::FromRow)]
pub struct Link {
    id: String,
    pub href: String,
}

impl Link {
    pub fn new(id: String, href: String) -> Link {
        Link { id, href }
    }
}

pub async fn insert(pool: Pool<Sqlite>, link: Link) -> Result<Link, Error> {
    sqlx::query_as!(
        Link,
        "INSERT INTO links (id, href) VALUES (?1, ?2) ON CONFLICT(id) DO UPDATE SET href=?2 RETURNING id, href;",
        link.id,
        link.href
    )
    .fetch_one(&pool)
    .await
}

pub async fn delete(pool: Pool<Sqlite>, id: String) -> Result<bool, Error> {
    Ok(sqlx::query!("DELETE FROM links WHERE id=?", id)
        .execute(&pool)
        .await?
        .rows_affected()
        > 0)
}

pub async fn get(pool: Pool<Sqlite>, id: String) -> Result<Option<Link>, Error> {
    sqlx::query_as!(Link, "SELECT * FROM links WHERE id=?", id)
        .fetch_optional(&pool)
        .await
}

pub async fn get_all(pool: Pool<Sqlite>) -> Result<Vec<Link>, Error> {
    sqlx::query_as!(Link, "SELECT * FROM links")
        .fetch_all(&pool)
        .await
}
