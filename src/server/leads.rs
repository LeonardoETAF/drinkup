//! Repositório de leads (orçamentos) para o painel. Server-only.
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{FiltroLeads, LeadResumo, PaginaLeads};
use crate::error::AppError;

const STATUS_VALIDOS: [&str; 4] = ["novo", "em_atendimento", "convertido", "perdido"];

/// Lista leads com busca (nome/contato) e filtro de status, paginado.
pub async fn listar(
    pool: &PgPool,
    filtro: &FiltroLeads,
    pagina: i64,
    por_pagina: i64,
) -> Result<PaginaLeads, sqlx::Error> {
    let offset = pagina.max(1).saturating_sub(1) * por_pagina;
    let itens = sqlx::query_as!(
        LeadResumo,
        r#"
        SELECT id AS "id!", nome AS "nome!", contato AS "contato!",
               origem AS "origem!", status AS "status!",
               to_char(created_at, 'DD/MM/YYYY') AS "inscricao!"
        FROM leads
        WHERE ($1::text IS NULL OR nome ILIKE '%' || $1 || '%' OR contato ILIKE '%' || $1 || '%')
          AND ($2::text IS NULL OR status = $2)
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        "#,
        filtro.busca.as_deref(),
        filtro.status.as_deref(),
        por_pagina,
        offset,
    )
    .fetch_all(pool)
    .await?;

    let total = sqlx::query_scalar!(
        r#"
        SELECT count(*) AS "c!"
        FROM leads
        WHERE ($1::text IS NULL OR nome ILIKE '%' || $1 || '%' OR contato ILIKE '%' || $1 || '%')
          AND ($2::text IS NULL OR status = $2)
        "#,
        filtro.busca.as_deref(),
        filtro.status.as_deref(),
    )
    .fetch_one(pool)
    .await?;

    Ok(PaginaLeads { itens, total })
}

/// Atualiza o status de um lead (valida o valor no servidor).
pub async fn atualizar_status(pool: &PgPool, id: Uuid, status: &str) -> Result<(), AppError> {
    if !STATUS_VALIDOS.contains(&status) {
        return Err(AppError::Validation);
    }
    sqlx::query!("UPDATE leads SET status = $2 WHERE id = $1", id, status)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "falha ao atualizar status do lead");
            AppError::Internal
        })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Integração: a query paginada executa e respeita o LIMIT (OFFSET aplicado).
    #[tokio::test]
    async fn listar_paginado_respeita_limite() {
        let Ok(url) = std::env::var("DATABASE_URL") else {
            return;
        };
        let Ok(pool) = crate::server::db::create_pool(&url).await else {
            return;
        };
        let filtro = FiltroLeads {
            busca: None,
            status: None,
        };
        let p = listar(&pool, &filtro, 1, 5).await.expect("página 1");
        assert!(p.itens.len() as i64 <= 5, "respeita o LIMIT");
        assert!(p.total >= p.itens.len() as i64, "total é o geral");
    }
}
