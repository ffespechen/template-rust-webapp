// Para compartir el estado de la aplicación (como la conexión a la base de datos)
// Atomic Reference Counting
use std::sync::Arc;

pub struct AppState {
    {% if database == "sqlite" %}
    pub db: sqlx::SqlitePool,
    {% else if database == "postgres" %}
    pub db: sqlx::PgPool,
    {% else if database == "mongodb" %}
    pub db: mongodb::Database,
    {% endif %}
}

// Definimos un alias para que sea más fácil de escribir en los handlers
pub type SharedState = Arc<AppState>;
