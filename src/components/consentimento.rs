//! Consentimento de cookies (LGPD): opt-in explícito para os cookies não
//! essenciais (medição/Google Tag Manager).
//!
//! A decisão é uma preferência do visitante, sem valor de segurança — vive no
//! `localStorage`, não no servidor. O que ela controla é apenas se o `gtm.js`
//! chega a ser baixado: o script do GTM fica no `<head>` embrulhado numa função
//! (`window.__drinkupGtm`) que nada dispara sozinha sem o aceite gravado.
use leptos::prelude::*;

/// Chave no `localStorage`. Deve casar com a lida pelo `GTM_INIT` (`app.rs`).
#[cfg(feature = "hydrate")]
const CHAVE: &str = "consentimento";
#[cfg(feature = "hydrate")]
const ACEITO: &str = "aceito";
#[cfg(feature = "hydrate")]
const RECUSADO: &str = "recusado";

/// Sinal compartilhado que abre o banner. O rodapé o usa para reabrir as
/// preferências depois que o visitante já decidiu (a LGPD exige poder mudar de
/// ideia com a mesma facilidade com que se consentiu).
#[derive(Clone, Copy)]
pub struct AbrirConsentimento(pub RwSignal<bool>);

/// Decisão já registrada, se houver.
#[cfg(feature = "hydrate")]
fn decisao() -> Option<String> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|ls| ls.get_item(CHAVE).ok().flatten())
}

#[cfg(feature = "hydrate")]
fn gravar(valor: &str) {
    if let Some(ls) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = ls.set_item(CHAVE, valor);
    }
}

/// Executa o loader do GTM já presente no `<head>` (`window.__drinkupGtm`).
/// Chamar a função do snippet oficial — em vez de montar outra `<script>` aqui —
/// mantém um único caminho de carga do GTM e dispensa `<script>` inline no
/// cliente (que a CSP baseada em nonce bloquearia).
#[cfg(feature = "hydrate")]
fn carregar_gtm() {
    use wasm_bindgen::{JsCast, JsValue};

    let Some(win) = web_sys::window() else { return };
    let alvo = js_sys::Reflect::get(&win, &JsValue::from_str("__drinkupGtm"));
    if let Ok(func) = alvo.and_then(|v| v.dyn_into::<js_sys::Function>()) {
        let _ = func.call0(&JsValue::NULL);
    }
}

/// Recarrega a página. Usado ao revogar um consentimento já ativo: o GTM não tem
/// como ser "descarregado" da página em que já rodou, então recomeçamos do zero
/// — e no novo carregamento o `gtm.js` não é mais pedido.
#[cfg(feature = "hydrate")]
fn recarregar() {
    if let Some(win) = web_sys::window() {
        let _ = win.location().reload();
    }
}

/// O clique partiu de dentro do banner (ou do link que o abre)?
#[cfg(feature = "hydrate")]
fn clique_no_banner(ev: &leptos::ev::MouseEvent) -> bool {
    use wasm_bindgen::JsCast;

    ev.target()
        .and_then(|alvo| alvo.dyn_into::<web_sys::Element>().ok())
        .and_then(|el| {
            el.closest(".cookie-banner, .footer-links__botao")
                .ok()
                .flatten()
        })
        .is_some()
}

/// Faixa de consentimento. Não é renderizada no SSR (o HTML é o mesmo para todos
/// e cacheável); ela aparece na hidratação, e só para quem ainda não decidiu.
#[component]
pub fn BannerConsentimento() -> impl IntoView {
    let aberto = use_context::<AbrirConsentimento>()
        .map(|c| c.0)
        .unwrap_or_else(|| RwSignal::new(false));

    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        if decisao().is_none() {
            aberto.set(true);
        }
    });

    let aceitar_agora = move || {
        #[cfg(feature = "hydrate")]
        {
            gravar(ACEITO);
            carregar_gtm();
        }
        aberto.set(false);
    };

    let aceitar = move |_| aceitar_agora();

    // Clicar fora do banner também aceita (decisão do produto). O clique no
    // próprio banner e no link "Cookies" do rodapé — que é justamente quem abre
    // o banner, e cujo clique borbulharia até aqui — não contam.
    #[cfg(feature = "hydrate")]
    {
        let ouvinte = window_event_listener(leptos::ev::click, move |ev| {
            if aberto.get_untracked() && !clique_no_banner(&ev) {
                aceitar_agora();
            }
        });
        on_cleanup(move || ouvinte.remove());
    }

    let recusar = move |_| {
        #[cfg(feature = "hydrate")]
        {
            let ativo = decisao().as_deref() == Some(ACEITO);
            gravar(RECUSADO);
            if ativo {
                recarregar();
                return;
            }
        }
        aberto.set(false);
    };

    view! {
        <Show when=move || aberto.get()>
            <div
                class="cookie-banner"
                role="dialog"
                aria-modal="false"
                aria-labelledby="cookie-titulo"
            >
                <div class="cookie-banner__texto">
                    <h2 id="cookie-titulo" class="cookie-banner__titulo">"Cookies"</h2>
                    <p>
                        "Usamos cookies necessários para o site funcionar e, com a sua permissão, \
                         cookies de medição para entender como a vitrine é usada. Ao aceitar ou \
                         seguir navegando, você concorda com os cookies de medição. Você pode \
                         mudar de ideia quando quiser em “Cookies”, no rodapé."
                        <a href="/politica-de-privacidade">"Política de Privacidade"</a>
                    </p>
                </div>
                <div class="cookie-banner__acoes">
                    <button type="button" class="btn btn--primary" on:click=aceitar>
                        "Aceitar"
                    </button>
                    <button type="button" class="cookie-banner__recusar" on:click=recusar>
                        "Recusar"
                    </button>
                </div>
            </div>
        </Show>
    }
}
