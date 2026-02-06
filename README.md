# 🦀 Axum + MongoDB + Askama Template

Este es un template profesional para `cargo-generate` diseñado para construir aplicaciones web robustas, modulares y de alto rendimiento en **Rust**. Es ideal para proyectos de telemetría, IoT, pasarelas de sensores y dashboards de automatización.

## ✨ Características Principales

- 🏗️ **Arquitectura Modular:** Organización clara por capas (Models, Handlers, Routes, Services).
- 🍃 **Integración con Bases de Datos:** Driver oficial configurado para persistencia NoSQL (MongoDB) o SQL (SQLite, Postgres) flexible.
- 🎨 **Dashboard Dinámico:** Renderizado HTML en tiempo de compilación con Askama y estilos con Bootstrap 5.
- 🤖 **API REST Ready:** Endpoints preparados para operaciones CRUDL de sensores o dispositivos.
- 🔌 **Configuración Segura:** Manejo de credenciales mediante variables de entorno (`.env`).
- 🐳 **Entorno Dockerizado:** Docker Compose incluido para una base de datos persistente e instantánea.
- 🚀 **Alto Rendimiento:** Basado en el ecosistema de Tokio para manejar miles de conexiones concurrentes.

## 🛠️ Stack Tecnológico

- **Framework:** [Axum](https://github.com/tokio-rs/axum)
- **Runtime:** [Tokio](https://tokio.rs/)
- **Bases de Datos:** [MongoDB](https://www.mongodb.com/) [SQLite](https://sqlite.org/) [PostgreSQL](https://www.postgresql.org/)
- **Plantillas:** [Askama](https://github.com/djc/askama)
- **Serialización:** [Serde](https://serde.rs/)
- **Middleware:** [Tower-HTTP](https://github.com/tower-rs/tower-http)

## 📋 Requisitos Previos

Asegúrate de tener instalado:

- [Rust](https://www.rust-lang.org/) (Edición 2021+)
- [cargo-generate](https://github.com/cargo-generate/cargo-generate) (`cargo install cargo-generate`)
- [Docker](https://www.docker.com/) y Docker Compose

## 🚀 Uso Rápido

1. **Generar el proyecto:**

   ```bash
   cargo generate --git [https://github.com/ffespechen/template-rust-webapp](https://github.com/ffespechen/template-rust-webapp) --name mi-proyecto-sensor
   ```

2. **Configurar el entorno:**

   ```bash
   cp .env.example .env
   ```

3. **Modificar el docker-compose.yml y levantar la aplicación:**

   ```bash
   docker-compose up -d
   ```

4. **Ejecutar la aplicación:**
   ```bash
   cargo run
   ```

## 📂 Estructura del Proyecto

```bash
.
├── src/
│ ├── models/ # Estructuras de datos y validaciones
│ ├── handlers/ # Lógica de negocio (API y Vistas HTML)
│ ├── routes/ # Definición y prefijos de endpoints
│ ├── db.rs # Conexión y gestión de Bases de Datos
│ ├── state.rs # Estado compartido de la aplicación (Arc)
│ └── main.rs # Punto de entrada y configuración
├── templates/ # Archivos de plantilla Askama (.html)
├── uploads/ # Almacenamiento local de archivos/imágenes
├── .env # Configuración sensible (No incluir en git)
└── docker-compose.yml
```

## 🤖 Desarrollado usando el ecosistema de Rust 🦀.
