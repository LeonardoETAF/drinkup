use leptos::prelude::*;

use crate::domain::Categoria;

/// Barra de filtros do catálogo: pills de categoria (links) + pills de
/// subcategoria da categoria ativa + busca ao vivo. O destaque (is-active) e a
/// linha de subcategorias reagem ao filtro atual, então a seleção acompanha a
/// navegação por SPA sem recarregar a página.
#[component]
pub fn FilterBar(
    categorias: Vec<Categoria>,
    ativa: Signal<Option<String>>,
    sub_ativa: Signal<Option<String>>,
    busca: RwSignal<String>,
) -> impl IntoView {
    let categorias = StoredValue::new(categorias);
    // Lista fixa de categorias principais (o destaque é que é reativo).
    let topo: Vec<Categoria> = categorias.with_value(|cs| {
        cs.iter()
            .filter(|c| c.parent_id.is_none())
            .cloned()
            .collect()
    });

    view! {
        <div class="filter-bar container">
            <nav class="filter-pills" aria-label="Categorias">
                <a
                    class="filter-pill"
                    class:is-active=move || ativa.get().is_none()
                    href="/produtos"
                >
                    "Todos"
                </a>
                {topo
                    .into_iter()
                    .map(|c| {
                        let slug = c.slug.clone();
                        let href = format!("/produtos?categoria={}", c.slug);
                        view! {
                            <a
                                class="filter-pill"
                                class:is-active=move || ativa.get().as_deref() == Some(slug.as_str())
                                href=href
                            >
                                {c.nome}
                            </a>
                        }
                    })
                    .collect_view()}
            </nav>

            {move || {
                // Subcategorias da categoria ativa — recomputadas a cada mudança
                // de filtro (categoria diferente => outra lista de subcategorias).
                let ativa_slug = ativa.get();
                let cat = categorias.with_value(|cs| {
                    ativa_slug.as_deref().and_then(|s| {
                        cs.iter().find(|c| c.parent_id.is_none() && c.slug == s).cloned()
                    })
                });
                let Some(cat) = cat else { return ().into_any() };
                let subs: Vec<Categoria> = categorias.with_value(|cs| {
                    cs.iter().filter(|c| c.parent_id == Some(cat.id)).cloned().collect()
                });
                if subs.is_empty() {
                    return ().into_any();
                }
                let cat_slug = cat.slug;
                let href_todos = format!("/produtos?categoria={cat_slug}");
                view! {
                    <nav class="filter-pills filter-pills--sub" aria-label="Subcategorias">
                        <a
                            class="filter-pill filter-pill--sub"
                            class:is-active=move || sub_ativa.get().is_none()
                            href=href_todos
                        >
                            "Todos"
                        </a>
                        {subs
                            .into_iter()
                            .map(|s| {
                                let sslug = s.slug.clone();
                                let href = format!(
                                    "/produtos?categoria={cat_slug}&sub={}",
                                    s.slug,
                                );
                                view! {
                                    <a
                                        class="filter-pill filter-pill--sub"
                                        class:is-active=move || {
                                            sub_ativa.get().as_deref() == Some(sslug.as_str())
                                        }
                                        href=href
                                    >
                                        {s.nome}
                                    </a>
                                }
                            })
                            .collect_view()}
                    </nav>
                }
                    .into_any()
            }}

            <div class="filter-search" role="search">
                <input
                    type="search"
                    placeholder="Buscar no catálogo..."
                    prop:value=move || busca.get()
                    on:input=move |ev| busca.set(event_target_value(&ev))
                    aria-label="Buscar produtos"
                />
            </div>
        </div>
    }
}
