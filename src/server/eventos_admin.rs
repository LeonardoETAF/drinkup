//! CRUD de "eventos" = categorias do carrossel da home. Server-only.
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{EventoCarrossel, EventoForm, EventoLista};
use crate::error::AppError;
use crate::server::produtos_admin::slugify;

/// Categorias ativas do carrossel "Do seu jeito" (home pública).
pub async fn listar_publicos(pool: &PgPool) -> Result<Vec<EventoCarrossel>, sqlx::Error> {
    sqlx::query_as!(
        EventoCarrossel,
        r#"SELECT titulo AS "titulo!", slug AS "slug!", cor, imagem_url
           FROM eventos WHERE ativo = true ORDER BY ordem, titulo"#
    )
    .fetch_all(pool)
    .await
}

/// Lista os eventos ordenados, paginado.
pub async fn listar(
    pool: &PgPool,
    pagina: i64,
    por_pagina: i64,
) -> Result<crate::domain::Pagina<EventoLista>, sqlx::Error> {
    let offset = pagina.max(1).saturating_sub(1) * por_pagina;
    let itens = sqlx::query_as!(
        EventoLista,
        r#"SELECT id AS "id!", titulo AS "titulo!", cor, ordem AS "ordem!", ativo AS "ativo!"
           FROM eventos ORDER BY ordem, titulo
           LIMIT $1 OFFSET $2"#,
        por_pagina,
        offset,
    )
    .fetch_all(pool)
    .await?;

    let total = sqlx::query_scalar!(r#"SELECT count(*) AS "c!" FROM eventos"#)
        .fetch_one(pool)
        .await?;

    Ok(crate::domain::Pagina { itens, total })
}

/// Carrega uma categoria para edição.
pub async fn obter_form(pool: &PgPool, id: Uuid) -> Result<Option<EventoForm>, sqlx::Error> {
    let row = sqlx::query!(
        r#"SELECT id, titulo, cor, imagem_url, ordem, ativo FROM eventos WHERE id = $1"#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| EventoForm {
        id: Some(r.id),
        titulo: r.titulo,
        cor: r.cor,
        imagem_url: r.imagem_url,
        ordem: r.ordem,
        ativo: r.ativo,
    }))
}

/// Cria ou atualiza uma categoria.
pub async fn salvar(pool: &PgPool, form: &EventoForm) -> Result<Uuid, AppError> {
    let titulo = form.titulo.trim();
    if titulo.is_empty() || titulo.chars().count() > 120 {
        return Err(AppError::Validation);
    }

    let id = match form.id {
        Some(id) => {
            sqlx::query!(
                r#"UPDATE eventos SET titulo = $2, cor = $3, imagem_url = $4,
                    ordem = $5, ativo = $6 WHERE id = $1"#,
                id,
                titulo,
                form.cor.as_deref(),
                form.imagem_url.as_deref(),
                form.ordem,
                form.ativo,
            )
            .execute(pool)
            .await
            .map_err(interno)?;
            id
        }
        None => {
            let slug = slug_unico(pool, &slugify(titulo)).await?;
            sqlx::query_scalar!(
                r#"INSERT INTO eventos (titulo, slug, cor, imagem_url, ordem, ativo)
                   VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"#,
                titulo,
                slug,
                form.cor.as_deref(),
                form.imagem_url.as_deref(),
                form.ordem,
                form.ativo,
            )
            .fetch_one(pool)
            .await
            .map_err(interno)?
        }
    };
    Ok(id)
}

/// Exclui uma categoria.
pub async fn excluir(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM eventos WHERE id = $1", id)
        .execute(pool)
        .await
        .map_err(interno)?;
    Ok(())
}

/// Inverte a visibilidade (campo `ativo`) de uma categoria.
pub async fn alternar_ativo(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query!("UPDATE eventos SET ativo = NOT ativo WHERE id = $1", id)
        .execute(pool)
        .await
        .map_err(interno)?;
    Ok(())
}

async fn slug_unico(pool: &PgPool, base: &str) -> Result<String, AppError> {
    let mut slug = base.to_string();
    let mut i = 1;
    while sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM eventos WHERE slug = $1) AS "e!""#,
        slug
    )
    .fetch_one(pool)
    .await
    .map_err(interno)?
    {
        slug = format!("{base}-{i}");
        i += 1;
        if i > 100 {
            return Err(AppError::Internal);
        }
    }
    Ok(slug)
}

fn interno(e: sqlx::Error) -> AppError {
    tracing::error!(error = %e, "erro de banco em eventos_admin");
    AppError::Internal
}
