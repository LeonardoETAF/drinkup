use leptos::prelude::*;

use crate::domain::EventoCarrossel;

/// Lista as categorias do carrossel "Do seu jeito" da home (sem autenticação).
#[server]
pub async fn listar_eventos() -> Result<Vec<EventoCarrossel>, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::server::eventos_admin::listar_publicos(&pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao listar eventos públicos");
            ServerFnError::new("Não foi possível carregar as categorias.")
        })
}
