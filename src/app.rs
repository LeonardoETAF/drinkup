//! Componente raiz, shell HTML e roteador.
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    ParamSegment, SsrMode, StaticSegment,
};

use crate::admin::{
    AdminConfiguracoes, AdminConteudoHome, AdminConteudoQuemSomos, AdminDashboard, AdminEventoForm,
    AdminEventos, AdminLayout, AdminLeads, AdminLoginPage, AdminNovidades, AdminParceiroForm,
    AdminParceiros, AdminProdutoForm, AdminProdutos, AdminRecuperarSenhaPage,
    AdminRedefinirSenhaPage, AdminUsuarioForm, AdminUsuarios,
};
use crate::components::{AbrirConsentimento, BannerConsentimento, SiteFooter, SiteHeader};
use crate::pages::{
    ContatoPage, HomePage, ParceirosPage, PrivacidadePage, ProdutoPage, ProdutosPage,
    QuemSomosPage, TermosPage,
};

/// Script (inline) que aplica o tema salvo antes da primeira pintura, evitando
/// "flash" do tema padrão. Sem `<`/`>`/`&` para passar intacto no HTML.
const TEMA_INIT: &str = "(function(){try{if(localStorage.getItem('tema')==='light'){document.documentElement.setAttribute('data-theme','light')}}catch(e){}})()";

/// Loader do Google Tag Manager (snippet oficial), embrulhado numa função e
/// **não executado de imediato**: o `gtm.js` só é baixado se houver consentimento.
///
/// Fica no `<head>` (com nonce) para que quem já aceitou seja medido desde o
/// primeiro paint, sem esperar a hidratação do WASM. Quem não decidiu ou recusou
/// não recebe nenhuma requisição ao Google. O `BannerConsentimento` chama esta
/// mesma função ao aceitar — daí ela viver no `window`.
///
/// Injetado via `inner_html` para não sofrer escape de HTML.
const GTM_INIT: &str = "window.__drinkupGtm=function(){if(window.__drinkupGtmOn){return}\
window.__drinkupGtmOn=true;(function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':\
new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],\
j=d.createElement(s),dl=l!='dataLayer'?'\\u0026l='+l:'';j.async=true;j.src=\
'https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);\
})(window,document,'script','dataLayer','GTM-MTJ898HN')};\
try{if(localStorage.getItem('consentimento')==='aceito'){window.__drinkupGtm()}}catch(e){}";

/// O GTM só entra no site público: no painel, cada navegação viraria pageview no
/// Google (expondo URLs internas) e sujaria as métricas de visita da vitrine.
#[cfg(feature = "ssr")]
fn medir_pagina() -> bool {
    use_context::<axum::http::request::Parts>()
        .is_none_or(|partes| !partes.uri.path().starts_with("/admin"))
}

/// Em hidratação o `shell` não é renderizado; o valor é irrelevante.
#[cfg(not(feature = "ssr"))]
fn medir_pagina() -> bool {
    false
}

/// Documento HTML renderizado no servidor (SSR).
pub fn shell(options: LeptosOptions) -> impl IntoView {
    #[cfg(feature = "ssr")]
    let nonce = leptos::nonce::use_nonce().map(|n| n.to_string());
    #[cfg(not(feature = "ssr"))]
    let nonce: Option<String> = None;
    let medir = medir_pagina();
    view! {
        <!DOCTYPE html>
        <html lang="pt-BR">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" href="/favicon.png?v=2"/>
                <script nonce=nonce.clone()>{TEMA_INIT}</script>
                {medir.then(|| view! { <script nonce=nonce inner_html=GTM_INIT></script> })}
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            // Sem o `<noscript>` do GTM de propósito: sem JavaScript não há como
            // pedir nem registrar consentimento, então também não se rastreia.
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
    // Compartilhado entre o banner de cookies e o link "Cookies" do rodapé, que
    // reabre as preferências depois de o visitante já ter decidido.
    provide_context(AbrirConsentimento(RwSignal::new(false)));

    view! {
        <Stylesheet id="leptos" href="/pkg/drinkup.css?v=65"/>
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
                    <Route path=StaticSegment("termos-de-uso") view=TermosPage/>
                    <Route
                        path=StaticSegment("politica-de-privacidade")
                        view=PrivacidadePage
                    />
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
    // Google Tag Manager: o loader é inline (cobre o nonce), mas o gtm.js e as tags
    // que ele injeta vêm do googletagmanager.com e enviam dados ao Google Analytics.
    // `tagassistant.google.com` é só para o modo Visualizar/depurar do GTM: ele embute
    // a página (daí o frame-ancestors, que substitui o X-Frame-Options nos navegadores
    // que entendem CSP) e conversa com ela.
    let csp = format!(
        "default-src 'self'; base-uri 'self'; object-src 'none'; \
         frame-ancestors https://tagassistant.google.com; form-action 'self'; \
         img-src 'self' data: https://www.googletagmanager.com https://*.google-analytics.com; \
         font-src 'self'; style-src 'self' 'unsafe-inline'; \
         connect-src 'self' https://www.googletagmanager.com https://tagassistant.google.com https://*.google-analytics.com https://*.analytics.google.com; \
         media-src 'self' https:; \
         frame-src https://www.googletagmanager.com https://tagassistant.google.com https://www.youtube-nocookie.com https://www.youtube.com https://player.vimeo.com; \
         script-src 'self' 'wasm-unsafe-eval' 'nonce-{nonce}' https://www.googletagmanager.com"
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
        <BannerConsentimento/>
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
        <BannerConsentimento/>
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
