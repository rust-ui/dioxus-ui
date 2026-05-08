use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
}

#[server]
pub async fn get_items() -> Result<Vec<Item>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>();

    let rows = sqlx::query_as!(
        Item,
        "SELECT id, title, description FROM items ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(rows)
}

#[server]
pub async fn get_item(id: Uuid) -> Result<Option<Item>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>();

    let row = sqlx::query_as!(
        Item,
        "SELECT id, title, description FROM items WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(row)
}

#[server]
pub async fn create_item(title: String, description: Option<String>) -> Result<Item, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>();

    let row = sqlx::query_as!(
        Item,
        "INSERT INTO items (title, description) VALUES ($1, $2) RETURNING id, title, description",
        title,
        description
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(row)
}

#[server]
pub async fn delete_item(id: Uuid) -> Result<(), ServerFnError> {
    let pool = use_context::<sqlx::PgPool>();

    sqlx::query!("DELETE FROM items WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
