use leptos::prelude::*;
use uuid::Uuid;

use crate::domain::{ProdutoForm, ProdutoLista};

/// Lista produtos no painel (papel mínimo: editor).
#[server]
pub async fn listar_produtos_admin(busca: String) -> Result<Vec<ProdutoLista>, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    let busca = (!busca.trim().is_empty()).then(|| busca.trim().to_string());
    crate::server::produtos_admin::listar_admin(&pool, busca.as_deref())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao listar produtos (admin)");
            ServerFnError::new("Não foi possível carregar os produtos.")
        })
}

/// Carrega um produto para edição.
#[server]
pub async fn obter_produto_admin(id: Uuid) -> Result<Option<ProdutoForm>, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    crate::server::produtos_admin::obter_form(&pool, id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao obter produto (admin)");
            ServerFnError::new("Não foi possível carregar o produto.")
        })
}

/// Cria ou atualiza um produto. Retorna o id salvo.
#[server]
pub async fn salvar_produto(form: ProdutoForm) -> Result<Uuid, ServerFnError> {
    use crate::error::AppError;

    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    match crate::server::produtos_admin::salvar(&pool, &form).await {
        Ok(id) => Ok(id),
        Err(AppError::Validation) => Err(ServerFnError::new("Informe ao menos um nome válido.")),
        Err(_) => Err(ServerFnError::new("Não foi possível salvar o produto.")),
    }
}

/// Exclui um produto.
#[server]
pub async fn excluir_produto(id: Uuid) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_papel(crate::server::rbac::Papel::Editor).await?;
    crate::server::produtos_admin::excluir(&pool, id)
        .await
        .map_err(|_| ServerFnError::new("Não foi possível excluir o produto."))
}
