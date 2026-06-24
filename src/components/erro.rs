//! Conversão de erros de server function em mensagens limpas para o usuário.
use leptos::prelude::ServerFnError;

/// Mensagem amigável (PT-BR) a partir de um `ServerFnError`, sem prefixos
/// internos do framework ("error running server function: …") nem detalhes
/// técnicos. Erros de servidor já trazem texto pronto para o usuário; falhas de
/// rede/serialização viram uma mensagem genérica.
#[must_use]
pub fn mensagem_erro(e: &ServerFnError) -> String {
    match e {
        ServerFnError::ServerError(msg) => msg.clone(),
        _ => "Não foi possível concluir a ação. Tente novamente.".to_string(),
    }
}
