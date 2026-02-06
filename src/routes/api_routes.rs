// Rutas para la API REST
use crate::handlers;
use crate::state::SharedState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn router(state: SharedState) -> Router {
    Router::new()
        .route("/api", get(handlers::api_handlers::index)) // Ruta para registrar lecturas
        .with_state(state) // Pasar el estado compartido a las rutas
}
