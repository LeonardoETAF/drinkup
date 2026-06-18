use leptos::prelude::*;

use crate::components::PageHero;

/// Placeholder para páginas ainda não construídas (Quem Somos, Parceiros, etc.).
/// Mantém a navegação funcional; cada uma é implementada na sua fase.
#[component]
pub fn EmBrevePage(#[prop(into)] titulo: String, #[prop(into)] kicker: String) -> impl IntoView {
    view! {
        <PageHero kicker=kicker titulo=titulo/>
        <section class="container em-breve">
            <p>"Esta página está em construção e chega em breve."</p>
            <a class="btn btn--primary" href="/">"Voltar ao início"</a>
        </section>
    }
}
