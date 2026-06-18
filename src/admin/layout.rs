use leptos::prelude::*;
use leptos_router::components::{Outlet, Redirect};

use crate::api::auth::{logout, usuario_atual};

/// Layout do painel: protege as rotas (redireciona não autenticados) e provê a
/// casca (topo + área de conteúdo via `Outlet`). A autorização é do servidor.
#[component]
pub fn AdminLayout() -> impl IntoView {
    // Blocking: resolve a checagem de sessão ANTES de streamar, permitindo o
    // redirect server-side (302) para não autenticados.
    let usuario = Resource::new_blocking(|| (), |_| async move { usuario_atual().await });

    view! {
        <Suspense fallback=|| {
            view! { <div class="admin-loading">"Carregando..."</div> }
        }>
            {move || Suspend::new(async move {
                match usuario.await {
                    Ok(Some(u)) => view! { <AdminShell nome=u.nome papel=u.papel/> }.into_any(),
                    // Em SSR/HTTP direto o middleware já barra com 302; aqui cobre
                    // a navegação client-side (SPA).
                    _ => view! { <Redirect path="/admin/login"/> }.into_any(),
                }
            })}
        </Suspense>
    }
}

#[component]
fn AdminShell(nome: String, papel: String) -> impl IntoView {
    let info = StoredValue::new((nome, papel));
    let sair = Action::new(|_: &()| async move { logout().await });
    let saiu = move || matches!(sair.value().get(), Some(Ok(())));

    view! {
        <Show
            when=saiu
            fallback=move || {
                let (nome, papel) = info.get_value();
                view! {
                    <div class="admin-shell">
                        <header class="admin-top">
                            <a href="/admin" class="admin-top__brand">
                                <img
                                    src="/brand/logo-branca.png"
                                    alt="DRINK UP"
                                    width="120"
                                    height="27"
                                />
                            </a>
                            <div class="admin-top__user">
                                <span class="admin-top__nome">{nome}</span>
                                <span class="admin-top__papel">{papel}</span>
                                <button
                                    class="btn btn--ghost"
                                    on:click=move |_| {
                                        sair.dispatch(());
                                    }
                                >
                                    "Sair"
                                </button>
                            </div>
                        </header>
                        <main class="admin-main">
                            <Outlet/>
                        </main>
                    </div>
                }
            }
        >
            <Redirect path="/admin/login"/>
        </Show>
    }
}
