//! Gerência de categorias de produtos (painel). Server-only.
use sqlx::PgPool;
use uuid::Uuid;

use super::produtos_admin::slugify;
use crate::domain::Categoria;
use crate::error::AppError;

const MAX_NOME: usize = 60;

fn interno(e: sqlx::Error) -> AppError {
    tracing::error!(error = %e, "erro em categorias");
    AppError::Internal
}

/// Lista todas as categorias (ordenadas) para o painel.
pub async fn listar(pool: &PgPool) -> Result<Vec<Categoria>, sqlx::Error> {
    sqlx::query_as!(
        Categoria,
        r#"SELECT id AS "id!", nome AS "nome!", slug AS "slug!", descricao
           FROM categorias ORDER BY ordem, nome"#
    )
    .fetch_all(pool)
    .await
}

/// Cria uma categoria (gera slug único; fica ativa).
pub async fn criar(pool: &PgPool, nome: &str) -> Result<(), AppError> {
    let nome = nome.trim();
    if nome.is_empty() || nome.chars().count() > MAX_NOME {
        return Err(AppError::Validation);
    }
    let base = slugify(nome);
    let base = if base.is_empty() { "categoria".to_string() } else { base };
    let mut slug = base.clone();
    let mut i = 1;
    while sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM categorias WHERE slug = $1) AS "e!""#,
        slug
    )
    .fetch_one(pool)
    .await
    .map_err(interno)?
    {
        i += 1;
        slug = format!("{base}-{i}");
    }
    sqlx::query!(
        r#"INSERT INTO categorias (nome, slug, ordem, ativo)
           VALUES ($1, $2, COALESCE((SELECT MAX(ordem) + 1 FROM categorias), 0), true)"#,
        nome,
        slug,
    )
    .execute(pool)
    .await
    .map_err(interno)?;
    Ok(())
}

/// Exclui uma categoria (produtos vinculados ficam sem categoria — FK SET NULL).
pub async fn excluir(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM categorias WHERE id = $1", id)
        .execute(pool)
        .await
        .map_err(interno)?;
    Ok(())
}
