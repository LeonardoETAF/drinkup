use leptos::prelude::*;
use uuid::Uuid;

use crate::domain::{DashboardResumo, FiltroLeads, PaginaLeads};

/// Resumo do dashboard. Exige usuário autenticado (papel mínimo: editor).
#[server]
pub async fn resumo_dashboard() -> Result<DashboardResumo, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    crate::server::dashboard::resumo(&pool).await.map_err(|e| {
        tracing::error!(error = %e, "falha no resumo do dashboard");
        ServerFnError::new("Não foi possível carregar o dashboard.")
    })
}

/// Lista leads (busca + filtro de status). Exige autenticação (papel: editor).
#[server]
pub async fn listar_leads(filtro: FiltroLeads) -> Result<PaginaLeads, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    crate::server::leads::listar(&pool, &filtro)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao listar leads");
            ServerFnError::new("Não foi possível carregar os leads.")
        })
}

/// Atualiza o status de um lead. Exige autenticação (papel: editor).
#[server]
pub async fn atualizar_status_lead(id: Uuid, status: String) -> Result<(), ServerFnError> {
    use crate::error::AppError;

    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    match crate::server::leads::atualizar_status(&pool, id, &status).await {
        Ok(()) => Ok(()),
        Err(AppError::Validation) => Err(ServerFnError::new("Status inválido.")),
        Err(_) => Err(ServerFnError::new("Não foi possível atualizar o lead.")),
    }
}
