//! Componente raiz, shell HTML e roteador.
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    ParamSegment, SsrMode, StaticSegment,
};

use crate::admin::{
    AdminConfiguracoes, AdminConteudoHome, AdminConteudoQuemSomos, AdminDashboard, AdminEventoForm,
    AdminEventos, AdminLayout, AdminLeads, AdminLoginPage, AdminParceiroForm, AdminParceiros,
    AdminNovidades, AdminProdutoForm, AdminProdutos, AdminRecuperarSenhaPage,
    AdminRedefinirSenhaPage, AdminUsuarioForm, AdminUsuarios,
};
use crate::components::{SiteFooter, SiteHeader};
use crate::pages::{
    ContatoPage, HomePage, ParceirosPage, ProdutoPage, ProdutosPage, QuemSomosPage,
};

/// Script (inline) que aplica o tema salvo antes da primeira pintura, evitando
/// "flash" do tema padrão. Sem `<`/`>`/`&` para passar intacto no HTML.
const TEMA_INIT: &str = "(function(){try{if(localStorage.getItem('tema')==='light'){document.documentElement.setAttribute('data-theme','light')}}catch(e){}})()";

/// Documento HTML renderizado no servidor (SSR).
pub fn shell(options: LeptosOptions) -> impl IntoView {
    #[cfg(feature = "ssr")]
    let nonce = leptos::nonce::use_nonce().map(|n| n.to_string());
    #[cfg(not(feature = "ssr"))]
    let nonce: Option<String> = None;
    view! {
        <!DOCTYPE html>
        <html lang="pt-BR">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" href="/favicon.png?v=2"/>
                <script nonce=nonce>{TEMA_INIT}</script>
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
    definir_csp();

    view! {
        <Stylesheet id="leptos" href="/pkg/drinkup.css?v=50"/>
        <Title text="DRINK UP — Copos personalizados"/>
        <Router>
            <Routes fallback=NotFound>
                // Site público (com cabeçalho/rodapé).
                <ParentRoute path=StaticSegment("") view=PublicLayout>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("produtos") view=ProdutosPage/>
                    // SsrMode::Async: aguarda o produto no servidor antes do <head>,
                    // garantindo título/OG/canonical corretos no HTML (SEO e compartilhamento).
                    <Route
                        path=(StaticSegment("produtos"), ParamSegment("slug"))
                        view=ProdutoPage
                        ssr=SsrMode::Async
                    />
                    <Route path=StaticSegment("quem-somos") view=QuemSomosPage/>
                    <Route path=StaticSegment("parceiros") view=ParceirosPage/>
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
                <Route
                    path=(StaticSegment("admin"), StaticSegment("redefinir-senha"))
                    view=AdminRedefinirSenhaPage
                />
                <ParentRoute path=StaticSegment("admin") view=AdminLayout>
                    <Route path=StaticSegment("") view=AdminDashboard/>
                    <Route path=StaticSegment("leads") view=AdminLeads/>
                    <Route path=StaticSegment("novidades") view=AdminNovidades/>
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
                    <Route path=StaticSegment("configuracoes") view=AdminConfiguracoes/>
                    <Route path=StaticSegment("conteudo") view=AdminConteudoHome/>
                    <Route
                        path=StaticSegment("conteudo-quem-somos")
                        view=AdminConteudoQuemSomos
                    />
                    <Route path=StaticSegment("usuarios") view=AdminUsuarios/>
                    <Route
                        path=(StaticSegment("usuarios"), StaticSegment("novo"))
                        view=AdminUsuarioForm
                    />
                    <Route
                        path=(StaticSegment("usuarios"), ParamSegment("id"))
                        view=AdminUsuarioForm
                    />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

/// Define a Content-Security-Policy (baseada em nonce) na resposta — apenas em
/// SSR/release. Em dev é no-op para não interferir no live-reload do cargo-leptos.
/// O nonce é o mesmo que o Leptos injeta nos `<script>` da hidratação.
#[cfg(all(feature = "ssr", not(debug_assertions)))]
fn definir_csp() {
    use leptos::nonce::use_nonce;
    use leptos_axum::ResponseOptions;

    let Some(nonce) = use_nonce() else { return };
    let csp = format!(
        "default-src 'self'; base-uri 'self'; object-src 'none'; \
         frame-ancestors 'none'; form-action 'self'; img-src 'self' data:; \
         font-src 'self'; style-src 'self' 'unsafe-inline'; connect-src 'self'; \
         media-src 'self' https:; \
         frame-src https://www.youtube-nocookie.com https://www.youtube.com https://player.vimeo.com; \
         script-src 'self' 'wasm-unsafe-eval' 'nonce-{nonce}'"
    );
    if let Ok(valor) = axum::http::HeaderValue::from_str(&csp) {
        expect_context::<ResponseOptions>()
            .insert_header(axum::http::header::CONTENT_SECURITY_POLICY, valor);
    }
}

/// No-op em hidratação (cliente) e em dev.
#[cfg(not(all(feature = "ssr", not(debug_assertions))))]
fn definir_csp() {}

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
    definir_status_404();
    view! {
        <Title text="Página não encontrada | DRINK UP"/>
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

/// Define o status HTTP 404 na resposta (SSR) para não servir "soft 404".
#[cfg(feature = "ssr")]
fn definir_status_404() {
    if let Some(resp) = use_context::<leptos_axum::ResponseOptions>() {
        resp.set_status(axum::http::StatusCode::NOT_FOUND);
    }
}

/// No-op na hidratação (cliente).
#[cfg(not(feature = "ssr"))]
fn definir_status_404() {}
