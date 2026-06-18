use leptos::prelude::*;

use crate::domain::NovoOrcamento;

/// Recebe um pedido de orçamento/contato do cliente. A validação real, o
/// anti-spam e a persistência acontecem no servidor (frontend burro).
///
/// `honeypot` é um campo oculto: se vier preenchido, é bot — descartamos
/// silenciosamente (fingindo sucesso).
#[server]
pub async fn enviar_orcamento(dados: NovoOrcamento, honeypot: String) -> Result<(), ServerFnError> {
    use crate::error::AppError;

    if !honeypot.trim().is_empty() {
        tracing::warn!("honeypot acionado; descartando envio de orçamento");
        return Ok(());
    }

    let pool = expect_context::<sqlx::PgPool>();
    match crate::server::quotes::criar(&pool, &dados).await {
        Ok(()) => Ok(()),
        Err(AppError::Validation) => {
            Err(ServerFnError::new("Dados inválidos. Verifique os campos."))
        }
        Err(AppError::RateLimited) => Err(ServerFnError::new(
            "Recebemos um pedido seu agora há pouco. Aguarde um instante antes de enviar outro.",
        )),
        Err(_) => Err(ServerFnError::new(
            "Não foi possível enviar seu pedido. Tente novamente.",
        )),
    }
}
