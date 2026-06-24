use leptos::prelude::*;
use uuid::Uuid;

use crate::domain::Categoria;

/// Lista as categorias para o painel (papel mínimo: editor).
#[server]
pub async fn listar_categorias_admin() -> Result<Vec<Categoria>, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_acesso(crate::server::rbac::Papel::Visualizador, "produtos").await?;
    crate::server::categorias_admin::listar(&pool)
        .await
        .map_err(|_| ServerFnError::new("Não foi possível carregar as categorias."))
}

/// Cria uma categoria (ou subcategoria, se `parent_id` for informado). Papel
/// mínimo: editor.
#[server]
pub async fn criar_categoria(nome: String, parent_id: Option<Uuid>) -> Result<(), ServerFnError> {
    use crate::error::AppError;

    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_acesso(crate::server::rbac::Papel::Editor, "produtos").await?;
    match crate::server::categorias_admin::criar(&pool, &nome, parent_id).await {
        Ok(()) => Ok(()),
        Err(AppError::Validation) => Err(ServerFnError::new("Informe um nome válido.")),
        Err(_) => Err(ServerFnError::new("Não foi possível criar a categoria.")),
    }
}

/// Renomeia uma categoria ou subcategoria (papel mínimo: editor).
#[server]
pub async fn renomear_categoria(id: Uuid, nome: String) -> Result<(), ServerFnError> {
    use crate::error::AppError;

    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_acesso(crate::server::rbac::Papel::Editor, "produtos").await?;
    match crate::server::categorias_admin::renomear(&pool, id, &nome).await {
        Ok(()) => Ok(()),
        Err(AppError::Validation) => Err(ServerFnError::new("Informe um nome válido.")),
        Err(_) => Err(ServerFnError::new("Não foi possível renomear.")),
    }
}

/// Exclui uma categoria (papel mínimo: editor).
#[server]
pub async fn excluir_categoria(id: Uuid) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::api::auth::exigir_acesso(crate::server::rbac::Papel::Editor, "produtos").await?;
    crate::server::categorias_admin::excluir(&pool, id)
        .await
        .map_err(|_| ServerFnError::new("Não foi possível excluir a categoria."))
}
