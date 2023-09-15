use crate::db::Db;

mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Db::connect("db/sqlite.db").await?;
    let r = db.save_short_url("https://shortr.com/short_url").await;
    r
}
