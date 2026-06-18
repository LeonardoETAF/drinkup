//! Resumo do dashboard administrativo. Server-only.
use sqlx::PgPool;

use crate::domain::{DashboardResumo, LeadResumo};

/// Agrega contagens e os leads mais recentes para o dashboard.
pub async fn resumo(pool: &PgPool) -> Result<DashboardResumo, sqlx::Error> {
    let total_produtos = sqlx::query_scalar!(r#"SELECT count(*) AS "c!" FROM produtos"#)
        .fetch_one(pool)
        .await?;
    let total_leads = sqlx::query_scalar!(r#"SELECT count(*) AS "c!" FROM leads"#)
        .fetch_one(pool)
        .await?;
    let leads_novos =
        sqlx::query_scalar!(r#"SELECT count(*) AS "c!" FROM leads WHERE status = 'novo'"#)
            .fetch_one(pool)
            .await?;
    let total_eventos = sqlx::query_scalar!(r#"SELECT count(*) AS "c!" FROM eventos"#)
        .fetch_one(pool)
        .await?;

    let recentes = sqlx::query_as!(
        LeadResumo,
        r#"
        SELECT id AS "id!", nome AS "nome!", contato AS "contato!",
               origem AS "origem!", status AS "status!",
               to_char(created_at, 'DD/MM/YYYY') AS "inscricao!"
        FROM leads
        ORDER BY created_at DESC
        LIMIT 6
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(DashboardResumo {
        total_produtos,
        total_leads,
        leads_novos,
        total_eventos,
        recentes,
    })
}
