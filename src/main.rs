use clap::Parser;

mod routes;
mod models;
mod seed;
mod server;
mod db;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    seed: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.seed {
        Some(seed) => seed::seed_db(&seed).await,
        None => server::run_server().await,
    }
}
