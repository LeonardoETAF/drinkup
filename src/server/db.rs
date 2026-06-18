//! Conexão e pool do PostgreSQL (SQLx). O pool é disponibilizado às server
//! functions via contexto do Leptos (ver `main.rs`).
use std::time::Duration;

use sqlx::postgres::{PgPool, PgPoolOptions};

/// Cria o pool de conexões a partir da URL do banco (`DATABASE_URL`).
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
}

/// Aplica as migrations pendentes (embutidas no binário em tempo de compilação).
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

/// Verifica a conectividade do banco (`SELECT 1`).
pub async fn ping(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await.map(|_| ())
}
