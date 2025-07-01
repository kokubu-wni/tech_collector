CREATE TABLE IF NOT EXISTS repository_records (
    id SERIAL PRIMARY KEY,
    github_id BIGINT UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    description TEXT,
    url VARCHAR(500) NOT NULL,
    stars INTEGER NOT NULL,
    language VARCHAR(100),
    topics TEXT[] DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    collected_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_repository_records_stars ON repository_records(stars DESC);
CREATE INDEX idx_repository_records_collected_at ON repository_records(collected_at DESC);
CREATE INDEX idx_repository_records_language ON repository_records(language);
CREATE INDEX idx_repository_records_github_id ON repository_records(github_id);