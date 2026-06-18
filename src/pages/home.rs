use leptos::prelude::*;

use crate::api::catalogo::listar_produtos;
use crate::components::{ProductCard, Seo};
use crate::domain::FiltroProdutos;

/// Home da vitrine: hero + produtos em destaque + chamada para orçamento.
#[component]
pub fn HomePage() -> impl IntoView {
    let destaques = Resource::new(
        || (),
        |_| async move {
            listar_produtos(FiltroProdutos {
                pagina: 1,
                por_pagina: 4,
                ..Default::default()
            })
            .await
        },
    );

    view! {
        <Seo
            titulo="Copos personalizados para eventos — Maringá-PR"
            descricao="Copos acrílicos personalizados para formaturas, casamentos e eventos. \
            Fábrica própria em Maringá-PR. Sua marca em cada momento. Peça um orçamento."
            caminho="/"
        />
        <section class="hero">
            <div class="container hero__inner">
                <span class="kicker">"Maringá, PR · Fábrica própria"</span>
                <h1 class="hero__title">"Perso"<span class="accent">"nalize"</span></h1>
                <p class="hero__text">
                    "Copos acrílicos personalizados para formaturas, casamentos, eventos e \
                    muito mais. Sua marca em cada momento especial."
                </p>
                <div class="hero__cta">
                    <a href="/contato" class="btn btn--primary">"Quero um orçamento"</a>
                    <a href="/produtos" class="btn btn--ghost">"Ver catálogo"</a>
                </div>
                <span class="hero__slogan">"Cheers to life"</span>
            </div>
        </section>

        <section class="destaques container">
            <span class="kicker">"Do seu jeito"</span>
            <h2 class="destaques__title">"Produtos em destaque"</h2>
            <Suspense fallback=move || {
                view! { <p class="catalog-status">"Carregando..."</p> }
            }>
                {move || Suspend::new(async move {
                    match destaques.await {
                        Ok(pagina) if !pagina.itens.is_empty() => {
                            view! {
                                <div class="product-grid">
                                    {pagina
                                        .itens
                                        .into_iter()
                                        .map(|p| view! { <ProductCard produto=p/> })
                                        .collect_view()}
                                </div>
                            }
                                .into_any()
                        }
                        _ => view! { <p class="catalog-status">"Catálogo em breve."</p> }.into_any(),
                    }
                })}
            </Suspense>
            <div class="destaques__cta">
                <a class="btn btn--primary" href="/produtos">"Ver catálogo completo"</a>
            </div>
        </section>

        <section class="cta-band">
            <div class="container cta-band__inner">
                <h2 class="cta-band__title">"Sua marca em cada brinde"</h2>
                <p>"Peça um orçamento e personalize seus copos para o seu evento."</p>
                <a class="btn btn--dark" href="/contato">"Quero um orçamento"</a>
            </div>
        </section>
    }
}
