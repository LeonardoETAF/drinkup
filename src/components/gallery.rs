use leptos::prelude::*;

use crate::components::product_card::CUP_SVG;
use crate::domain::ProdutoImagem;

/// Galeria de imagens do produto: imagem principal + miniaturas (interativo).
#[component]
pub fn Gallery(imagens: Vec<ProdutoImagem>, nome: String) -> impl IntoView {
    if imagens.is_empty() {
        return view! {
            <div class="gallery">
                <div class="gallery__main gallery__main--ph">
                    <span class="gallery__ph" inner_html=CUP_SVG></span>
                </div>
            </div>
        }
        .into_any();
    }

    let (sel, set_sel) = signal(0usize);
    let imgs = StoredValue::new(imagens.clone());
    let nome_alt = nome.clone();

    let principal = move || {
        let i = sel.get();
        imgs.with_value(|v| {
            v.get(i).or_else(|| v.first()).map(|im| {
                let alt = im.alt.clone().unwrap_or_else(|| nome_alt.clone());
                let (src, srcset) = crate::components::responsiva(&im.url);
                view! {
                    <img
                        class="gallery__img"
                        src=src
                        srcset=srcset
                        sizes="(max-width: 900px) 100vw, 600px"
                        alt=alt
                    />
                }
            })
        })
    };

    let thumbs = imagens
        .iter()
        .enumerate()
        .map(|(i, im)| {
            let src = crate::components::responsiva(&im.url).0;
            view! {
                <button
                    type="button"
                    class="gallery__thumb"
                    class:is-active=move || sel.get() == i
                    on:click=move |_| set_sel.set(i)
                    aria-label=format!("Ver imagem {}", i + 1)
                >
                    <img src=src alt="" loading="lazy"/>
                </button>
            }
        })
        .collect_view();

    view! {
        <div class="gallery">
            <div class="gallery__main">{principal}</div>
            <div class="gallery__thumbs">{thumbs}</div>
        </div>
    }
    .into_any()
}
