mod models;
mod handlers;
mod routes;
mod state;
mod db;

use state::AppState;
use dotenvy::dotenv;
use std::sync::Arc;


#[tokio::main]
async fn main() {
    // Cargar archivo .env
    dotenv().ok();

    let database = db::conectar().await;

    let compartido = Arc::new(AppState {
        db: database,
    });

    let app = routes::crear_router(compartido);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.expect("No se pudo bindear al puerto");
    println!("Servidor escuchando en http://{}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
}
