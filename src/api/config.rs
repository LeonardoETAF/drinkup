use leptos::prelude::*;

use crate::domain::Configuracoes;

/// Lê as configurações da loja para o painel (papel mínimo: gerente).
#[server]
pub async fn obter_config() -> Result<Configuracoes, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Gerente).await?;
    crate::server::config::obter(&pool).await.map_err(|e| {
        tracing::error!(error = %e, "falha ao ler configurações");
        ServerFnError::new("Não foi possível carregar as configurações.")
    })
}

/// Grava as configurações da loja (papel mínimo: gerente).
#[server]
pub async fn salvar_config(cfg: Configuracoes) -> Result<(), ServerFnError> {
    use crate::error::AppError;

    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Gerente).await?;
    match crate::server::config::salvar(&pool, &cfg).await {
        Ok(()) => Ok(()),
        Err(AppError::Validation) => Err(ServerFnError::new("Algum campo é muito longo.")),
        Err(_) => Err(ServerFnError::new(
            "Não foi possível salvar as configurações.",
        )),
    }
}

/// Dados públicos de contato/loja para o site (sem autenticação).
#[server]
pub async fn obter_contato() -> Result<Configuracoes, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::server::config::obter(&pool)
        .await
        .map_err(|_| ServerFnError::new("Não foi possível carregar o contato."))
}
