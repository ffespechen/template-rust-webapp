// -- Lógica para manejar las rutas web, como mostrar el dashboard o el index --
use crate::models;
use crate::state::SharedState;
use askama::Template;
use axum::{extract::State, response::Html};
use futures_util::TryStreamExt; // Para manejar el cursor de mongodb


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    valor: i32,
}

pub async fn index(State(state): State<SharedState>) -> impl axum::response::IntoResponse {
    Html(IndexTemplate { valor: 42 }.render().unwrap())
}
