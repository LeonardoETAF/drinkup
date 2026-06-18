use leptos::prelude::*;

const ICON_MENU: &str = r#"<svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" aria-hidden="true"><path d="M3 6h18M3 12h18M3 18h18"/></svg>"#;

const ICON_CLOSE: &str = r#"<svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" aria-hidden="true"><path d="M6 6l12 12M18 6L6 18"/></svg>"#;

/// Cabeçalho do site público, totalmente responsivo (menu vira hambúrguer no
/// mobile). Destinos dos links/busca são ligados na Fase 4; a busca é processada
/// no servidor (frontend burro).
#[component]
pub fn SiteHeader() -> impl IntoView {
    let (open, set_open) = signal(false);
    let close = move || set_open.set(false);

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
                        <label class="search">
                            <span class="visually-hidden">"Pesquisar produto"</span>
                            <input type="search" placeholder="Pesquisar produto..."/>
                        </label>
                        <a href="/contato" class="btn btn--primary" on:click=move |_| close()>
                            "Orçamento"
                        </a>
                    </div>
                </div>
            </div>
        </header>
    }
}
