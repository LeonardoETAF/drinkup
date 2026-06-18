//! CRUD de parceiros para o painel. Server-only.
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{ParceiroForm, ParceiroLista, ParceiroPublico};
use crate::error::AppError;
use crate::server::produtos_admin::slugify;

/// Lista parceiros ativos para a página pública (ordenados).
pub async fn listar_publicos(pool: &PgPool) -> Result<Vec<ParceiroPublico>, sqlx::Error> {
    sqlx::query_as!(
        ParceiroPublico,
        r#"SELECT nome AS "nome!", logo_url, site_url, descricao
           FROM parceiros WHERE ativo = true ORDER BY ordem, nome"#
    )
    .fetch_all(pool)
    .await
}

/// Lista parceiros (ordenados) para a grade do painel.
pub async fn listar(pool: &PgPool, busca: Option<&str>) -> Result<Vec<ParceiroLista>, sqlx::Error> {
    sqlx::query_as!(
        ParceiroLista,
        r#"
        SELECT id AS "id!", nome AS "nome!", logo_url, ativo AS "ativo!"
        FROM parceiros
        WHERE ($1::text IS NULL OR nome ILIKE '%' || $1 || '%')
        ORDER BY ordem, nome
        "#,
        busca
    )
    .fetch_all(pool)
    .await
}

/// Carrega um parceiro para edição.
pub async fn obter_form(pool: &PgPool, id: Uuid) -> Result<Option<ParceiroForm>, sqlx::Error> {
    let row = sqlx::query!(
        r#"SELECT id, nome, logo_url, site_url, descricao, ordem, ativo
           FROM parceiros WHERE id = $1"#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| ParceiroForm {
        id: Some(r.id),
        nome: r.nome,
        logo_url: r.logo_url,
        site_url: r.site_url,
        descricao: r.descricao,
        ordem: r.ordem,
        ativo: r.ativo,
    }))
}

/// Cria ou atualiza um parceiro.
pub async fn salvar(pool: &PgPool, form: &ParceiroForm) -> Result<Uuid, AppError> {
    let nome = form.nome.trim();
    if nome.is_empty() || nome.chars().count() > 160 {
        return Err(AppError::Validation);
    }

    let id = match form.id {
        Some(id) => {
            sqlx::query!(
                r#"UPDATE parceiros SET nome = $2, logo_url = $3, site_url = $4,
                    descricao = $5, ordem = $6, ativo = $7 WHERE id = $1"#,
                id,
                nome,
                form.logo_url.as_deref(),
                form.site_url.as_deref(),
                form.descricao.as_deref(),
                form.ordem,
                form.ativo,
            )
            .execute(pool)
            .await
            .map_err(interno)?;
            id
        }
        None => {
            let slug = slug_unico(pool, &slugify(nome)).await?;
            sqlx::query_scalar!(
                r#"INSERT INTO parceiros (nome, slug, logo_url, site_url, descricao, ordem, ativo)
                   VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id"#,
                nome,
                slug,
                form.logo_url.as_deref(),
                form.site_url.as_deref(),
                form.descricao.as_deref(),
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

/// Exclui um parceiro.
pub async fn excluir(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM parceiros WHERE id = $1", id)
        .execute(pool)
        .await
        .map_err(interno)?;
    Ok(())
}

async fn slug_unico(pool: &PgPool, base: &str) -> Result<String, AppError> {
    let mut slug = base.to_string();
    let mut i = 1;
    while sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM parceiros WHERE slug = $1) AS "e!""#,
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
    tracing::error!(error = %e, "erro de banco em parceiros_admin");
    AppError::Internal
}
