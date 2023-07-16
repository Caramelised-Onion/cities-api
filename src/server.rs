use std::net::SocketAddr;

use axum::{http, routing::get, Router};
use http::Method;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    db::connect_to_db,
    routes::{get_cities, get_distance, get_random_city, root, get_country},
};

pub async fn run_server() {
    let pool = connect_to_db().await;
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/rand", get(get_random_city))
        .route("/cities", get(get_cities))
        .route("/distance", get(get_distance))
        .route("/countries", get(get_country))
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
