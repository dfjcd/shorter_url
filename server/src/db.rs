use sqlx::{QueryBuilder, Row, Sqlite, SqlitePool};

pub struct Db {
    connection: SqlitePool,
}

impl Db {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        let connection = SqlitePool::connect(url).await?;
        Ok(Self { connection })
    }

    pub async fn find_url(&self, code: &str) -> Option<String> {
        let mut query = QueryBuilder::<Sqlite>::new("SELECT url FROM short_urls WHERE short_code = ");
        query.push_bind(code);

        let row = query.build().fetch_one(&self.connection).await;
        match row {
            Ok(r) => {
                if r.is_empty() {
                    return None;
                }

                let result: String = r.get(0);
                println!("QUERY RESULT: {result}");

                Some(result)
            }
            Err(_) => None
        }
    }
}