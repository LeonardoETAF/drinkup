//! DRINKUP — biblioteca compartilhada (UI + servidor).
//!
//! Regra do projeto: nenhum `unsafe` no nosso código. A única exceção é o ponto
//! de entrada de hidratação (módulo `hydration`), cujo `unsafe` é gerado pelo
//! `wasm-bindgen` (glue do framework, não lógica nossa) e fica isolado e anotado.
#![deny(unsafe_code)]

pub mod admin;
pub mod api;
pub mod app;
pub mod components;
pub mod domain;
pub mod error;
pub mod pages;

// Lógica de servidor: compilada apenas com a feature `ssr`. Nunca vai ao cliente.
#[cfg(feature = "ssr")]
pub mod server;

// Bootstrap de hidratação no navegador (feature `hydrate`).
#[cfg(feature = "hydrate")]
mod hydration;
