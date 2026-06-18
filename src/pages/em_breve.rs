use leptos::prelude::*;

use crate::components::{PageHero, Seo};

/// Placeholder para páginas ainda não construídas (Quem Somos, Parceiros, etc.).
/// Mantém a navegação funcional; cada uma é implementada na sua fase.
#[component]
pub fn EmBrevePage(
    #[prop(into)] titulo: String,
    #[prop(into)] kicker: String,
    #[prop(into)] caminho: String,
) -> impl IntoView {
    let titulo_seo = titulo.clone();
    view! {
        <Seo
            titulo=titulo_seo
            descricao="DRINK UP — copos acrílicos personalizados para eventos, em Maringá-PR."
            caminho=caminho
        />
        <PageHero kicker=kicker titulo=titulo/>
        <section class="container em-breve">
            <p>"Esta página está em construção e chega em breve."</p>
            <a class="btn btn--primary" href="/">"Voltar ao início"</a>
        </section>
    }
}
