//! Componente raiz, shell HTML e roteador.
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    ParamSegment, StaticSegment,
};

use crate::admin::{
    AdminDashboard, AdminEmBreve, AdminEventoForm, AdminEventos, AdminLayout, AdminLeads,
    AdminLoginPage, AdminParceiroForm, AdminParceiros, AdminProdutoForm, AdminProdutos,
    AdminRecuperarSenhaPage,
};
use crate::components::{SiteFooter, SiteHeader};
use crate::pages::{ContatoPage, EmBrevePage, HomePage, ProdutoPage, ProdutosPage};

/// Documento HTML renderizado no servidor (SSR).
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="pt-BR">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" href="/favicon.png"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// Aplicação Leptos (compartilhada entre SSR e hidratação).
///
/// O cabeçalho/rodapé do site público envolvem as rotas. O painel admin terá
/// seu próprio layout (rotas aninhadas) a partir da Fase 7.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/drinkup.css"/>
        <Title text="DRINK UP — Copos personalizados"/>
        <Router>
            <Routes fallback=NotFound>
                // Site público (com cabeçalho/rodapé).
                <ParentRoute path=StaticSegment("") view=PublicLayout>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("produtos") view=ProdutosPage/>
                    <Route
                        path=(StaticSegment("produtos"), ParamSegment("slug"))
                        view=ProdutoPage
                    />
                    <Route
                        path=StaticSegment("quem-somos")
                        view=|| view! { <EmBrevePage kicker="Sobre" titulo="Quem Somos"/> }
                    />
                    <Route
                        path=StaticSegment("parceiros")
                        view=|| view! { <EmBrevePage kicker="Rede" titulo="Parceiros"/> }
                    />
                    <Route path=StaticSegment("contato") view=ContatoPage/>
                </ParentRoute>

                // Painel administrativo (layout próprio, rotas protegidas).
                <Route
                    path=(StaticSegment("admin"), StaticSegment("login"))
                    view=AdminLoginPage
                />
                <Route
                    path=(StaticSegment("admin"), StaticSegment("recuperar-senha"))
                    view=AdminRecuperarSenhaPage
                />
                <ParentRoute path=StaticSegment("admin") view=AdminLayout>
                    <Route path=StaticSegment("") view=AdminDashboard/>
                    <Route path=StaticSegment("leads") view=AdminLeads/>
                    <Route path=StaticSegment("produtos") view=AdminProdutos/>
                    <Route
                        path=(StaticSegment("produtos"), StaticSegment("novo"))
                        view=AdminProdutoForm
                    />
                    <Route
                        path=(StaticSegment("produtos"), ParamSegment("id"))
                        view=AdminProdutoForm
                    />
                    <Route path=StaticSegment("parceiros") view=AdminParceiros/>
                    <Route
                        path=(StaticSegment("parceiros"), StaticSegment("novo"))
                        view=AdminParceiroForm
                    />
                    <Route
                        path=(StaticSegment("parceiros"), ParamSegment("id"))
                        view=AdminParceiroForm
                    />
                    <Route path=StaticSegment("eventos") view=AdminEventos/>
                    <Route
                        path=(StaticSegment("eventos"), StaticSegment("novo"))
                        view=AdminEventoForm
                    />
                    <Route
                        path=(StaticSegment("eventos"), ParamSegment("id"))
                        view=AdminEventoForm
                    />
                    <Route
                        path=StaticSegment("configuracoes")
                        view=|| view! { <AdminEmBreve titulo="Configurações"/> }
                    />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

/// Layout do site público: cabeçalho + conteúdo (Outlet) + rodapé.
#[component]
fn PublicLayout() -> impl IntoView {
    view! {
        <a class="skip-link" href="#conteudo">"Pular para o conteúdo"</a>
        <SiteHeader/>
        <main id="conteudo">
            <Outlet/>
        </main>
        <SiteFooter/>
    }
}

/// Página 404 (com cabeçalho/rodapé do site).
#[component]
fn NotFound() -> impl IntoView {
    view! {
        <SiteHeader/>
        <main id="conteudo">
            <section class="container detalhe-status">
                <h1>"Página não encontrada"</h1>
                <a class="btn btn--primary" href="/">"Voltar ao início"</a>
            </section>
        </main>
        <SiteFooter/>
    }
}
