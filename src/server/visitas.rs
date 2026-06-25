//! Registro de acessos (page views) das páginas públicas. Server-only.
//! Analytics não pode derrubar a página: falhas são apenas logadas.
use sqlx::PgPool;

/// Origens de tráfego válidas (espelham o CHECK da tabela `visitas`).
const ORIGENS: [&str; 6] = [
    "instagram",
    "google",
    "facebook",
    "whatsapp",
    "direto",
    "outro",
];

/// Deriva a origem do tráfego a partir do cabeçalho `Referer` e do `Host` atual.
/// Sem referer → acesso direto; referer do próprio site → direto; senão mapeia.
#[must_use]
pub fn origem_do_referer(referer: Option<&str>, host: &str) -> &'static str {
    let Some(r) = referer else { return "direto" };
    let r = r.to_lowercase();
    if r.contains("instagram") {
        "instagram"
    } else if r.contains("google") {
        "google"
    } else if r.contains("facebook") || r.contains("//fb.") || r.contains(".fb.") {
        "facebook"
    } else if r.contains("whatsapp") || r.contains("wa.me") {
        "whatsapp"
    } else if !host.is_empty() && r.contains(&host.to_lowercase()) {
        "direto"
    } else {
        "outro"
    }
}

/// Registra a visualização da página de um produto (pelo slug), contabilizada
/// ao entrar na tela de detalhe — inclusive em navegação no cliente (SPA).
/// Só registra se o produto existir, para não poluir o rastreio.
pub async fn registrar_produto(pool: &PgPool, slug: &str) {
    // Insere a visita apenas se o produto existir — numa única query — já
    // gravando o produto_id (o SELECT só produz linha quando há produto).
    if let Err(e) = sqlx::query!(
        r#"INSERT INTO visitas (caminho, origem, produto_id)
           SELECT '/produtos/' || $1, 'direto', p.id
           FROM produtos p WHERE p.slug = $1"#,
        slug,
    )
    .execute(pool)
    .await
    {
        tracing::warn!(error = %e, "falha ao registrar visita de produto");
    }
}

/// Registra uma visita (caminho público + origem). Best-effort.
pub async fn registrar(pool: &PgPool, caminho: &str, origem: &str) {
    let origem = if ORIGENS.contains(&origem) {
        origem
    } else {
        "outro"
    };
    if let Err(e) = sqlx::query!(
        "INSERT INTO visitas (caminho, origem) VALUES ($1, $2)",
        caminho,
        origem,
    )
    .execute(pool)
    .await
    {
        tracing::warn!(error = %e, "falha ao registrar visita");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Integração: registrar_produto grava o produto_id da visita.
    #[tokio::test]
    async fn registra_produto_id() {
        let Ok(url) = std::env::var("DATABASE_URL") else {
            return;
        };
        let Ok(pool) = crate::server::db::create_pool(&url).await else {
            return;
        };
        let Some(prod) = sqlx::query!("SELECT id, slug FROM produtos LIMIT 1")
            .fetch_optional(&pool)
            .await
            .unwrap()
        else {
            return; // sem produtos no banco de teste
        };

        registrar_produto(&pool, &prod.slug).await;

        let row = sqlx::query!(
            r#"SELECT id, produto_id FROM visitas
               WHERE caminho = '/produtos/' || $1 ORDER BY id DESC LIMIT 1"#,
            prod.slug,
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(row.produto_id, Some(prod.id), "grava o produto_id");

        let _ = sqlx::query!("DELETE FROM visitas WHERE id = $1", row.id)
            .execute(&pool)
            .await;
    }
}
