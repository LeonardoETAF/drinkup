use leptos::prelude::*;

use crate::domain::{Categoria, FiltroProdutos, PaginaProdutos, ProdutoDetalhe};

/// Lista produtos do catálogo (filtros + paginação). Resolvido no servidor.
#[server]
pub async fn listar_produtos(filtro: FiltroProdutos) -> Result<PaginaProdutos, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::server::products::listar(&pool, &filtro)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao listar produtos");
            ServerFnError::new("Não foi possível carregar os produtos.")
        })
}

/// Registra a visualização de um produto (chamada no cliente ao abrir a tela
/// de detalhe, contando também a navegação SPA). Best-effort.
#[server]
pub async fn registrar_visita_produto(slug: String) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::server::visitas::registrar_produto(&pool, &slug).await;
    Ok(())
}

/// Detalhe de um produto pelo slug.
#[server]
pub async fn obter_produto(slug: String) -> Result<Option<ProdutoDetalhe>, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::server::products::por_slug(&pool, &slug)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao obter produto");
            ServerFnError::new("Não foi possível carregar o produto.")
        })
}

/// Lista categorias ativas (para filtros/menus).
#[server]
pub async fn listar_categorias() -> Result<Vec<Categoria>, ServerFnError> {
    let pool = expect_context::<sqlx::PgPool>();
    crate::server::products::listar_categorias(&pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao listar categorias");
            ServerFnError::new("Não foi possível carregar as categorias.")
        })
}
