use leptos::prelude::*;

use crate::domain::ParceiroPublico;

/// Lista parceiros ativos para a vitrine pública (sem autenticação).
#[server]
pub async fn listar_parceiros() -> Result<Vec<ParceiroPublico>, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::server::parceiros_admin::listar_publicos(&pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao listar parceiros públicos");
            ServerFnError::new("Não foi possível carregar os parceiros.")
        })
}
