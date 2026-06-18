use leptos::prelude::*;

use crate::domain::FiltroProdutos;

/// Monta a URL do catálogo preservando os filtros e trocando a página.
fn url(filtro: &FiltroProdutos, pagina: u32) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(c) = &filtro.categoria_slug {
        parts.push(format!("categoria={}", urlencoding::encode(c)));
    }
    if let Some(m) = &filtro.material {
        parts.push(format!("material={}", urlencoding::encode(m)));
    }
    if let Some(c) = &filtro.cor {
        parts.push(format!("cor={}", urlencoding::encode(c)));
    }
    if let Some(b) = &filtro.busca {
        parts.push(format!("busca={}", urlencoding::encode(b)));
    }
    parts.push(format!("pagina={pagina}"));
    format!("/produtos?{}", parts.join("&"))
}

/// Paginação por links (funciona sem JS). Esconde-se quando há uma só página.
#[component]
pub fn Pagination(
    filtro: FiltroProdutos,
    total: i64,
    por_pagina: u32,
    pagina: u32,
) -> impl IntoView {
    let por = por_pagina.max(1);
    let total_pages = ((total as u64).div_ceil(u64::from(por)) as u32).max(1);

    if total_pages <= 1 {
        return ().into_any();
    }

    let atual = pagina.clamp(1, total_pages);
    let prev = (atual > 1).then(|| url(&filtro, atual - 1));
    let next = (atual < total_pages).then(|| url(&filtro, atual + 1));

    let paginas = (1..=total_pages)
        .map(|p| {
            let href = url(&filtro, p);
            view! {
                <a class="page-link" class:is-active=p == atual href=href>
                    {p.to_string()}
                </a>
            }
        })
        .collect_view();

    view! {
        <nav class="pagination" aria-label="Paginação">
            {prev
                .map(|h| view! { <a class="page-link page-link--nav" href=h>"‹"</a> })}
            {paginas}
            {next
                .map(|h| view! { <a class="page-link page-link--nav" href=h>"›"</a> })}
        </nav>
    }
    .into_any()
}
