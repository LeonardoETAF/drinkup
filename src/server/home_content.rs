//! Conteúdo editável da home (faixa de números + bento "Sua marca").
//! Armazenado na tabela `configuracoes` (chave/valor) com chaves `home_*`.
use std::collections::HashMap;

use sqlx::PgPool;

use crate::domain::{HomeConteudo, HomeForm};
use crate::error::AppError;

async fn mapa(pool: &PgPool) -> Result<HashMap<String, String>, sqlx::Error> {
    let rows = sqlx::query!("SELECT chave, valor FROM configuracoes")
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|r| (r.chave, r.valor)).collect())
}

/// Converte "valor | rótulo" por linha em pares (ignora linhas vazias).
fn pares(raw: &str) -> Vec<(String, String)> {
    raw.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| match l.split_once('|') {
            Some((v, r)) => (v.trim().to_string(), r.trim().to_string()),
            None => (l.trim().to_string(), String::new()),
        })
        .collect()
}

fn nao_vazia(m: &HashMap<String, String>, chave: &str) -> Option<String> {
    m.get(chave).filter(|s| !s.trim().is_empty()).cloned()
}

/// Conteúdo já resolvido para a home pública.
pub async fn obter(pool: &PgPool) -> Result<HomeConteudo, sqlx::Error> {
    let m = mapa(pool).await?;
    let g = |k: &str| m.get(k).cloned().unwrap_or_default();
    Ok(HomeConteudo {
        numeros: pares(&g("home_numeros")),
        marca_titulo: g("home_marca_titulo"),
        marca_sub: g("home_marca_sub"),
        bento: pares(&g("home_bento")),
        foto1_url: nao_vazia(&m, "home_foto1"),
        foto2_url: nao_vazia(&m, "home_foto2"),
    })
}

/// Campos crus para edição no painel.
pub async fn obter_form(pool: &PgPool) -> Result<HomeForm, sqlx::Error> {
    let m = mapa(pool).await?;
    let g = |k: &str| m.get(k).cloned().unwrap_or_default();
    Ok(HomeForm {
        numeros: g("home_numeros"),
        marca_titulo: g("home_marca_titulo"),
        marca_sub: g("home_marca_sub"),
        bento: g("home_bento"),
        foto1_url: nao_vazia(&m, "home_foto1"),
        foto2_url: nao_vazia(&m, "home_foto2"),
    })
}

/// Grava (upsert) o conteúdo da home.
pub async fn salvar(pool: &PgPool, f: &HomeForm) -> Result<(), AppError> {
    let pares = [
        ("home_numeros", f.numeros.trim()),
        ("home_marca_titulo", f.marca_titulo.trim()),
        ("home_marca_sub", f.marca_sub.trim()),
        ("home_bento", f.bento.trim()),
        ("home_foto1", f.foto1_url.as_deref().unwrap_or("").trim()),
        ("home_foto2", f.foto2_url.as_deref().unwrap_or("").trim()),
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
            tracing::error!(error = %e, "erro ao salvar conteúdo da home");
            AppError::Internal
        })?;
    }
    Ok(())
}
