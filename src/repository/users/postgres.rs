use std::collections::HashMap;
use futures::TryStreamExt;
use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::repository::{error::QueryError, postgres::PostgresDatabase};

use super::{UserSearchRecord, UserSearchRepository};

impl UserSearchRepository for PostgresDatabase {
    async fn create(&self, user_id: &str, username: &str) -> Result<(), QueryError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO users_search (user_id, username, code)
            VALUES ($1, $2, DMETAPHONE($2))
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(user_id)
            .bind(username)
            .execute(conn.as_mut())
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn query<'a>(&self, terms: &'a Vec<&str>) -> Result<Vec<UserSearchRecord>, QueryError> {
        const SEARCH_QUERY: &'static str = r#"
            SELECT s.user_id as uid, s.username as u, s.username <-> q.value AS s
            FROM UNNEST($1) as q(value)
            JOIN users_search s 
            ON s.username % q.value
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let mut results = sqlx::query(SEARCH_QUERY)
            .bind(terms)
            .map(|row: PgRow| (
                row.get::<String, &str>("uid"),
                row.get("u"),
                1. - row.get::<f32, &str>("s"),
            ))
            .fetch(conn.as_mut());

        let mut rows = HashMap::new();

        while let Some((user_id, username, score)) = results.try_next().await? {
            let user_id = user_id
                .to_string()
                .clone();   

            rows.entry(user_id.to_owned())
                .or_insert_with(|| UserSearchRecord {
                    user_id,
                    username,
                    score: 0.0,
                })
                .score += score
        }

        let mut rows: Vec<UserSearchRecord> = rows.into_values().collect();

        rows.sort_by(|l, r| r.score.total_cmp(&l.score));
        rows.truncate(32);

        Ok(rows)
    }
}