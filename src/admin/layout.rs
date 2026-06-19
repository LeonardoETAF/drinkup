use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::{Outlet, Redirect};
use leptos_router::hooks::use_location;

use super::util::iniciais;
use crate::api::auth::{logout, usuario_atual};
use crate::domain::UsuarioSessao;

const IC_DASH: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect x="3" y="3" width="7" height="7" rx="1"/><rect x="14" y="3" width="7" height="7" rx="1"/><rect x="14" y="14" width="7" height="7" rx="1"/><rect x="3" y="14" width="7" height="7" rx="1"/></svg>"#;
const IC_PROD: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><path d="M3.3 7L12 12l8.7-5M12 22V12"/></svg>"#;
const IC_LEADS: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M21 11.5a8.4 8.4 0 0 1-9 8 9 9 0 0 1-4-1L3 20l1.5-4.5A8.4 8.4 0 1 1 21 11.5z"/></svg>"#;
const IC_PARC: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.9M16 3.1a4 4 0 0 1 0 7.8"/></svg>"#;
const IC_EVT: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect x="3" y="4" width="18" height="18" rx="2"/><path d="M16 2v4M8 2v4M3 10h18"/></svg>"#;
const IC_CFG: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.6 1.6 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.6 1.6 0 0 0-2.7 1.2V21a2 2 0 1 1-4 0v-.1A1.6 1.6 0 0 0 9 19.4a1.6 1.6 0 0 0-1.8.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.6 1.6 0 0 0-1.2-2.7H3a2 2 0 1 1 0-4h.1A1.6 1.6 0 0 0 4.6 9a1.6 1.6 0 0 0-.3-1.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.6 1.6 0 0 0 1.8.3H9a1.6 1.6 0 0 0 1.2-1.5V3a2 2 0 1 1 4 0v.1A1.6 1.6 0 0 0 15 4.6a1.6 1.6 0 0 0 1.8-.3l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.6 1.6 0 0 0-.3 1.8V9a1.6 1.6 0 0 0 1.5 1.2H21a2 2 0 1 1 0 4h-.1a1.6 1.6 0 0 0-1.5 1z"/></svg>"#;
const IC_CONT: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18M9 21V9"/></svg>"#;
const IC_INFO: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4M12 8h.01"/></svg>"#;
const IC_SAIR: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><path d="M16 17l5-5-5-5M21 12H9"/></svg>"#;

/// Layout do painel: provê a casca (sidebar + topo + `Outlet`). A proteção da
/// rota é feita server-side pelo middleware (303); aqui os dados do usuário são
/// carregados no cliente após a hidratação (sem acessar sessão/banco no SSR).
#[component]
pub fn AdminLayout() -> impl IntoView {
    // None = carregando; Some(None) = não autenticado; Some(Some(u)) = usuário.
    let usuario = RwSignal::new(None::<Option<UsuarioSessao>>);
    Effect::new(move |_| {
        spawn_local(async move {
            usuario.set(Some(usuario_atual().await.unwrap_or(None)));
        });
    });

    let sair = Action::new(|_: &()| async move { logout().await });
    let saiu = move || matches!(sair.value().get(), Some(Ok(())));
    let nao_auth = move || matches!(usuario.get(), Some(None));

    let nome = move || usuario.get().flatten().map(|u| u.nome).unwrap_or_default();
    let papel = move || usuario.get().flatten().map(|u| u.papel).unwrap_or_default();
    // Menus permitidos do usuário logado (filtram a navegação).
    let menus = Memo::new(move |_| {
        usuario.get().flatten().map(|u| u.menus).unwrap_or_default()
    });
    let avatar = move || {
        usuario
            .get()
            .flatten()
            .map(|u| iniciais(&u.nome))
            .unwrap_or_else(|| "··".to_string())
    };

    view! {
        {move || {
            (saiu() || nao_auth()).then(|| view! { <Redirect path="/admin/login"/> })
        }}
        <div class="admin">
            <aside class="admin-side">
                <div class="admin-side__brand">
                    <img src="/brand/logo-branca.png" alt="DRINK UP" width="120" height="27"/>
                    <span class="admin-side__tag">"Admin"</span>
                </div>
                <nav class="admin-nav">
                    <NavLink href="/admin" label="Dashboard" icon=IC_DASH menu="dashboard" menus/>
                    <NavLink
                        href="/admin/produtos"
                        label="Produtos"
                        icon=IC_PROD
                        menu="produtos"
                        menus
                    />
                    <NavLink href="/admin/leads" label="Leads" icon=IC_LEADS menu="leads" menus/>
                    <NavLink
                        href="/admin/parceiros"
                        label="Parceiros"
                        icon=IC_PARC
                        menu="parceiros"
                        menus
                    />
                    <NavLink
                        href="/admin/eventos"
                        label="Eventos"
                        icon=IC_EVT
                        menu="eventos"
                        menus
                    />
                    <NavLink
                        href="/admin/conteudo"
                        label="Conteúdo Home"
                        icon=IC_CONT
                        menu="conteudo"
                        menus
                    />
                    <NavLink
                        href="/admin/conteudo-quem-somos"
                        label="Quem Somos"
                        icon=IC_INFO
                        menu="quem-somos"
                        menus
                    />
                    <NavLink
                        href="/admin/configuracoes"
                        label="Configurações"
                        icon=IC_CFG
                        menu="configuracoes"
                        menus
                    />
                </nav>
                <button
                    class="admin-nav__sair"
                    on:click=move |_| {
                        sair.dispatch(());
                    }
                >
                    <span class="admin-nav__icon" inner_html=IC_SAIR></span>
                    <span>"Sair"</span>
                </button>
            </aside>

            <div class="admin-body">
                <header class="admin-top">
                    <div class="admin-top__user">
                        <span class="admin-top__avatar">{avatar}</span>
                        <span class="admin-top__meta">
                            <strong>{nome}</strong>
                            <span class="admin-top__papel">{papel}</span>
                        </span>
                    </div>
                </header>
                <main class="admin-main">
                    <Outlet/>
                </main>
            </div>
        </div>
    }
}

#[component]
fn NavLink(
    href: &'static str,
    label: &'static str,
    icon: &'static str,
    menu: &'static str,
    menus: Memo<Vec<String>>,
) -> impl IntoView {
    let loc = use_location();
    let ativo = move || loc.pathname.get() == href;
    move || {
        menus
            .get()
            .iter()
            .any(|m| m == menu)
            .then(|| {
                view! {
                    <a class="admin-nav__item" class:is-active=ativo href=href>
                        <span class="admin-nav__icon" inner_html=icon></span>
                        <span>{label}</span>
                    </a>
                }
            })
    }
}
