use leptos::prelude::*;

use crate::domain::Categoria;

/// Barra de filtros do catálogo: pills de categoria (links) + busca ao vivo.
/// A busca filtra conforme o usuário digita; o servidor faz a filtragem real.
#[component]
pub fn FilterBar(
    categorias: Vec<Categoria>,
    ativa: Option<String>,
    busca: RwSignal<String>,
) -> impl IntoView {
    let is_all = ativa.is_none();

    view! {
        <div class="filter-bar container">
            <nav class="filter-pills" aria-label="Categorias">
                <a class="filter-pill" class:is-active=is_all href="/produtos">
                    "Todos"
                </a>
                {categorias
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
