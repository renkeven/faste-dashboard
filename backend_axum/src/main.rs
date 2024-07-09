use ::polars::prelude::*;
use polars::export::chrono::{NaiveDateTime, TimeZone};
use smartstring::alias::String as SmartString;
use axum::{routing::get, Json, Router};
use serde_json::Value;
use std::path::Path;
use tower_http::cors::{Any, CorsLayer};
use chrono_tz::{Australia::Melbourne, Tz};


fn filter_df_by_time(df: LazyFrame, time: &str, tz: Tz) -> LazyFrame {
    let naive_time = NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S")
    .expect("Failed to parse datetime");

    // Create a DateTime with the Australia/Melbourne timezone
    let melbourne_time = tz.from_local_datetime(&naive_time).unwrap().timestamp_nanos_opt().unwrap();

    df.filter(
        col("date").gt(lit(
            melbourne_time
        )
    ),
    )
}

fn df_groupby_time_max(df: LazyFrame, interval: &str) -> LazyGroupBy {
    let time_interval = DynamicGroupOptions {
        index_column: SmartString::from("timestamp"),
        every: Duration::parse("1month"),
        period: Duration::parse("1month"),
        offset: Duration::new(0),
        label: Label::Left,
        include_boundaries: true,
        closed_window: ClosedWindow::Left,
        start_by: StartBy::WindowBound,
    };

    df.group_by_dynamic(col("date"), [], time_interval)

}


fn get_lazy_parquet(file: &str) -> Result<LazyFrame, PolarsError> {
    let path = Path::new(file);
    let args = ScanArgsParquet::default();

    let lf = match LazyFrame::scan_parquet(path, args) {
        Ok(lf) => lf,
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
    let path = "/Users/keven/web_projects/litestar-svelte/notebook/fakets.parquet";

    let lf = get_lazy_parquet(path).unwrap();

    let json_value = convert_df_to_json(&lf.collect().unwrap());

    Json(json_value)
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

// #[tokio::main]
// async fn main() {

//     let app = app().await;
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

fn main() {
    // Load fake parquet
    let path = "/Users/keven/web_projects/litestar-svelte/notebook/fakets.parquet";

    let lf = get_lazy_parquet(path).unwrap();
    let filtered_time = filter_df_by_time(lf, "2022-06-01 00:00:00", Melbourne);



    println!("{:?}", filtered_time.collect().unwrap().height());

    // let json_value = convert_df_to_json(&lf);

    // println!("{}", json_value);
}
