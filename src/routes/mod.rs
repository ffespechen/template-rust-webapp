use crate::state::SharedState;
use axum::Router;

mod api_routes; // Archivo src/routes/api_routes.rs
mod web_routes; // Archivo src/routes/web_routes.rs

pub fn crear_router(state: SharedState) -> Router {
    Router::new()
        .merge(api_routes::router(state.clone())) // Rutas para la API
        .merge(web_routes::router(state)) // Rutas para el dashboard HTML
}
