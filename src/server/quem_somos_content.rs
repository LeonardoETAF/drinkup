//! Conteúdo editável da página "Quem Somos".
//! Armazenado na tabela `configuracoes` (chave/valor) com chaves `qs_*`.
use std::collections::HashMap;

use sqlx::PgPool;

use crate::domain::{QuemSomosConteudo, QuemSomosForm};
use crate::error::AppError;

async fn mapa(pool: &PgPool) -> Result<HashMap<String, String>, sqlx::Error> {
    let rows = sqlx::query!("SELECT chave, valor FROM configuracoes")
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|r| (r.chave, r.valor)).collect())
}

/// Converte "texto | autor" por linha em pares (ignora linhas vazias).
fn pares(raw: &str) -> Vec<(String, String)> {
    raw.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| match l.split_once('|') {
            Some((t, a)) => (t.trim().to_string(), a.trim().to_string()),
            None => (l.trim().to_string(), String::new()),
        })
        .collect()
}

fn nao_vazia(m: &HashMap<String, String>, chave: &str) -> Option<String> {
    m.get(chave).filter(|s| !s.trim().is_empty()).cloned()
}

/// Conteúdo já resolvido para a página pública.
pub async fn obter(pool: &PgPool) -> Result<QuemSomosConteudo, sqlx::Error> {
    let m = mapa(pool).await?;
    let g = |k: &str| m.get(k).cloned().unwrap_or_default();
    Ok(QuemSomosConteudo {
        stat_titulo: g("qs_stat_titulo"),
        stat_destaque: g("qs_stat_destaque"),
        stat_texto: g("qs_stat_texto"),
        video_url: nao_vazia(&m, "qs_video"),
        missao: g("qs_missao"),
        visao: g("qs_visao"),
        valores: g("qs_valores"),
        foto1_url: nao_vazia(&m, "qs_foto1"),
        foto2_url: nao_vazia(&m, "qs_foto2"),
        depoimentos: pares(&g("qs_depoimentos")),
    })
}

/// Campos crus para edição no painel.
pub async fn obter_form(pool: &PgPool) -> Result<QuemSomosForm, sqlx::Error> {
    let m = mapa(pool).await?;
    let g = |k: &str| m.get(k).cloned().unwrap_or_default();
    Ok(QuemSomosForm {
        stat_titulo: g("qs_stat_titulo"),
        stat_destaque: g("qs_stat_destaque"),
        stat_texto: g("qs_stat_texto"),
        video_url: nao_vazia(&m, "qs_video"),
        missao: g("qs_missao"),
        visao: g("qs_visao"),
        valores: g("qs_valores"),
        foto1_url: nao_vazia(&m, "qs_foto1"),
        foto2_url: nao_vazia(&m, "qs_foto2"),
        depoimentos: g("qs_depoimentos"),
    })
}

/// Grava (upsert) o conteúdo da página "Quem Somos".
pub async fn salvar(pool: &PgPool, f: &QuemSomosForm) -> Result<(), AppError> {
    let pares = [
        ("qs_stat_titulo", f.stat_titulo.trim()),
        ("qs_stat_destaque", f.stat_destaque.trim()),
        ("qs_stat_texto", f.stat_texto.trim()),
        ("qs_video", f.video_url.as_deref().unwrap_or("").trim()),
        ("qs_missao", f.missao.trim()),
        ("qs_visao", f.visao.trim()),
        ("qs_valores", f.valores.trim()),
        ("qs_foto1", f.foto1_url.as_deref().unwrap_or("").trim()),
        ("qs_foto2", f.foto2_url.as_deref().unwrap_or("").trim()),
        ("qs_depoimentos", f.depoimentos.trim()),
    ];
    for (chave, valor) in pares {
        sqlx::query!(
            r#"INSERT INTO configuracoes (chave, valor) VALUES ($1, $2)
               ON CONFLICT (chave) DO UPDATE SET valor = EXCLUDED.valor"#,
            chave,
            valor,
        )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "erro ao salvar conteúdo de Quem Somos");
            AppError::Internal
        })?;
    }
    Ok(())
}
