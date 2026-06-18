use leptos::prelude::*;

/// Placeholder para seções do painel ainda não construídas (próximas partes da Fase 7).
#[component]
pub fn AdminEmBreve(#[prop(into)] titulo: String) -> impl IntoView {
    view! {
        <header class="admin-head">
            <h1 class="admin-head__title">{titulo}</h1>
        </header>
        <div class="admin-card admin-status">
            "Esta seção será construída nas próximas partes da Fase 7."
        </div>
    }
}
