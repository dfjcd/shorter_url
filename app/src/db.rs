use sqlx::{QueryBuilder, Sqlite, SqlitePool};

pub struct Db {
    connection: SqlitePool,
}

impl Db {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        let connection = SqlitePool::connect(url).await?;
        Ok(Self { connection })
    }

    pub async fn save_short_url(&self, short_code: &str, url: &str) -> anyhow::Result<()> {
        let mut qb = QueryBuilder::<Sqlite>::new("INSERT INTO short_urls(short_code, url) VALUES(");
        let query = qb.push_bind(short_code).push(", ").push_bind(url).push(")").build();

        query.execute(&self.connection).await?;

        Ok(())
    }
}