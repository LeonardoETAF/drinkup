//! Encaminhamento do handoff de atribuição para a *edge function* do CRM.
//!
//! O navegador manda os identificadores do clique (`_fbc`/`_fbp`/`fbclid`) para
//! cá; nós repassamos à edge. O motivo de passar pelo servidor, em vez de o
//! navegador chamar a edge direto, é a regra nº 1 do projeto (`CLAUDE.md`):
//! **nenhuma credencial no cliente**. A chave da edge fica só em env no
//! servidor. De quebra, dispensa liberar o domínio na CSP e evita o CORS.
//!
//! Nada de Meta/Pixel/CRM acontece aqui — só repassamos o registro.
use crate::domain::{Handoff, MARCA_ATRIBUICAO};
use crate::error::AppError;

/// URL da edge function e chave de acesso — só no servidor.
const ENV_URL: &str = "ATTRIB_EDGE_URL";
const ENV_CHAVE: &str = "ATTRIB_KEY";

/// Encaminha o registro para a edge do CRM.
///
/// Sem as variáveis de ambiente configuradas, apenas registra em log e segue —
/// o handoff é *best-effort* e jamais deve atrapalhar o visitante indo ao
/// WhatsApp.
pub async fn registrar(mut dados: Handoff) -> Result<(), AppError> {
    // Revalida no servidor o que o cliente já filtrou (frontend é descartável).
    if !Handoff::token_valido(&dados.token) || !dados.tem_atribuicao() {
        return Err(AppError::Validation);
    }
    dados.brand = MARCA_ATRIBUICAO.to_string();

    let (Ok(url), Ok(chave)) = (std::env::var(ENV_URL), std::env::var(ENV_CHAVE)) else {
        tracing::warn!("atribuição: {ENV_URL}/{ENV_CHAVE} ausentes; registro ignorado");
        return Ok(());
    };

    let resposta = reqwest::Client::new()
        .post(&url)
        .header("content-type", "application/json")
        .header("x-attrib-key", chave)
        .json(&dados)
        .send()
        .await
        .map_err(|e| {
            tracing::warn!(erro = %e, "atribuição: falha ao chamar a edge");
            AppError::Internal
        })?;

    if !resposta.status().is_success() {
        // O corpo traz o motivo (invalid_token, rate_limited, ...) — útil no log
        // do servidor, nunca no cliente.
        let status = resposta.status();
        let corpo = resposta.text().await.unwrap_or_default();
        tracing::warn!(%status, %corpo, "atribuição: edge recusou o registro");
        return Err(AppError::Internal);
    }
    Ok(())
}
