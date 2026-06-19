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
