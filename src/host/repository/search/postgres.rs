use crate::host::repository::{error::QueryError, postgres::PostgresDatabase};

use super::SearchRepository;

impl SearchRepository for PostgresDatabase {
    async fn create(&self, result_type: i16, value: &str) -> Result<(), QueryError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO search (result_type, value, dmeta)
            VALUES ($1, $2, DMETAPHONE($2))
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(result_type)
            .bind(value)
            .execute(conn.as_mut())
            .await?;

        Ok(())
    }

    async fn search(&self, query: &str) -> Result<impl Iterator<Item = (i16, String)>, QueryError> {
        const SEARCH_QUERY: &'static str = r#"
            SELECT
                search.result_type, search.value, search.value <-> query.value AS score
            FROM 
                search 
                JOIN (
                    SELECT p as value, DMETAPHONE(p) AS dmeta 
                    FROM UNNEST($1) as query(p)
                ) AS query
                    ON search.dmeta = query.dmeta
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let mut rows = sqlx::query_as::<_, (i16, String, f32)>(SEARCH_QUERY)
            .bind(query.split(' ').collect::<Vec<&str>>())
            .fetch_all(conn.as_mut())
            .await?;

        rows.sort_by(|l, r| l.2.total_cmp(&r.2));

        Ok(rows
            .into_iter()
            .map(|(result_type, value, _)| (result_type, value))
            .take(32)
        )
    }
}