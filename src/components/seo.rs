use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

/// Domínio padrão (produção), usado quando `DRINKUP_SITE_URL` não é definido.
const DEFAULT_SITE_URL: &str = "https://drinkup.com.br";

/// URL base do site (canonical, Open Graph, sitemap, links de e-mail). Sem barra
/// final. Definida em tempo de **build** por `DRINKUP_SITE_URL` (assim o valor é
/// idêntico no servidor e no cliente — pré-requisito da hidratação); na ausência
/// dela, cai no domínio de produção. Não é segredo: é um domínio público.
pub const SITE_URL: &str = resolver_site_url();

/// Resolve a URL base em tempo de compilação (env de build, com fallback).
/// `DRINKUP_SITE_URL` vazia (ARG de build não informado) também cai no padrão.
const fn resolver_site_url() -> &'static str {
    match option_env!("DRINKUP_SITE_URL") {
        Some(v) => {
            if v.is_empty() {
                DEFAULT_SITE_URL
            } else {
                v
            }
        }
        None => DEFAULT_SITE_URL,
    }
}
const SITE_NAME: &str = "DRINK UP";
/// Imagem padrão de compartilhamento (caminho relativo a `SITE_URL`).
const OG_IMAGE: &str = "/brand/logo-branca.png";

/// Metadados de SEO de uma página: `<title>`, descrição, canonical e tags
/// Open Graph/Twitter. Renderizado no `<head>` durante o SSR.
#[component]
pub fn Seo(
    /// Título da página (sem o sufixo da marca).
    #[prop(into)]
    titulo: String,
    /// Descrição para resultados de busca e compartilhamento.
    #[prop(into)]
    descricao: String,
    /// Caminho da página (ex.: "/produtos"), usado em canonical e og:url.
    #[prop(into)]
    caminho: String,
    /// Imagem de compartilhamento (caminho relativo). Default: logo.
    #[prop(optional_no_strip)]
    imagem: Option<String>,
) -> impl IntoView {
    let titulo_completo = if titulo == SITE_NAME {
        titulo
    } else {
        format!("{titulo} | {SITE_NAME}")
    };
    let url = format!("{SITE_URL}{caminho}");
    let imagem = format!("{SITE_URL}{}", imagem.as_deref().unwrap_or(OG_IMAGE));

    view! {
        <Title text=titulo_completo.clone()/>
        <Meta name="description" content=descricao.clone()/>
        <Link rel="canonical" href=url.clone()/>

        <Meta property="og:type" content="website"/>
        <Meta property="og:site_name" content=SITE_NAME/>
        <Meta property="og:locale" content="pt_BR"/>
        <Meta property="og:title" content=titulo_completo.clone()/>
        <Meta property="og:description" content=descricao.clone()/>
        <Meta property="og:url" content=url/>
        <Meta property="og:image" content=imagem.clone()/>

        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=titulo_completo/>
        <Meta name="twitter:description" content=descricao/>
        <Meta name="twitter:image" content=imagem/>
    }
}
