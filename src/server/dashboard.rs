//! Resumo do dashboard administrativo (tempo real, do banco). Server-only.
//! Tudo é filtrado pelo período (ano / mês / dia) escolhido no painel.
use sqlx::PgPool;

use crate::domain::{DashboardResumo, DiaAcesso, ItemRanking, LeadResumo};

const MESES: [&str; 12] = [
    "Jan", "Fev", "Mar", "Abr", "Mai", "Jun", "Jul", "Ago", "Set", "Out", "Nov", "Dez",
];

/// Dias no mês (com bissexto), para limitar o dia escolhido.
fn dias_no_mes(ano: i32, mes: i32) -> i32 {
    match mes {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if (ano % 4 == 0 && ano % 100 != 0) || ano % 400 == 0 => 29,
        2 => 28,
        _ => 31,
    }
}

/// Variação percentual de `cur` sobre `prev` (None quando não há base anterior).
fn delta_pct(cur: f64, prev: f64) -> Option<i32> {
    if prev <= 0.0 {
        return None;
    }
    Some((((cur - prev) / prev) * 100.0).round() as i32)
}


/// Agrega contadores, gráficos e leads recentes para o período selecionado.
/// `ano = None` → padrão = mês atual.
pub async fn resumo(
    pool: &PgPool,
    ano: Option<i32>,
    mes: Option<i32>,
    dia: Option<i32>,
) -> Result<DashboardResumo, sqlx::Error> {
    // --- Resolve o período (padrão = ano/mês atuais) ---
    let hoje = sqlx::query!(
        r#"SELECT extract(year FROM now())::int AS "y!", extract(month FROM now())::int AS "m!""#
    )
    .fetch_one(pool)
    .await?;
    let ano_atual = hoje.y;
    let (ano, mes, dia) = match ano {
        None => (ano_atual, Some(hoje.m), None),
        Some(a) => {
            let mes = mes.filter(|m| (1..=12).contains(m));
            let dia = match (mes, dia) {
                (Some(m), Some(d)) => Some(d.clamp(1, dias_no_mes(a, m))),
                _ => None,
            };
            (a, mes, dia)
        }
    };

    // --- Acessos no período x período anterior ---
    let acessos = sqlx::query!(
        r#"
        WITH per AS (SELECT make_date($1, coalesce($2,1), coalesce($3,1))::timestamptz AS d0,
          CASE WHEN $3 IS NOT NULL THEN interval '1 day'
               WHEN $2 IS NOT NULL THEN interval '1 month' ELSE interval '1 year' END AS passo)
        SELECT
          count(*) FILTER (WHERE created_at >= per.d0 AND created_at < per.d0 + per.passo) AS "atual!",
          count(*) FILTER (WHERE created_at >= per.d0 - per.passo AND created_at < per.d0) AS "prev!"
        FROM visitas, per
        "#,
        ano,
        mes,
        dia,
    )
    .fetch_one(pool)
    .await?;

    // --- Leads no período x período anterior (e total geral) ---
    let leads = sqlx::query!(
        r#"
        WITH per AS (SELECT make_date($1, coalesce($2,1), coalesce($3,1))::timestamptz AS d0,
          CASE WHEN $3 IS NOT NULL THEN interval '1 day'
               WHEN $2 IS NOT NULL THEN interval '1 month' ELSE interval '1 year' END AS passo)
        SELECT
          count(*) FILTER (WHERE created_at >= per.d0 AND created_at < per.d0 + per.passo) AS "atual!",
          count(*) FILTER (WHERE created_at >= per.d0 - per.passo AND created_at < per.d0) AS "prev!",
          count(*) FILTER (WHERE status = 'convertido'
                             AND created_at >= per.d0 AND created_at < per.d0 + per.passo) AS "conv_atual!",
          count(*) FILTER (WHERE status = 'convertido'
                             AND created_at >= per.d0 - per.passo AND created_at < per.d0) AS "conv_prev!"
        FROM leads, per
        "#,
        ano,
        mes,
        dia,
    )
    .fetch_one(pool)
    .await?;

    // --- Produtos e eventos (não dependem do período) ---
    let geral = sqlx::query!(
        r#"SELECT
             (SELECT count(*) FROM produtos) AS "prod_total!",
             (SELECT count(*) FROM produtos WHERE ativo) AS "prod_ativos!",
             (SELECT count(*) FROM eventos) AS "eventos!""#
    )
    .fetch_one(pool)
    .await?;

    // --- Conversão (leads convertidos / leads no período) ---
    let conv = |c: i64, base: i64| if base > 0 { c as f64 / base as f64 * 100.0 } else { 0.0 };
    let taxa_conversao = conv(leads.conv_atual, leads.atual);
    let conversao_delta = delta_pct(taxa_conversao, conv(leads.conv_prev, leads.prev));

    // --- Série de acessos (por hora no dia / por dia no mês / por mês no ano) ---
    let serie_rows = sqlx::query!(
        r#"
        WITH per AS (SELECT make_date($1, coalesce($2,1), coalesce($3,1))::timestamptz AS d0,
          CASE WHEN $3 IS NOT NULL THEN interval '1 day'
               WHEN $2 IS NOT NULL THEN interval '1 month' ELSE interval '1 year' END AS passo,
          CASE WHEN $3 IS NOT NULL THEN interval '1 hour'
               WHEN $2 IS NOT NULL THEN interval '1 day' ELSE interval '1 month' END AS sub)
        SELECT
          to_char(g.ts, CASE WHEN $3 IS NOT NULL THEN 'FMHH24'
                             WHEN $2 IS NOT NULL THEN 'FMDD' ELSE 'FMMM' END) AS "rotulo!",
          count(v.id) AS "total!"
        FROM per
        JOIN LATERAL generate_series(per.d0, per.d0 + per.passo - interval '1 microsecond', per.sub) AS g(ts) ON true
        LEFT JOIN visitas v ON v.created_at >= g.ts AND v.created_at < g.ts + per.sub
        GROUP BY g.ts
        ORDER BY g.ts
        "#,
        ano,
        mes,
        dia,
    )
    .fetch_all(pool)
    .await?;
    let acessos_serie = serie_rows
        .into_iter()
        .map(|r| {
            let rotulo = if mes.is_none() {
                r.rotulo
                    .parse::<usize>()
                    .ok()
                    .and_then(|m| MESES.get(m - 1).copied())
                    .unwrap_or("")
                    .to_string()
            } else if dia.is_some() {
                format!("{}h", r.rotulo)
            } else {
                r.rotulo
            };
            DiaAcesso {
                rotulo,
                total: r.total,
            }
        })
        .collect();

    // --- Produtos mais vistos no período ---
    let produtos_vistos = sqlx::query_as!(
        ItemRanking,
        r#"
        WITH per AS (SELECT make_date($1, coalesce($2,1), coalesce($3,1))::timestamptz AS d0,
          CASE WHEN $3 IS NOT NULL THEN interval '1 day'
               WHEN $2 IS NOT NULL THEN interval '1 month' ELSE interval '1 year' END AS passo)
        SELECT p.nome AS "rotulo!", count(v.id) AS "total!"
        FROM produtos p
        JOIN visitas v ON v.caminho = '/produtos/' || p.slug
        JOIN per ON v.created_at >= per.d0 AND v.created_at < per.d0 + per.passo
        GROUP BY p.id, p.nome ORDER BY count(v.id) DESC LIMIT 4
        "#,
        ano,
        mes,
        dia,
    )
    .fetch_all(pool)
    .await?;

    // --- Leads recentes no período ---
    let recentes = sqlx::query_as!(
        LeadResumo,
        r#"
        WITH per AS (SELECT make_date($1, coalesce($2,1), coalesce($3,1))::timestamptz AS d0,
          CASE WHEN $3 IS NOT NULL THEN interval '1 day'
               WHEN $2 IS NOT NULL THEN interval '1 month' ELSE interval '1 year' END AS passo)
        SELECT id AS "id!", nome AS "nome!", contato AS "contato!",
               origem AS "origem!", status AS "status!",
               to_char(created_at, 'DD/MM/YYYY') AS "inscricao!"
        FROM leads, per
        WHERE created_at >= per.d0 AND created_at < per.d0 + per.passo
        ORDER BY created_at DESC LIMIT 6
        "#,
        ano,
        mes,
        dia,
    )
    .fetch_all(pool)
    .await?;

    Ok(DashboardResumo {
        acessos_mes: acessos.atual,
        acessos_delta: delta_pct(acessos.atual as f64, acessos.prev as f64),
        total_leads: leads.atual,
        leads_delta: delta_pct(leads.atual as f64, leads.prev as f64),
        produtos_total: geral.prod_total,
        produtos_ativos: geral.prod_ativos,
        total_eventos: geral.eventos,
        taxa_conversao,
        conversao_delta,
        acessos_serie,
        produtos_vistos,
        recentes,
        sel_ano: ano,
        sel_mes: mes,
        sel_dia: dia,
        ano_atual,
    })
}
