//! Camada de API: server functions (boundary cliente/servidor). Compilada em
//! ambos os alvos — no cliente cada função vira uma chamada HTTP; no servidor
//! executa a lógica (acesso a dados, validação, autorização).
pub mod admin;
pub mod auth;
pub mod catalogo;
pub mod config;
pub mod eventos;
pub mod eventos_admin;
pub mod orcamento;
pub mod parceiros;
pub mod parceiros_admin;
pub mod produtos_admin;
pub mod recuperacao;
pub mod usuarios_admin;
