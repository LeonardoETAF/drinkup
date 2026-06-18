use leptos::prelude::*;

use crate::domain::ProdutoResumo;

/// Ilustração de copo usada quando o produto não tem imagem.
pub(crate) const CUP_SVG: &str = r#"<svg viewBox="0 0 48 48" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M13 10h22l-2.2 27.5A4 4 0 0 1 28.8 41h-9.6a4 4 0 0 1-4-3.5z"/><path d="M11 10h26"/></svg>"#;

/// Card de produto (burro): recebe um `ProdutoResumo` e apenas renderiza.
#[component]
pub fn ProductCard(produto: ProdutoResumo) -> impl IntoView {
    let href = format!("/produtos/{}", produto.slug);
    let capacidade = produto.capacidade_ml.map(|ml| format!("{ml}ml"));
    let nome = produto.nome.clone();

    view! {
        <a class="product-card" href=href>
            <div class="product-card__media">
                {match produto.imagem_url {
                    Some(url) => view! {
                        <img class="product-card__img" src=url alt=nome loading="lazy"/>
                    }
                    .into_any(),
                    None => view! {
                        <span class="product-card__ph" inner_html=CUP_SVG></span>
                    }
                    .into_any(),
                }}
            </div>
            <div class="product-card__body">
                <h3 class="product-card__name">{produto.nome}</h3>
                {capacidade.map(|c| view! { <p class="product-card__meta">{c}</p> })}
                {produto.material.map(|m| view! { <p class="product-card__sub">{m}</p> })}
                <span class="btn btn--pink btn--block product-card__cta">"Ver mais"</span>
            </div>
        </a>
    }
}
