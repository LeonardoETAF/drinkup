//! Erros do projeto. Detalhes internos ficam no servidor (logs); ao cliente vai
//! apenas uma mensagem genérica e segura.
use thiserror::Error;

/// Erro de aplicação seguro para cruzar a borda cliente/servidor.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AppError {
    #[error("Recurso não encontrado.")]
    NotFound,
    #[error("Não autorizado.")]
    Unauthorized,
    #[error("Dados inválidos.")]
    Validation,
    #[error("Muitas tentativas em sequência.")]
    RateLimited,
    #[error("Ocorreu um erro interno.")]
    Internal,
}
