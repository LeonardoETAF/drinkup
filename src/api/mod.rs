//! Camada de API: server functions (boundary cliente/servidor). Compilada em
//! ambos os alvos — no cliente cada função vira uma chamada HTTP; no servidor
//! executa a lógica (acesso a dados, validação, autorização).
pub mod admin;
pub mod auth;
pub mod catalogo;
pub mod categorias_admin;
pub mod config;
pub mod eventos;
pub mod eventos_admin;
pub mod home;
pub mod home_admin;
pub mod novidades;
pub mod orcamento;
pub mod parceiros;
pub mod parceiros_admin;
pub mod produtos_admin;
pub mod quem_somos;
pub mod quem_somos_admin;
pub mod recuperacao;
pub mod usuarios_admin;
