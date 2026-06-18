use leptos::prelude::*;
use uuid::Uuid;

use crate::domain::{EventoForm, EventoLista};

/// Lista as categorias do carrossel (papel mínimo: editor).
#[server]
pub async fn listar_eventos_admin() -> Result<Vec<EventoLista>, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    crate::server::eventos_admin::listar(&pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao listar eventos");
            ServerFnError::new("Não foi possível carregar as categorias.")
        })
}

/// Carrega uma categoria para edição.
#[server]
pub async fn obter_evento_admin(id: Uuid) -> Result<Option<EventoForm>, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    crate::server::eventos_admin::obter_form(&pool, id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao obter evento");
            ServerFnError::new("Não foi possível carregar a categoria.")
        })
}

/// Cria ou atualiza uma categoria.
#[server]
pub async fn salvar_evento(form: EventoForm) -> Result<Uuid, ServerFnError> {
    use crate::error::AppError;

    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    match crate::server::eventos_admin::salvar(&pool, &form).await {
        Ok(id) => Ok(id),
        Err(AppError::Validation) => Err(ServerFnError::new("Informe um nome válido.")),
        Err(_) => Err(ServerFnError::new("Não foi possível salvar a categoria.")),
    }
}

/// Exclui uma categoria.
#[server]
pub async fn excluir_evento(id: Uuid) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    crate::server::eventos_admin::excluir(&pool, id)
        .await
        .map_err(|_| ServerFnError::new("Não foi possível excluir a categoria."))
}
