use leptos::prelude::*;

use crate::api::parceiros::listar_parceiros;
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
    // Cada logo é um atalho que rola até a seção do parceiro correspondente.
    let logos = itens
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            p.logo_url.clone().map(|url| {
                view! {
                    <a
                        class="parceiros-logos__item"
                        href=format!("#parceiro-{i}")
                        aria-label=format!("Ir para {}", p.nome)
                    >
                        <img
                            class="parceiros-logos__img"
                            src=url
                            alt=p.nome.clone()
                            loading="lazy"
                        />
                    </a>
                }
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
        view! { <MarcaSwipe imagens=p.itens.clone()/> }.into_any()
    };

    view! {
        <section class=classe id=format!("parceiro-{i}")>
            <div class="marca__brand" style=format!("background:{cor}")>
                {marca_visual}
                {p.tagline.map(|t| view! { <span class="marca__tag">{t}</span> })}
            </div>
            <div class="marca__lado">{lado}</div>
        </section>
    }
}

/// Swipe das imagens de produtos do parceiro (uma por slide, sem legenda) com
/// bolinhas de navegação.
#[component]
fn MarcaSwipe(imagens: Vec<String>) -> impl IntoView {
    let total = imagens.len();
    let ativo = RwSignal::new(0usize);
    let track = NodeRef::<leptos::html::Div>::new();

    view! {
        <div class="marca-swipe">
            <div
                class="marca-swipe__track"
                node_ref=track
                on:scroll=move |_| {
                    #[cfg(feature = "hydrate")]
                    if let Some(el) = track.get_untracked() {
                        let passo = (f64::from(el.scroll_width()) / total.max(1) as f64).max(1.0);
                        ativo.set((f64::from(el.scroll_left()) / passo).round() as usize);
                    }
                }
            >
                {imagens
                    .into_iter()
                    .map(|url| {
                        view! {
                            <div class="marca-swipe__item">
                                <img class="marca-swipe__img" src=url alt="" loading="lazy"/>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
            {(total > 1)
                .then(|| {
                    view! {
                        <div class="marca-swipe__dots" aria-label="Imagens do parceiro">
                            {(0..total)
                                .map(|i| {
                                    view! {
                                        <button
                                            type="button"
                                            class="marca-swipe__dot"
                                            class:is-active=move || ativo.get() == i
                                            aria-label=format!("Imagem {}", i + 1)
                                            on:click=move |_| {
                                                ativo.set(i);
                                                #[cfg(feature = "hydrate")]
                                                if let Some(el) = track.get_untracked() {
                                                    let passo = f64::from(el.scroll_width())
                                                        / total.max(1) as f64;
                                                    el.set_scroll_left((passo * i as f64) as i32);
                                                }
                                            }
                                        ></button>
                                    }
                                })
                                .collect_view()}
                        </div>
                    }
                })}
        </div>
    }
}
