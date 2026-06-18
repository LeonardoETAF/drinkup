//! Camada de API: server functions (boundary cliente/servidor). Compilada em
//! ambos os alvos — no cliente cada função vira uma chamada HTTP; no servidor
//! executa a lógica (acesso a dados, validação, autorização).
pub mod admin;
pub mod auth;
pub mod catalogo;
pub mod orcamento;
