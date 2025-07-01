use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RepositoryRecord {
    pub id: i32,
    pub github_id: i64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub url: String,
    pub stars: i32,
    pub language: Option<String>,
    pub topics: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub collected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectedRepository {
    pub github_id: i64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub url: String,
    pub stars: i32,
    pub language: Option<String>,
    pub topics: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RepositoryListResponse {
    pub repositories: Vec<RepositoryRecord>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Debug, Deserialize)]
pub struct GitHubApiRepository {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stargazers_count: i32,
    pub language: Option<String>,
    pub topics: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<GitHubApiRepository> for CollectedRepository {
    fn from(github: GitHubApiRepository) -> Self {
        CollectedRepository {
            github_id: github.id,
            name: github.name,
            full_name: github.full_name,
            description: github.description,
            url: github.html_url,
            stars: github.stargazers_count,
            language: github.language,
            topics: github.topics,
        }
    }
}