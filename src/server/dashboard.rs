//! Resumo do dashboard administrativo (tudo em tempo real, do banco). Server-only.
use sqlx::PgPool;

use crate::domain::{DashboardResumo, DiaAcesso, ItemRanking, LeadResumo, OrigemFatia};

/// Variação percentual de `cur` sobre `prev` (None quando não há base anterior).
fn delta_pct(cur: f64, prev: f64) -> Option<i32> {
    if prev <= 0.0 {
        return None;
    }
    Some((((cur - prev) / prev) * 100.0).round() as i32)
}

/// Rótulo amigável para um caminho de página pública.
fn rotulo_pagina(caminho: &str) -> String {
    match caminho {
        "/" => "Início".to_string(),
        "/produtos" => "Produtos".to_string(),
        "/quem-somos" => "Quem Somos".to_string(),
        "/parceiros" => "Parceiros".to_string(),
        "/contato" => "Contato".to_string(),
        outro => outro
            .strip_prefix("/produtos/")
            .map(|s| format!("Produto: {s}"))
            .unwrap_or_else(|| outro.to_string()),
    }
}

/// Agrega contadores, gráficos e os leads mais recentes para o dashboard.
pub async fn resumo(pool: &PgPool) -> Result<DashboardResumo, sqlx::Error> {
    // --- Acessos (mês atual x mês anterior) ---
    let acessos_mes = sqlx::query_scalar!(
        r#"SELECT count(*) AS "c!" FROM visitas
           WHERE created_at >= date_trunc('month', now())"#
    )
    .fetch_one(pool)
    .await?;
    let acessos_ant = sqlx::query_scalar!(
        r#"SELECT count(*) AS "c!" FROM visitas
           WHERE created_at >= date_trunc('month', now()) - interval '1 month'
             AND created_at <  date_trunc('month', now())"#
    )
    .fetch_one(pool)
    .await?;

    // --- Leads (total, mês atual x anterior, novos) ---
    let total_leads = sqlx::query_scalar!(r#"SELECT count(*) AS "c!" FROM leads"#)
        .fetch_one(pool)
        .await?;
    let leads_mes = sqlx::query_scalar!(
        r#"SELECT count(*) AS "c!" FROM leads WHERE created_at >= date_trunc('month', now())"#
    )
    .fetch_one(pool)
    .await?;
    let leads_ant = sqlx::query_scalar!(
        r#"SELECT count(*) AS "c!" FROM leads
           WHERE created_at >= date_trunc('month', now()) - interval '1 month'
             AND created_at <  date_trunc('month', now())"#
    )
    .fetch_one(pool)
    .await?;
    let leads_novos =
        sqlx::query_scalar!(r#"SELECT count(*) AS "c!" FROM leads WHERE status = 'novo'"#)
            .fetch_one(pool)
            .await?;

    // --- Produtos e eventos ---
    let produtos_total = sqlx::query_scalar!(r#"SELECT count(*) AS "c!" FROM produtos"#)
        .fetch_one(pool)
        .await?;
    let produtos_ativos =
        sqlx::query_scalar!(r#"SELECT count(*) AS "c!" FROM produtos WHERE ativo"#)
            .fetch_one(pool)
            .await?;
    let total_eventos = sqlx::query_scalar!(r#"SELECT count(*) AS "c!" FROM eventos"#)
        .fetch_one(pool)
        .await?;

    // --- Conversão (leads/acessos no mês) ---
    let conv = |leads: i64, acessos: i64| {
        if acessos > 0 {
            leads as f64 / acessos as f64 * 100.0
        } else {
            0.0
        }
    };
    let taxa_conversao = conv(leads_mes, acessos_mes);
    let conversao_delta = delta_pct(taxa_conversao, conv(leads_ant, acessos_ant));

    // --- Acessos por dia (últimos 7 dias) ---
    let acessos_7dias = sqlx::query_as!(
        DiaAcesso,
        r#"
        SELECT
          CASE extract(isodow FROM d.dia)::int
            WHEN 1 THEN 'Seg' WHEN 2 THEN 'Ter' WHEN 3 THEN 'Qua'
            WHEN 4 THEN 'Qui' WHEN 5 THEN 'Sex' WHEN 6 THEN 'Sáb' ELSE 'Dom'
          END AS "rotulo!",
          count(v.id) AS "total!"
        FROM generate_series(current_date - interval '6 days', current_date, interval '1 day') AS d(dia)
        LEFT JOIN visitas v
          ON v.created_at >= d.dia AND v.created_at < d.dia + interval '1 day'
        GROUP BY d.dia
        ORDER BY d.dia
        "#
    )
    .fetch_all(pool)
    .await?;

    // --- Origem do tráfego (percentuais calculados no servidor) ---
    let origem_rows = sqlx::query!(
        r#"SELECT origem AS "origem!", count(*) AS "total!"
           FROM visitas GROUP BY origem ORDER BY count(*) DESC"#
    )
    .fetch_all(pool)
    .await?;
    let soma_origem: i64 = origem_rows.iter().map(|r| r.total).sum();
    let origem_trafego = origem_rows
        .into_iter()
        .map(|r| {
            let pct = if soma_origem > 0 {
                (r.total as f64 / soma_origem as f64 * 100.0).round() as i32
            } else {
                0
            };
            OrigemFatia {
                origem: r.origem,
                total: r.total,
                pct,
            }
        })
        .collect();

    // --- Páginas mais visitadas ---
    let paginas = sqlx::query!(
        r#"SELECT caminho AS "caminho!", count(*) AS "total!"
           FROM visitas GROUP BY caminho ORDER BY count(*) DESC LIMIT 5"#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| ItemRanking {
        rotulo: rotulo_pagina(&r.caminho),
        total: r.total,
    })
    .collect();

    // --- Produtos mais vistos (visitas na página do produto) ---
    let produtos_vistos = sqlx::query_as!(
        ItemRanking,
        r#"
        SELECT p.nome AS "rotulo!", count(v.id) AS "total!"
        FROM produtos p
        JOIN visitas v ON v.caminho = '/produtos/' || p.slug
        GROUP BY p.id, p.nome
        ORDER BY count(v.id) DESC
        LIMIT 4
        "#
    )
    .fetch_all(pool)
    .await?;

    // --- Leads recentes ---
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
        acessos_mes,
        acessos_delta: delta_pct(acessos_mes as f64, acessos_ant as f64),
        total_leads,
        leads_delta: delta_pct(leads_mes as f64, leads_ant as f64),
        leads_novos,
        produtos_total,
        produtos_ativos,
        total_eventos,
        taxa_conversao,
        conversao_delta,
        acessos_7dias,
        origem_trafego,
        paginas,
        produtos_vistos,
        recentes,
    })
}
