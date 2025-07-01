use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::{RepositoryRecord, RepositoryListResponse};

#[derive(Debug, Deserialize)]
pub struct ListParams {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

fn default_page() -> i32 {
    1
}

fn default_per_page() -> i32 {
    20
}

// リポジトリ一覧を取得
pub async fn list_repositories(
    State(pool): State<PgPool>,
    Query(params): Query<ListParams>,
) -> Result<Json<RepositoryListResponse>, StatusCode> {
    let offset = (params.page - 1) * params.per_page;
    
    // リポジトリを取得
    // let repositories = sqlx::query_as!(
    //     RepositoryRecord,
    //     r#"
    //     SELECT * FROM repository_records
    //     ORDER BY collected_at DESC, stars DESC
    //     LIMIT $1 OFFSET $2
    //     "#,
    //     params.per_page as i64,
    //     offset as i64
    // )
    // .fetch_all(&pool)
    // .await
    // .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let repositories = sqlx::query_as::<_, RepositoryRecord>(
        r#"
        SELECT * FROM repository_records
        ORDER BY collected_at DESC, stars DESC
        LIMIT $1 OFFSET $2
        "#
    )
    .bind(params.per_page as i64)
    .bind(offset as i64)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // 総数を取得
    // let total_result = sqlx::query!(
    //     "SELECT COUNT(*) as count FROM repository_records"
    // )
    // .fetch_one(&pool)
    // .await
    // .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // let total = total_result.count.unwrap_or(0);

    let total_row = sqlx::query("SELECT COUNT(*) as count FROM repository_records")
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // let total: i64 = total_row.get("count");

    #[derive(sqlx::FromRow)]
    struct CountResult {
        count: Option<i64>,
    }

    let total_result = sqlx::query_as::<_, CountResult>(
        "SELECT COUNT(*) as count FROM repository_records"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let total = total_result.count.unwrap_or(0);

    Ok(Json(RepositoryListResponse {
        repositories,
        total,
        page: params.page,
        per_page: params.per_page,
    }))
}