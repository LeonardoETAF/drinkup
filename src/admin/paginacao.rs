use leptos::prelude::*;

/// Paginação reutilizável do painel: "‹ Anterior · X de Y · Próximo ›".
/// Esconde-se com uma única página. `pagina` é 1-based e é atualizada nos
/// cliques; `total_paginas` é derivado do total de itens / itens por página.
#[component]
pub fn AdminPaginacao(pagina: RwSignal<u32>, total_paginas: Signal<u32>) -> impl IntoView {
    view! {
        {move || {
            let tp = total_paginas.get().max(1);
            (tp > 1)
                .then(|| {
                    view! {
                        <nav class="admin-paginacao" aria-label="Paginação">
                            <button
                                type="button"
                                class="btn btn--ghost btn--sm"
                                prop:disabled=move || pagina.get() <= 1
                                on:click=move |_| pagina.update(|p| *p = p.saturating_sub(1).max(1))
                            >
                                "‹ Anterior"
                            </button>
                            <span class="admin-paginacao__info">
                                {move || format!("{} de {tp}", pagina.get().min(tp).max(1))}
                            </span>
                            <button
                                type="button"
                                class="btn btn--ghost btn--sm"
                                prop:disabled=move || pagina.get() >= tp
                                on:click=move |_| {
                                    pagina.update(|p| {
                                        if *p < tp {
                                            *p += 1;
                                        }
                                    })
                                }
                            >
                                "Próximo ›"
                            </button>
                        </nav>
                    }
                })
        }}
    }
}
