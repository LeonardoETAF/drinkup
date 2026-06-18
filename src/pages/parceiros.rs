use leptos::prelude::*;

use crate::api::parceiros::listar_parceiros;
use crate::components::product_card::CUP_SVG;
use crate::components::Seo;
use crate::domain::ParceiroPublico;

/// Cor da marca: a definida pelo parceiro ou uma cor padrão cíclica (tokens).
fn cor_marca(cor: &Option<String>, i: usize) -> String {
    cor.clone().filter(|c| !c.is_empty()).unwrap_or_else(|| {
        let token = ["lime", "pink", "cyan", "orange"][i % 4];
        format!("var(--color-{token})")
    })
}

/// Página pública de parceiros: showcase por marca (orientado a dados do painel).
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

        <Suspense fallback=move || {
            view! { <p class="catalog-status container">"Carregando parceiros..."</p> }
        }>
            {move || Suspend::new(async move {
                match parceiros.await {
                    Ok(itens) if itens.is_empty() => {
                        view! {
                            <p class="catalog-status container">
                                "Em breve anunciaremos nossos parceiros."
                            </p>
                        }
                            .into_any()
                    }
                    Ok(itens) => conteudo(itens).into_any(),
                    Err(_) => {
                        view! {
                            <p class="catalog-status container">
                                "Não foi possível carregar os parceiros."
                            </p>
                        }
                            .into_any()
                    }
                }
            })}
        </Suspense>
    }
}

fn conteudo(itens: Vec<ParceiroPublico>) -> AnyView {
    let logos = itens
        .iter()
        .filter_map(|p| {
            p.logo_url.clone().map(|url| {
                view! { <img class="parceiros-logos__img" src=url alt=p.nome.clone() loading="lazy"/> }
            })
        })
        .collect_view();

    let marcas = itens
        .into_iter()
        .enumerate()
        .map(|(i, p)| marca(i, p))
        .collect_view();

    view! {
        <div class="container parceiros-logos">{logos}</div>
        <div class="marcas">{marcas}</div>
    }
    .into_any()
}

fn marca(i: usize, p: ParceiroPublico) -> impl IntoView {
    let cor = cor_marca(&p.cor, i);
    let classe = if i % 2 == 1 {
        "marca marca--rev"
    } else {
        "marca"
    };

    let marca_visual = match p.logo_url.clone() {
        Some(url) => view! {
            <img class="marca__logo" src=url alt=p.nome.clone() loading="lazy"/>
        }
        .into_any(),
        None => view! { <span class="marca__nome">{p.nome.clone()}</span> }.into_any(),
    };

    // Lado direito: grade de produtos-exemplo (se houver) ou descrição + site.
    let lado = if p.itens.is_empty() {
        view! {
            <div class="marca__sobre">
                {p.descricao.clone().map(|d| view! { <p>{d}</p> })}
                {p
                    .site_url
                    .clone()
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
            </div>
        }
        .into_any()
    } else {
        let cards = p
            .itens
            .iter()
            .map(|nome| {
                view! {
                    <div class="marca-item">
                        <span
                            class="marca-item__cup"
                            style=format!("color:{cor}")
                            inner_html=CUP_SVG
                        ></span>
                        <span class="marca-item__nome">{nome.clone()}</span>
                    </div>
                }
            })
            .collect_view();
        view! { <div class="marca__grade">{cards}</div> }.into_any()
    };

    view! {
        <section class=classe>
            <div class="marca__brand" style=format!("background:{cor}")>
                {marca_visual}
                {p.tagline.map(|t| view! { <span class="marca__tag">{t}</span> })}
            </div>
            <div class="marca__lado">{lado}</div>
        </section>
    }
}
