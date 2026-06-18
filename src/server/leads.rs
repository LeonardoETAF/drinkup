//! Repositório de leads (orçamentos) para o painel. Server-only.
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{FiltroLeads, LeadResumo, PaginaLeads};
use crate::error::AppError;

const STATUS_VALIDOS: [&str; 4] = ["novo", "em_atendimento", "convertido", "perdido"];

/// Lista leads com busca (nome/contato) e filtro de status. Limite de 100.
pub async fn listar(pool: &PgPool, filtro: &FiltroLeads) -> Result<PaginaLeads, sqlx::Error> {
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
        LIMIT 100
        "#,
        filtro.busca.as_deref(),
        filtro.status.as_deref(),
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
