use ::polars::prelude::*;
use std::path::Path;
use axum::{routing::get, Router, Json};
use serde_json::Value;

fn get_lazy_parquet(file: &str) -> Result<DataFrame, PolarsError> {
    let path = Path::new(file);
    let args = ScanArgsParquet::default();

    let lf = match LazyFrame::scan_parquet(path, args) {
        Ok(lf) => {
            let lf = lf
                .filter(
                    col("id").eq(lit(1913790)).or(col("id").eq(lit(3198640)))
                )
                .select([col("id"), col("name")]);

            match lf.collect() {
                Ok(df) => df,
                Err(e) => {
                    println!("Error collecting parquet file: {}", e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
            println!("Error scanning parquet file: {}", e);
            return Err(e);
        }
    };

    Ok(lf)
}

fn convert_df_to_json(df: &DataFrame) -> serde_json::Value {
    serde_json::to_value(df).unwrap()
}

async fn get_fake_output() -> Json<Value> {
    let path = "/Users/keven/web_projects/workspace/150_items.parquet";

    let lf = get_lazy_parquet(path).unwrap();

    let json_value = convert_df_to_json(&lf);

    Json(json_value)

}

async fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/enter", get(get_fake_output))
}

#[tokio::main]
async fn main() {
    let app = app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
