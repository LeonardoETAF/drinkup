use leptos::prelude::*;
use uuid::Uuid;

use crate::domain::{ParceiroForm, ParceiroLista};

/// Lista parceiros no painel (papel mínimo: editor).
#[server]
pub async fn listar_parceiros_admin(busca: String) -> Result<Vec<ParceiroLista>, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    let busca = (!busca.trim().is_empty()).then(|| busca.trim().to_string());
    crate::server::parceiros_admin::listar(&pool, busca.as_deref())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao listar parceiros");
            ServerFnError::new("Não foi possível carregar os parceiros.")
        })
}

/// Carrega um parceiro para edição.
#[server]
pub async fn obter_parceiro_admin(id: Uuid) -> Result<Option<ParceiroForm>, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    crate::server::parceiros_admin::obter_form(&pool, id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao obter parceiro");
            ServerFnError::new("Não foi possível carregar o parceiro.")
        })
}

/// Cria ou atualiza um parceiro.
#[server]
pub async fn salvar_parceiro(form: ParceiroForm) -> Result<Uuid, ServerFnError> {
    use crate::error::AppError;

    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    match crate::server::parceiros_admin::salvar(&pool, &form).await {
        Ok(id) => Ok(id),
        Err(AppError::Validation) => Err(ServerFnError::new("Informe um nome válido.")),
        Err(_) => Err(ServerFnError::new("Não foi possível salvar o parceiro.")),
    }
}

/// Exclui um parceiro.
#[server]
pub async fn excluir_parceiro(id: Uuid) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    crate::server::parceiros_admin::excluir(&pool, id)
        .await
        .map_err(|_| ServerFnError::new("Não foi possível excluir o parceiro."))
}
