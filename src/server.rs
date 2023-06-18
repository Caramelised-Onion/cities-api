use std::net::SocketAddr;

use axum::{Router, routing::get};

use crate::{routes::{root, get_random_city}, db::connect_to_db};

pub async fn run_server() {
    let pool = connect_to_db().await;

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/rand", get(get_random_city))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}