use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use tokio_postgres::NoTls;
use serde::Serialize;
use std::env;
use dotenv::dotenv;

#[derive(Serialize)]
struct GeoFeature {
    id: i32,
    geom: String,
}

async fn get_features() -> impl Responder {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // PostGIS에 연결
    let (client, connection) =
        tokio_postgres::connect(&db_url, NoTls).await.expect("Failed to connect to DB");

    // DB 연결 유지
    actix_web::rt::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // WFS/WMS 요청 처리 (간단한 WFS 기능)
    let rows = client
        .query("SELECT id, ST_AsGeoJSON(geom) FROM your_table LIMIT 10", &[])
        .await
        .expect("Failed to execute query");

    let mut features = Vec::new();
    for row in rows {
        let id: i32 = row.get(0);
        let geom: String = row.get(1);
        features.push(GeoFeature { id, geom });
    }

    HttpResponse::Ok().json(features)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/wfs", web::get().to(get_features)) // WFS endpoint
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}