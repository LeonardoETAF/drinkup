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
    push_evento_com_id(evento, &capi_event_id(), extras);
}

/// Igual a [`push_evento`], mas com o `event_id` fornecido — para quando o mesmo
/// id precisa ser reaproveitado fora do `dataLayer` (ex.: o handoff de
/// atribuição amarra o `event_id` do Contact ao registro no CRM).
#[cfg(feature = "hydrate")]
pub fn push_evento_com_id(evento: &str, event_id: &str, extras: &[(&str, &str)]) {
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
    definir("event_id", event_id);
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

/// Lê um cookie do documento.
#[cfg(feature = "hydrate")]
fn cookie(nome: &str) -> Option<String> {
    use wasm_bindgen::JsValue;

    let doc = web_sys::window()?.document()?;
    let bruto = js_sys::Reflect::get(&doc, &JsValue::from_str("cookie"))
        .ok()?
        .as_string()?;
    let prefixo = format!("{nome}=");
    bruto
        .split(';')
        .map(str::trim)
        .find_map(|p| p.strip_prefix(&prefixo))
        .map(|v| v.to_string())
        .filter(|v| !v.is_empty())
}

/// Valor (já decodificado) de um parâmetro da query string da página atual.
#[cfg(feature = "hydrate")]
fn param_url(nome: &str) -> Option<String> {
    let busca = web_sys::window()?.location().search().ok()?;
    busca
        .trim_start_matches('?')
        .split('&')
        .filter_map(|par| par.split_once('='))
        .find(|(chave, _)| *chave == nome)
        .map(|(_, valor)| urlencoding::decode(valor).unwrap_or_default().into_owned())
        .filter(|v| !v.is_empty())
}

/// `_fbc` (Meta Click ID). Se o cookie não existe mas há `fbclid` na URL,
/// reconstrói no formato que o Meta espera: `fb.1.<timestamp_ms>.<fbclid>`.
#[cfg(feature = "hydrate")]
fn fbc() -> Option<String> {
    cookie("_fbc").or_else(|| {
        param_url("fbclid").map(|id| format!("fb.1.{}.{id}", js_sys::Date::now() as u64))
    })
}

/// Token do handoff: `DU-` + 8 caracteres `A-Z0-9`, como a edge exige.
/// Deriva de um UUID criptográfico (hex em maiúsculas cabe em `[A-Z0-9]`).
#[cfg(feature = "hydrate")]
fn gerar_token() -> String {
    let bruto: String = capi_event_id()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .take(8)
        .collect();
    format!("DU-{}", bruto.to_uppercase())
}

/// Envia o handoff ao nosso servidor (que repassa à edge com a chave). Usa
/// `keepalive` para a requisição sobreviver à navegação até o WhatsApp, e é
/// disparada sem espera — o visitante nunca aguarda por ela.
#[cfg(feature = "hydrate")]
fn registrar_handoff(corpo: String) {
    use wasm_bindgen::{JsCast, JsValue};

    let Some(win) = web_sys::window() else { return };

    let cabecalhos = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &cabecalhos,
        &JsValue::from_str("content-type"),
        &JsValue::from_str("application/json"),
    );

    let init = js_sys::Object::new();
    let def = |k: &str, v: &JsValue| {
        let _ = js_sys::Reflect::set(&init, &JsValue::from_str(k), v);
    };
    def("method", &JsValue::from_str("POST"));
    def("headers", &cabecalhos);
    def("body", &JsValue::from_str(&corpo));
    def("keepalive", &JsValue::TRUE);

    if let Ok(f) = js_sys::Reflect::get(&win, &JsValue::from_str("fetch")) {
        if let Ok(fetch) = f.dyn_into::<js_sys::Function>() {
            // Erros são ignorados de propósito: best-effort.
            let _ = fetch.call2(&win, &JsValue::from_str("/api/atribuicao"), &init);
        }
    }
}

/// Escuta cliques em **qualquer** link de WhatsApp do site (`wa.me`) por
/// delegação na janela — assim vale para header, rodapé, páginas e para
/// qualquer link novo que apareça depois, sem precisar instrumentar um a um.
///
/// A cada clique: dispara o `dl_contact` e, havendo consentimento e ao menos um
/// identificador de atribuição, gera um token, registra o handoff e injeta
/// `?text=...(ref: TOKEN)` no link — o CRM lê esse token na 1ª mensagem para
/// ligar a conversa ao anúncio.
///
/// A URL é reescrita **no próprio elemento**, deixando a navegação padrão
/// acontecer: preserva `target="_blank"`, não esbarra em bloqueador de pop-up e
/// não atrasa a ida ao WhatsApp.
///
/// Chamado uma vez, na raiz da aplicação.
#[cfg(feature = "hydrate")]
pub fn escutar_cliques_whatsapp() {
    use leptos::prelude::window_event_listener;
    use wasm_bindgen::JsCast;

    let _ = window_event_listener(leptos::ev::click, |ev| {
        let Some(link) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .and_then(|el| el.closest("a[href*='wa.me']").ok().flatten())
        else {
            return;
        };

        let id_evento = capi_event_id();
        push_evento_com_id("dl_contact", &id_evento, &[]);

        // Sem consentimento não há rastreamento: o WhatsApp abre normalmente,
        // sem token e sem registro (mesmo critério que libera o GTM/Pixel).
        if !super::consentimento::rastreio_permitido() {
            return;
        }

        // Sem nenhum identificador não há o que atribuir — não gera token nem
        // chama o servidor; o link segue como está.
        let (fbc, fbp, fbclid) = (fbc(), cookie("_fbp"), param_url("fbclid"));
        if fbc.is_none() && fbp.is_none() && fbclid.is_none() {
            return;
        }

        let token = gerar_token();
        let href = link.get_attribute("href").unwrap_or_default();
        let (base, _) = href.split_once('?').unwrap_or((href.as_str(), ""));
        let texto =
            urlencoding::encode(&format!("Quero um orçamento 🙌 (ref: {token})")).into_owned();
        let _ = link.set_attribute("href", &format!("{base}?text={texto}"));

        let handoff = crate::domain::Handoff {
            token,
            brand: crate::domain::MARCA_ATRIBUICAO.to_string(),
            fbc,
            fbp,
            fbclid,
            user_agent: web_sys::window().and_then(|w| w.navigator().user_agent().ok()),
            event_id_contact: Some(id_evento),
            page_url: web_sys::window().and_then(|w| w.location().href().ok()),
        };
        if let Ok(corpo) = serde_json::to_string(&handoff) {
            registrar_handoff(corpo);
        }
    });
}

/// No-op no servidor (SSR).
#[cfg(not(feature = "hydrate"))]
pub fn escutar_cliques_whatsapp() {}
