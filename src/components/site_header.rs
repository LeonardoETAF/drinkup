use leptos::prelude::*;

use crate::api::config::obter_contato;
use crate::domain::link_whatsapp;

const ICON_MENU: &str = r#"<svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" aria-hidden="true"><path d="M3 6h18M3 12h18M3 18h18"/></svg>"#;

const ICON_CLOSE: &str = r#"<svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" aria-hidden="true"><path d="M6 6l12 12M18 6L6 18"/></svg>"#;

const ICON_WHATS: &str = r#"<svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M.057 24l1.687-6.163a11.867 11.867 0 0 1-1.587-5.946C.16 5.335 5.495 0 12.05 0a11.821 11.821 0 0 1 8.413 3.488 11.824 11.824 0 0 1 3.48 8.414c-.003 6.557-5.338 11.892-11.893 11.892a11.9 11.9 0 0 1-5.688-1.448L.057 24zm6.597-3.807c1.676.995 3.276 1.591 5.392 1.592 5.448 0 9.886-4.434 9.889-9.885.002-5.462-4.415-9.89-9.881-9.892-5.452 0-9.887 4.434-9.889 9.884a9.86 9.86 0 0 0 1.51 5.26l-.999 3.648 3.742-.981zm11.387-5.464c-.074-.124-.272-.198-.57-.347-.297-.149-1.758-.868-2.031-.967-.272-.099-.47-.149-.669.149-.198.297-.768.967-.941 1.165-.173.198-.347.223-.644.074-.297-.149-1.255-.462-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.297-.347.446-.521.151-.172.2-.296.3-.495.099-.198.05-.372-.025-.521-.075-.148-.669-1.611-.916-2.206-.242-.579-.487-.501-.669-.51l-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.626.712.226 1.36.194 1.872.118.571-.085 1.758-.719 2.006-1.413.248-.695.248-1.29.173-1.414z"/></svg>"#;

/// Cabeçalho do site público, totalmente responsivo (menu vira hambúrguer no
/// mobile). Destinos dos links/busca são ligados na Fase 4; a busca é processada
/// no servidor (frontend burro).
#[component]
pub fn SiteHeader() -> impl IntoView {
    let (open, set_open) = signal(false);
    let close = move || set_open.set(false);

    // Botão "Orçamento" abre o WhatsApp cadastrado (fallback: página de contato).
    let info = Resource::new(|| (), |_| async move { obter_contato().await });
    let link_orcamento = move || {
        info.get()
            .and_then(Result::ok)
            .and_then(|c| link_whatsapp(&c.telefone))
            .unwrap_or_else(|| "/contato".to_string())
    };

    view! {
        <header class="site-header">
            <div class="container site-header__inner">
                <a href="/" class="site-header__logo" aria-label="DRINK UP — início">
                    <img src="/brand/logo-branca.png" alt="DRINK UP" width="135" height="30"/>
                </a>

                <button
                    class="nav-toggle"
                    type="button"
                    aria-label="Abrir/fechar menu"
                    aria-controls="menu-principal"
                    aria-expanded=move || if open.get() { "true" } else { "false" }
                    on:click=move |_| set_open.update(|o| *o = !*o)
                    inner_html=move || if open.get() { ICON_CLOSE } else { ICON_MENU }
                ></button>

                <div id="menu-principal" class="site-menu" class:is-open=move || open.get()>
                    <nav class="site-nav" aria-label="Navegação principal">
                        <a href="/" on:click=move |_| close()>"Início"</a>
                        <a href="/quem-somos" on:click=move |_| close()>"Quem Somos"</a>
                        <a href="/produtos" on:click=move |_| close()>"Produtos"</a>
                        <a href="/parceiros" on:click=move |_| close()>"Parceiros"</a>
                        <a href="/contato" on:click=move |_| close()>"Contato"</a>
                    </nav>
                    <div class="site-header__actions">
                        <a
                            href=link_orcamento
                            class="btn btn--primary btn--sm"
                            target="_blank"
                            rel="noopener noreferrer"
                            on:click=move |_| close()
                        >
                            <span class="btn__ic" inner_html=ICON_WHATS></span>
                            "Orçamento"
                        </a>
                    </div>
                </div>
            </div>
        </header>
    }
}
