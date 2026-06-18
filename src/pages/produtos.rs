use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::api::catalogo::{listar_categorias, listar_produtos};
use crate::components::{FilterBar, Pagination, ProductCard, Seo};
use crate::domain::FiltroProdutos;

/// Catálogo: filtros, busca e paginação resolvidos no servidor via query params.
#[component]
pub fn ProdutosPage() -> impl IntoView {
    let query = use_query_map();

    let filtro = Memo::new(move |_| {
        let q = query.read();
        let txt = |k: &str| q.get(k).map(|s| s.to_string()).filter(|s| !s.is_empty());
        FiltroProdutos {
            categoria_slug: txt("categoria"),
            material: txt("material"),
            cor: txt("cor"),
            busca: txt("busca"),
            pagina: q
                .get("pagina")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(1)
                .max(1),
            por_pagina: 12,
        }
    });

    let categorias = Resource::new(|| (), |_| async move { listar_categorias().await });
    let produtos = Resource::new(
        move || filtro.get(),
        |f| async move { listar_produtos(f).await },
    );

    let titulo = move || match filtro.read().busca.clone() {
        Some(b) => format!("\"{b}\""),
        None => "Todos".to_string(),
    };

    view! {
        <Seo
            titulo="Catálogo de copos personalizados"
            descricao="Long drinks, taças, calderetas e mais — copos acrílicos personalizáveis \
            para o seu evento. Veja o catálogo da DRINK UP e peça seu orçamento."
            caminho="/produtos"
        />
        <section class="page-hero">
            <div class="container">
                <span class="page-hero__kicker">"Catálogo"</span>
                <h1 class="page-hero__title">{titulo}</h1>
            </div>
        </section>

        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                let cats = categorias.await.unwrap_or_default();
                let f = filtro.get_untracked();
                view! { <FilterBar categorias=cats ativa=f.categoria_slug busca=f.busca/> }
            })}
        </Suspense>

        <section class="catalog container">
            <Suspense fallback=move || {
                view! { <p class="catalog-status">"Carregando produtos..."</p> }
            }>
                {move || Suspend::new(async move {
                    match produtos.await {
                        Ok(pagina) if pagina.itens.is_empty() => {
                            view! { <p class="catalog-status">"Nenhum produto encontrado."</p> }
                                .into_any()
                        }
                        Ok(pagina) => {
                            let f = filtro.get_untracked();
                            let (total, por, pg) = (pagina.total, pagina.por_pagina, pagina.pagina);
                            view! {
                                <div class="product-grid">
                                    {pagina
                                        .itens
                                        .into_iter()
                                        .map(|p| view! { <ProductCard produto=p/> })
                                        .collect_view()}
                                </div>
                                <Pagination filtro=f total=total por_pagina=por pagina=pg/>
                            }
                                .into_any()
                        }
                        Err(_) => {
                            view! {
                                <p class="catalog-status">
                                    "Não foi possível carregar os produtos."
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
