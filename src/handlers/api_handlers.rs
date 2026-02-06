// -- Lógica de los endpoints de la API REST
use crate::db;
use crate::models;
use crate::state::SharedState;
use axum::{extract::State, Json, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct EstadoResponse {
    estado: &'static str,
}

pub async fn index(
    State(_state): State<SharedState>,
) -> Json<EstadoResponse> {
    Json(EstadoResponse { estado: "OK" })
}