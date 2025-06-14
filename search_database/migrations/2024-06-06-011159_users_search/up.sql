CREATE TABLE users_search (
    code VARCHAR(32) NOT NULL,
    user_id VARCHAR(16) NOT NULL,
    username VARCHAR(32) NOT NULL
);

CREATE INDEX idx_users_search_username_trgm ON users_search USING gin (username gin_trgm_ops);