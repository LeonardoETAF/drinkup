use leptos::prelude::*;

use crate::components::product_card::CUP_SVG;
use crate::domain::ProdutoImagem;

/// Galeria de imagens do produto: swipe lateral (uma por vez) com bolinhas.
#[component]
pub fn Gallery(imagens: Vec<ProdutoImagem>, nome: String) -> impl IntoView {
    if imagens.is_empty() {
        return view! {
            <div class="gallery">
                <div class="gallery__slide gallery__slide--ph">
                    <span class="gallery__ph" inner_html=CUP_SVG></span>
                </div>
            </div>
        }
        .into_any();
    }

    let total = imagens.len();
    let ativo = RwSignal::new(0usize);
    let track = NodeRef::<leptos::html::Div>::new();

    let slides = imagens
        .iter()
        .map(|im| {
            let alt = im.alt.clone().unwrap_or_else(|| nome.clone());
            let (src, srcset) = crate::components::responsiva(&im.url);
            view! {
                <div class="gallery__slide">
                    <img
                        class="gallery__img"
                        src=src
                        srcset=srcset
                        sizes="(max-width: 900px) 100vw, 600px"
                        alt=alt
                    />
                </div>
            }
        })
        .collect_view();

    view! {
        <div class="gallery">
            <div
                class="gallery__track"
                node_ref=track
                on:scroll=move |_| {
                    #[cfg(feature = "hydrate")]
                    if let Some(el) = track.get_untracked() {
                        let w = el.client_width().max(1);
                        ativo.set((f64::from(el.scroll_left()) / f64::from(w)).round() as usize);
                    }
                }
            >
                {slides}
            </div>
            {(total > 1)
                .then(|| {
                    view! {
                        <div class="gallery__dots" aria-label="Imagens do produto">
                            {(0..total)
                                .map(|i| {
                                    view! {
                                        <button
                                            type="button"
                                            class="gallery__dot"
                                            class:is-active=move || ativo.get() == i
                                            aria-label=format!("Ver imagem {}", i + 1)
                                            on:click=move |_| {
                                                ativo.set(i);
                                                #[cfg(feature = "hydrate")]
                                                if let Some(el) = track.get_untracked() {
                                                    el.set_scroll_left(el.client_width() * i as i32);
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
    .into_any()
}
