use leptos::prelude::*;

use crate::api::parceiros::listar_parceiros;
use crate::components::Seo;
use crate::domain::ParceiroPublico;

/// Iniciais para o "logo textual" quando o parceiro não tem imagem.
fn iniciais(nome: &str) -> String {
    nome.split_whitespace()
        .filter_map(|p| p.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

/// Acento de cor cíclico (ecoa os painéis coloridos do material de design).
fn acento(i: usize) -> &'static str {
    ["lime", "pink", "cyan", "orange"][i % 4]
}

/// Página pública de parceiros: orientada a dados (gerida no painel).
#[component]
pub fn ParceirosPage() -> impl IntoView {
    let parceiros = Resource::new(|| (), |_| async move { listar_parceiros().await });

    view! {
        <Seo
            titulo="Parceiros"
            descricao="Grandes marcas confiam na DRINK UP para tornar seus momentos memoráveis. \
            Conheça os parceiros que personalizam seus copos acrílicos com a gente."
            caminho="/parceiros"
        />

        <section class="page-hero page-hero--center">
            <div class="container">
                <span class="page-hero__kicker">"Confiança mútua"</span>
                <h1 class="page-hero__title">"Parceiros"</h1>
                <p class="page-hero__sub">
                    "Grandes marcas que confiam na DRINK UP para tornar seus momentos memoráveis."
                </p>
            </div>
        </section>

        <section class="container parceiros">
            <Suspense fallback=move || {
                view! { <p class="catalog-status">"Carregando parceiros..."</p> }
            }>
                {move || Suspend::new(async move {
                    match parceiros.await {
                        Ok(itens) if itens.is_empty() => {
                            view! {
                                <p class="catalog-status">
                                    "Em breve anunciaremos nossos parceiros."
                                </p>
                            }
                                .into_any()
                        }
                        Ok(itens) => grade(itens).into_any(),
                        Err(_) => {
                            view! {
                                <p class="catalog-status">
                                    "Não foi possível carregar os parceiros."
                                </p>
                            }
                                .into_any()
                        }
                    }
                })}
            </Suspense>
        </section>
    }
}

fn grade(itens: Vec<ParceiroPublico>) -> AnyView {
    view! {
        <div class="parceiros-grid">
            {itens
                .into_iter()
                .enumerate()
                .map(|(i, p)| cartao(i, p))
                .collect_view()}
        </div>
    }
    .into_any()
}

fn cartao(i: usize, p: ParceiroPublico) -> impl IntoView {
    let classe = format!("parceiro-card parceiro-card--{}", acento(i));
    let logo = match p.logo_url {
        Some(url) => view! {
            <img class="parceiro-card__logo" src=url alt=p.nome.clone() loading="lazy" decoding="async"/>
        }
        .into_any(),
        None => view! { <span class="parceiro-card__iniciais">{iniciais(&p.nome)}</span> }.into_any(),
    };

    view! {
        <article class=classe>
            <div class="parceiro-card__media">{logo}</div>
            <h2 class="parceiro-card__nome">{p.nome}</h2>
            {p.descricao.map(|d| view! { <p class="parceiro-card__desc">{d}</p> })}
            {p
                .site_url
                .map(|url| {
                    view! {
                        <a
                            class="parceiro-card__link"
                            href=url
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            "Visitar site ›"
                        </a>
                    }
                })}
        </article>
    }
}
