use chrono::Utc;
use sqlx::PgPool;

use crate::models::{CollectedRepository, RepositoryRecord};

pub struct RepositoryDb<'a> {
    pool: &'a PgPool,
}

impl<'a> RepositoryDb<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert_repository(
        &self,
        repo: &CollectedRepository,
    ) -> Result<RepositoryRecord, sqlx::Error> {
        let now = Utc::now();

        let record = sqlx::query_as::<_, RepositoryRecord>(
            r#"
            INSERT INTO repository_records
                (github_id, name, full_name, description, url, stars, language, topics, created_at, updated_at, collected_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (github_id)
            DO UPDATE SET
                name = EXCLUDED.name,
                full_name = EXCLUDED.full_name,
                description = EXCLUDED.description,
                url = EXCLUDED.url,
                stars = EXCLUDED.stars,
                language = EXCLUDED.language,
                topics = EXCLUDED.topics,
                updated_at = EXCLUDED.updated_at,
                collected_at = EXCLUDED.collected_at
            RETURNING *"#
        )
        .bind(repo.github_id)
        .bind(&repo.name)
        .bind(&repo.full_name)
        .bind(&repo.description)
        .bind(&repo.url)
        .bind(repo.stars)
        .bind(&repo.language)
        .bind(&repo.topics)
        .bind(now)
        .bind(now)
        .bind(now)
        .fetch_one(self.pool)
        .await?;

        Ok(record)
    }

    pub async fn upsert_repositories(
        &self,
        repos: &[CollectedRepository],
    ) -> Result<Vec<RepositoryRecord>, sqlx::Error> {
        let mut records = Vec::new();

        for repo in repos {
            match self.upsert_repository(repo).await {
                Ok(record) => records.push(record),
                Err(e) => {
                    tracing::error!("Failed to save repository {}: {:?}", repo.name, e);
                }
            }
        }

        Ok(records)
    }
}
