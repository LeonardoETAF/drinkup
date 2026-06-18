use leptos::prelude::*;
use uuid::Uuid;

use crate::domain::{UsuarioForm, UsuarioLista};

/// Lista usuários do painel. Somente `admin`.
#[server]
pub async fn listar_usuarios() -> Result<Vec<UsuarioLista>, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Admin).await?;
    crate::server::usuarios_admin::listar(&pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao listar usuários");
            ServerFnError::new("Não foi possível carregar os usuários.")
        })
}

/// Carrega um usuário para edição. Somente `admin`.
#[server]
pub async fn obter_usuario_admin(id: Uuid) -> Result<Option<UsuarioForm>, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Admin).await?;
    crate::server::usuarios_admin::obter_form(&pool, id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao obter usuário");
            ServerFnError::new("Não foi possível carregar o usuário.")
        })
}

/// Cria ou atualiza um usuário. Somente `admin`.
#[server]
pub async fn salvar_usuario(form: UsuarioForm) -> Result<Uuid, ServerFnError> {
    use crate::error::AppError;

    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Admin).await?;
    match crate::server::usuarios_admin::salvar(&pool, &form).await {
        Ok(id) => Ok(id),
        Err(AppError::Validation) => Err(ServerFnError::new(
            "Dados inválidos (verifique e-mail, papel e senha).",
        )),
        Err(_) => Err(ServerFnError::new("Não foi possível salvar o usuário.")),
    }
}

/// Exclui um usuário (não permite excluir a si mesmo). Somente `admin`.
#[server]
pub async fn excluir_usuario(id: Uuid) -> Result<(), ServerFnError> {
    use crate::error::AppError;

    let pool = expect_context::<sqlx::PgPool>();
    let atual = crate::api::auth::exigir_papel(crate::server::rbac::Papel::Admin).await?;
    match crate::server::usuarios_admin::excluir(&pool, id, atual.id).await {
        Ok(()) => Ok(()),
        Err(AppError::Validation) => Err(ServerFnError::new("Você não pode excluir a si mesmo.")),
        Err(_) => Err(ServerFnError::new("Não foi possível excluir o usuário.")),
    }
}
