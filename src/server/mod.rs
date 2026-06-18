//! Lógica de servidor — compilada APENAS com a feature `ssr`.
//!
//! Princípio do frontend burro: tudo aqui (acesso a dados, autenticação,
//! autorização/RBAC e regras de negócio sensíveis) vive no servidor e NUNCA
//! vai ao cliente. As server functions são a única porta de entrada para dados.
pub mod auth;
pub mod dashboard;
pub mod db;
pub mod leads;
pub mod products;
pub mod produtos_admin;
pub mod quotes;
pub mod rbac;
pub mod uploads;
