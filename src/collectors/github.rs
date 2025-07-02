use crate::models::{CollectedRepository, GitHubApiRepository};
use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;

pub struct GitHubCollector {
    client: Client,
    token: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubApiResponse {
    items: Vec<GitHubApiRepository>,
    total_count: u32,
}

impl GitHubCollector {
    pub fn new(token: String) -> Self {
        let client = Client::builder()
            .user_agent("tech_collector/0.1.0")
            .build()
            .expect("Failed to create HTTP client");
        Self { client, token }
    }

    async fn search_repositories(&self, query: &str) -> Result<Vec<CollectedRepository>> {
        let url = "https://api.github.com/search/repositories";

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("token {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .query(&[
                ("q", query),
                ("sort", "stars"),
                ("order", "desc"),
                ("per_page", "30"),
            ])
            .send()
            .await?;
        if !response.status().is_success() {
            anyhow::bail!("GitHub API error: {}", response.status());
        }

        let search_response: GitHubApiResponse = response.json().await?;

        Ok(search_response
            .items
            .into_iter()
            .map(CollectedRepository::from)
            .collect())
    }

    pub async fn search_ml_repositories(&self) -> Result<Vec<CollectedRepository>> {
        let mut all_repos = Vec::new();

        let queries = vec![
            "language:python deep-learning stars:>50 created:>2022-01-01",
            "topic:llm stars:>1000",
            "topic:transformer stars:>1000",
        ];

        for query in queries {
            let repos = self.search_repositories(query).await?;
            all_repos.extend(repos);
        }

        let mut unique_repos = std::collections::HashMap::new();
        for repo in all_repos {
            unique_repos.insert(repo.github_id, repo);
        }

        Ok(unique_repos.into_values().collect())
    }
}
