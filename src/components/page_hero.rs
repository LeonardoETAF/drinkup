use leptos::prelude::*;

/// Banner de topo das páginas internas (faixa lima com kicker + título).
#[component]
pub fn PageHero(#[prop(into)] kicker: String, #[prop(into)] titulo: String) -> impl IntoView {
    view! {
        <section class="page-hero">
            <div class="container">
                <span class="page-hero__kicker">{kicker}</span>
                <h1 class="page-hero__title">{titulo}</h1>
            </div>
        </section>
    }
}
