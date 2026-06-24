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

/// Lista todas as categorias e subcategorias (ordenadas) para o painel.
pub async fn listar(pool: &PgPool) -> Result<Vec<Categoria>, sqlx::Error> {
    sqlx::query_as!(
        Categoria,
        r#"SELECT id AS "id!", nome AS "nome!", slug AS "slug!", descricao, parent_id
           FROM categorias ORDER BY ordem, nome"#
    )
    .fetch_all(pool)
    .await
}

/// Cria uma categoria ou subcategoria (gera slug único; fica ativa). Com
/// `parent_id`, o pai precisa existir e ser de nível superior (apenas 2 níveis).
pub async fn criar(pool: &PgPool, nome: &str, parent_id: Option<Uuid>) -> Result<(), AppError> {
    let nome = nome.trim();
    if nome.is_empty() || nome.chars().count() > MAX_NOME {
        return Err(AppError::Validation);
    }
    if let Some(pid) = parent_id {
        let pai_valido = sqlx::query_scalar!(
            r#"SELECT EXISTS(
                 SELECT 1 FROM categorias WHERE id = $1 AND parent_id IS NULL
               ) AS "e!""#,
            pid
        )
        .fetch_one(pool)
        .await
        .map_err(interno)?;
        if !pai_valido {
            return Err(AppError::Validation);
        }
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
        r#"INSERT INTO categorias (nome, slug, ordem, ativo, parent_id)
           VALUES ($1, $2, COALESCE((SELECT MAX(ordem) + 1 FROM categorias), 0), true, $3)"#,
        nome,
        slug,
        parent_id,
    )
    .execute(pool)
    .await
    .map_err(interno)?;
    Ok(())
}

/// Renomeia uma categoria ou subcategoria. O `slug` é mantido de propósito,
/// para não quebrar links/filtros já em uso (`?categoria=...`); só o nome
/// exibido muda.
pub async fn renomear(pool: &PgPool, id: Uuid, nome: &str) -> Result<(), AppError> {
    let nome = nome.trim();
    if nome.is_empty() || nome.chars().count() > MAX_NOME {
        return Err(AppError::Validation);
    }
    sqlx::query!("UPDATE categorias SET nome = $2 WHERE id = $1", id, nome)
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
