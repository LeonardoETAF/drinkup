//! Ponto de entrada de hidratação no navegador.
//!
//! Único lugar onde `unsafe` é permitido no projeto: o glue gerado pelo
//! `#[wasm_bindgen]`. Não há lógica do projeto aqui — apenas o bootstrap.
#![allow(unsafe_code)]

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
