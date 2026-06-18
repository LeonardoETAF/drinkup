use leptos::prelude::*;

use crate::domain::Categoria;

/// Barra de filtros do catálogo: pills de categoria (links) + busca (form GET).
/// Sem estado no cliente — tudo resolvido via query params no servidor.
#[component]
pub fn FilterBar(
    categorias: Vec<Categoria>,
    ativa: Option<String>,
    busca: Option<String>,
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
            <form class="filter-search" method="get" action="/produtos" role="search">
                <input
                    type="search"
                    name="busca"
                    placeholder="Buscar no catálogo..."
                    value=busca.unwrap_or_default()
                    aria-label="Buscar produtos"
                />
                <button type="submit" class="btn btn--ghost">
                    "Buscar"
                </button>
            </form>
        </div>
    }
}
