use leptos::prelude::*;

/// Dashboard do painel (placeholder). As telas completas entram na Fase 7.
#[component]
pub fn AdminDashboard() -> impl IntoView {
    view! {
        <div class="admin-page">
            <h1 class="admin-page__title">"Dashboard"</h1>
            <p class="admin-page__muted">
                "Você está autenticado no painel. As telas completas (produtos, leads, \
                eventos, parceiros e configurações) chegam na Fase 7."
            </p>
        </div>
    }
}
