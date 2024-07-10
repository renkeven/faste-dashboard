use ::polars::prelude::*;
use dotenv::dotenv;
use serde_json::Value;
use std::env;

use axum::{routing::get, Router, Json};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};


fn convert_df_to_json(df: &DataFrame) -> serde_json::Value {
    serde_json::to_value(df).unwrap()
}


async fn app() -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([http::Method::GET, http::Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/enter", get(get_fake_output))
        .layer(cors)
}


async fn get_fake_output() -> Json<Value> {
    let test_path = env::var("PRIVATE_PATH").expect("Missing path");

    let lf = tokio::task::spawn_blocking(move || LazyFrame::scan_parquet(
        test_path.to_string(),
        ScanArgsParquet::default())
        .unwrap()
        .select([all()])
        .collect()
        .unwrap()
    ).await.unwrap();

    let json_value = convert_df_to_json(&lf);

    Json(json_value)
}


#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = app().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3848));

    println!("listening on {}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
