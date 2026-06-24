use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::api::catalogo::{listar_categorias, listar_produtos};
use crate::components::{FilterBar, Pagination, ProductCard, Seo};
use crate::domain::FiltroProdutos;

/// Catálogo: filtros, busca e paginação resolvidos no servidor via query params.
#[component]
pub fn ProdutosPage() -> impl IntoView {
    let query = use_query_map();

    // Busca ao vivo: sinal no cliente (atualiza ao digitar). Categoria/página
    // continuam vindo da URL.
    let busca = RwSignal::new(
        query
            .read_untracked()
            .get("busca")
            .map(|s| s.to_string())
            .unwrap_or_default(),
    );

    let filtro = Memo::new(move |_| {
        let q = query.read();
        let txt = |k: &str| q.get(k).map(|s| s.to_string()).filter(|s| !s.is_empty());
        let b = busca.get();
        let b = b.trim();
        FiltroProdutos {
            categoria_slug: txt("categoria"),
            subcategoria_slug: txt("sub"),
            material: txt("material"),
            cor: txt("cor"),
            busca: (!b.is_empty()).then(|| b.to_string()),
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

    // Estado ativo reativo, para o destaque das pills acompanhar a navegação.
    let cat_ativa = Signal::derive(move || filtro.get().categoria_slug);
    let sub_ativa = Signal::derive(move || filtro.get().subcategoria_slug);

    // Título do catálogo: muda conforme a categoria selecionada (a busca não
    // altera o título). Sem categoria → "Todos".
    let titulo = move || match filtro.get().categoria_slug {
        None => "Todos".to_string(),
        Some(slug) => {
            let bonito = slug.replace('-', " ");
            categorias
                .get()
                .and_then(Result::ok)
                .and_then(|cats| cats.into_iter().find(|c| c.slug == slug).map(|c| c.nome))
                .unwrap_or(bonito)
        }
    };

    view! {
        <Seo
            titulo="Catálogo de copos personalizados"
            descricao="Long drinks, taças, calderetas e mais — copos acrílicos personalizáveis \
            para o seu evento. Veja o catálogo da DRINK UP e peça seu orçamento."
            caminho="/produtos"
        />
        <section class="page-hero page-hero--doodles">
            <div class="container">
                <span class="page-hero__kicker">"Catálogo"</span>
                <h1 class="page-hero__title">{titulo}</h1>
            </div>
        </section>

        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                let cats = categorias.await.unwrap_or_default();
                view! {
                    <FilterBar categorias=cats ativa=cat_ativa sub_ativa=sub_ativa busca=busca/>
                }
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
