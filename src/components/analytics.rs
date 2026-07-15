//! Ponte fina com o `dataLayer` do Google Tag Manager.
//!
//! Empurra eventos de negócio (ex.: conversão de lead) para o `dataLayer`. Quem
//! decide o que fazer com o evento é o GTM — que só está carregado se houve
//! consentimento de cookies (ver [`crate::components`] / banner). Sem GTM, o
//! evento apenas fica no array e nada é enviado, o que é o comportamento correto.

/// Empurra `{ event: <nome> }` para `window.dataLayer`, criando o array se ainda
/// não existir (o GTM reprocessa itens já presentes ao inicializar). No-op no SSR.
#[cfg(feature = "hydrate")]
pub fn push_event(nome: &str) {
    use wasm_bindgen::{JsCast, JsValue};

    let Some(win) = web_sys::window() else { return };
    let chave = JsValue::from_str("dataLayer");

    // window.dataLayer = window.dataLayer || [];
    let mut dl = js_sys::Reflect::get(&win, &chave).unwrap_or(JsValue::UNDEFINED);
    if dl.is_undefined() || dl.is_null() {
        let arr = js_sys::Array::new();
        let _ = js_sys::Reflect::set(&win, &chave, &arr);
        dl = arr.into();
    }

    // { event: nome }
    let obj = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("event"), &JsValue::from_str(nome));

    // dataLayer.push(obj) — chama o `push` do próprio objeto para respeitar a
    // versão que o GTM instala (não assume que seja um Array cru).
    if let Ok(push) = js_sys::Reflect::get(&dl, &JsValue::from_str("push")) {
        if let Ok(func) = push.dyn_into::<js_sys::Function>() {
            let _ = func.call1(&dl, &obj);
        }
    }
}

/// No-op no servidor (SSR): não há `window`/`dataLayer`.
#[cfg(not(feature = "hydrate"))]
pub fn push_event(_nome: &str) {}
