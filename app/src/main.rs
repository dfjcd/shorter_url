use clap::Parser;
use crate::db::Db;
use crate::short_code_generator::ShortCodeGenerator;

mod db;
mod short_code_generator;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(required = true)]
    url: String
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let db = std::env::var("local_db")?;

    let code = ShortCodeGenerator::generate();

    let db = Db::connect(&db).await?;

    let _ = db.save_short_url(&code, &cli.url).await;

    println!("Added new entry");

    Ok(())
}
