use leptos::prelude::*;
use uuid::Uuid;

use crate::domain::{DashboardResumo, PaginaLeads};

/// Resumo do dashboard para o período (ano/mês/dia). `ano = None` → mês atual.
/// Exige usuário autenticado (papel mínimo: visualizador).
#[server]
pub async fn resumo_dashboard(
    ano: Option<i32>,
    mes: Option<i32>,
    dia: Option<i32>,
) -> Result<DashboardResumo, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_acesso(crate::server::rbac::Papel::Visualizador, "dashboard").await?;
    crate::server::dashboard::resumo(&pool, ano, mes, dia)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha no resumo do dashboard");
            ServerFnError::new("Não foi possível carregar o dashboard.")
        })
}

/// Lista leads (busca + filtro de status). Exige autenticação (papel: editor).
///
/// Recebe `busca`/`status` como argumentos separados (e não um `FiltroLeads`):
/// quando ambos são `None`, o corpo da requisição fica vazio e o framework não
/// consegue desserializar um campo struct obrigatório (`filtro`). Argumentos
/// `Option` no topo, ausentes, viram `None` sem erro.
#[server]
pub async fn listar_leads(
    busca: Option<String>,
    status: Option<String>,
    pagina: u32,
) -> Result<PaginaLeads, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_acesso(crate::server::rbac::Papel::Visualizador, "leads").await?;
    let filtro = crate::domain::FiltroLeads { busca, status };
    crate::server::leads::listar(
        &pool,
        &filtro,
        i64::from(pagina.max(1)),
        crate::domain::ADMIN_TABELA_POR_PAGINA,
    )
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
    crate::api::auth::exigir_acesso(crate::server::rbac::Papel::Editor, "leads").await?;
    match crate::server::leads::atualizar_status(&pool, id, &status).await {
        Ok(()) => Ok(()),
        Err(AppError::Validation) => Err(ServerFnError::new("Status inválido.")),
        Err(_) => Err(ServerFnError::new("Não foi possível atualizar o lead.")),
    }
}
