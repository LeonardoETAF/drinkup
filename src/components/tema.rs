use leptos::prelude::*;

const IC_SOL: &str = r#"<svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/></svg>"#;
const IC_LUA: &str = r#"<svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8z"/></svg>"#;

/// Aplica o tema no `<html>` e persiste no `localStorage` (apenas no cliente).
#[cfg(feature = "hydrate")]
fn aplicar_tema(claro: bool) {
    let valor = if claro { "light" } else { "dark" };
    if let Some(win) = web_sys::window() {
        if let Some(el) = win.document().and_then(|d| d.document_element()) {
            let _ = el.set_attribute("data-theme", valor);
        }
        if let Ok(Some(ls)) = win.local_storage() {
            let _ = ls.set_item("tema", valor);
        }
    }
}

/// Lê o tema atual do `<html>` (o script anti-flash já o aplicou).
#[cfg(feature = "hydrate")]
fn tema_atual_claro() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
        .and_then(|el| el.get_attribute("data-theme"))
        .map(|t| t == "light")
        .unwrap_or(false)
}

/// Botão de alternância de tema (claro/escuro). O escuro é o padrão.
/// A preferência é só de apresentação (sem segredo), guardada no `localStorage`.
#[component]
pub fn BotaoTema() -> impl IntoView {
    // true = tema claro. Começa no escuro (padrão) e sincroniza na hidratação.
    let claro = RwSignal::new(false);

    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        claro.set(tema_atual_claro());
    });

    let alternar = move |_| {
        let novo = !claro.get_untracked();
        claro.set(novo);
        #[cfg(feature = "hydrate")]
        aplicar_tema(novo);
    };

    view! {
        <button
            type="button"
            class="theme-toggle"
            aria-label="Alternar tema claro/escuro"
            title="Alternar tema"
            on:click=alternar
            inner_html=move || if claro.get() { IC_LUA } else { IC_SOL }
        ></button>
    }
}
