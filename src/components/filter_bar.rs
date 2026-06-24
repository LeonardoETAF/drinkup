use leptos::prelude::*;

use crate::domain::Categoria;

/// Barra de filtros do catálogo: pills de categoria (links) + pills de
/// subcategoria da categoria ativa + busca ao vivo. A busca filtra conforme o
/// usuário digita; o servidor faz a filtragem real.
#[component]
pub fn FilterBar(
    categorias: Vec<Categoria>,
    ativa: Option<String>,
    sub_ativa: Option<String>,
    busca: RwSignal<String>,
) -> impl IntoView {
    let is_all = ativa.is_none();
    let topo: Vec<Categoria> = categorias
        .iter()
        .filter(|c| c.parent_id.is_none())
        .cloned()
        .collect();
    // Categoria ativa (para localizar suas subcategorias).
    let cat_ativa = ativa
        .as_deref()
        .and_then(|s| topo.iter().find(|c| c.slug == s).cloned());
    let subs: Vec<Categoria> = cat_ativa
        .as_ref()
        .map(|ca| {
            categorias
                .iter()
                .filter(|c| c.parent_id == Some(ca.id))
                .cloned()
                .collect()
        })
        .unwrap_or_default();
    let cat_slug = cat_ativa.map(|c| c.slug);
    let sub_none = sub_ativa.is_none();

    view! {
        <div class="filter-bar container">
            <nav class="filter-pills" aria-label="Categorias">
                <a class="filter-pill" class:is-active=is_all href="/produtos">
                    "Todos"
                </a>
                {topo
                    .into_iter()
                    .map(|c| {
                        let active = ativa.as_deref() == Some(c.slug.as_str());
                        let href = format!("/produtos?categoria={}", c.slug);
                        view! {
                            <a class="filter-pill" class:is-active=active href=href>
                                {c.nome}
                            </a>
                        }
                    })
                    .collect_view()}
            </nav>

            {(!subs.is_empty())
                .then(|| {
                    let cat_slug = cat_slug.unwrap_or_default();
                    let href_todas = format!("/produtos?categoria={cat_slug}");
                    view! {
                        <nav class="filter-pills filter-pills--sub" aria-label="Subcategorias">
                            <a
                                class="filter-pill filter-pill--sub"
                                class:is-active=sub_none
                                href=href_todas
                            >
                                "Todas"
                            </a>
                            {subs
                                .into_iter()
                                .map(|s| {
                                    let active = sub_ativa.as_deref() == Some(s.slug.as_str());
                                    let href = format!(
                                        "/produtos?categoria={cat_slug}&sub={}",
                                        s.slug,
                                    );
                                    view! {
                                        <a
                                            class="filter-pill filter-pill--sub"
                                            class:is-active=active
                                            href=href
                                        >
                                            {s.nome}
                                        </a>
                                    }
                                })
                                .collect_view()}
                        </nav>
                    }
                })}

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
