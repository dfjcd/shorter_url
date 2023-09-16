use crate::db::Db;

mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Db::connect("db/sqlite.db").await?;
    let r = db.save_short_url("short_code", "https://google.com").await;
    r
}
