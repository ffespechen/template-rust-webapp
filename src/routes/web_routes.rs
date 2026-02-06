// Este módulo define las rutas para servir páginas HTML, como el dashboard.
use crate::handlers;
use crate::state::SharedState;
use axum::{routing::get, Router};

pub fn router(state: SharedState) -> Router {
    Router::new()
        .route("/", get(handlers::web_handlers::index)) // Ruta raíz para el dashboard
        .with_state(state) // Pasar el estado compartido a las rutas
}
