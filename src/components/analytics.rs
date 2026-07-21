//! Ponte fina com o `dataLayer` do Google Tag Manager.
//!
//! Empurra eventos de negócio para o `dataLayer`. Quem decide o que fazer com
//! eles é o GTM — que só está carregado se houve consentimento de cookies (ver
//! o banner em [`super::consentimento`]). Sem GTM, o evento apenas fica no array
//! local e **nada sai do navegador**, que é o comportamento correto.
//!
//! Todo evento leva um `event_id` único, gerado aqui. Ele é o que permite ao
//! Meta **deduplicar** a mesma conversão vinda do Pixel (navegador) e da
//! Conversions API (servidor): as duas pontas carregam o mesmo id.
//!
//! Este módulo só empurra dados. Nada de GTM/Pixel/gtag é configurado aqui.

/// Identificador único por disparo, para deduplicação Pixel ↔ CAPI.
///
/// Usa `crypto.randomUUID()` quando disponível; senão, tempo + aleatório.
#[cfg(feature = "hydrate")]
fn capi_event_id() -> String {
    use wasm_bindgen::{JsCast, JsValue};

    if let Some(win) = web_sys::window() {
        let cripto = js_sys::Reflect::get(&win, &JsValue::from_str("crypto"));
        if let Ok(cripto) = cripto {
            if let Ok(f) = js_sys::Reflect::get(&cripto, &JsValue::from_str("randomUUID")) {
                if let Ok(func) = f.dyn_into::<js_sys::Function>() {
                    if let Some(id) = func.call0(&cripto).ok().and_then(|v| v.as_string()) {
                        return id;
                    }
                }
            }
        }
    }

    let agora = js_sys::Date::now() as u64;
    let aleatorio = (js_sys::Math::random() * 1e12) as u64;
    format!("{agora:x}-{aleatorio:x}")
}

/// Empurra `{ event, event_id, ...extras }` para `window.dataLayer`.
///
/// O `event_id` é gerado internamente a cada chamada — por construção, nunca é
/// reaproveitado entre disparos. O array é criado se ainda não existir (o GTM
/// reprocessa o que já estiver lá ao inicializar). No-op no SSR.
#[cfg(feature = "hydrate")]
pub fn push_evento(evento: &str, extras: &[(&str, &str)]) {
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

    let obj = js_sys::Object::new();
    let definir = |chave: &str, valor: &str| {
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str(chave), &JsValue::from_str(valor));
    };
    definir("event", evento);
    definir("event_id", &capi_event_id());
    for (k, v) in extras {
        definir(k, v);
    }

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
pub fn push_evento(_evento: &str, _extras: &[(&str, &str)]) {}

/// Escuta cliques em **qualquer** link de WhatsApp do site (`wa.me`) por
/// delegação na janela — assim vale para header, rodapé, páginas e para
/// qualquer link novo que apareça depois, sem precisar instrumentar um a um.
///
/// Chamado uma vez, na raiz da aplicação.
#[cfg(feature = "hydrate")]
pub fn escutar_cliques_whatsapp() {
    use leptos::prelude::window_event_listener;
    use wasm_bindgen::JsCast;

    let _ = window_event_listener(leptos::ev::click, |ev| {
        let alvo = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .and_then(|el| el.closest("a[href*='wa.me']").ok().flatten());
        if alvo.is_some() {
            push_evento("dl_contact", &[]);
        }
    });
}

/// No-op no servidor (SSR).
#[cfg(not(feature = "hydrate"))]
pub fn escutar_cliques_whatsapp() {}
