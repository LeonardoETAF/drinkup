use leptos::prelude::*;

use crate::components::Seo;

const IC_PLAY: &str = r#"<svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M8 5v14l11-7z"/></svg>"#;

/// Página institucional "Quem Somos" (conteúdo estático, conforme o material de design).
#[component]
pub fn QuemSomosPage() -> impl IntoView {
    view! {
        <Seo
            titulo="Quem Somos"
            descricao="A DRINK UP é especializada na fabricação de acrílicos personalizados — \
            copos, canecas, taças e mais. Fábrica própria em Maringá-PR e clientes por todo o Brasil."
            caminho="/quem-somos"
        />

        <section class="sobre-hero">
            <div class="container">
                <h1 class="sobre-hero__title">"Quem Somos"</h1>
                <p class="sobre-hero__text">
                    "A Drink Up é especializada na fabricação de acrílicos personalizados: copos, \
                    canecas, taças e muito mais. Com anos no mercado e uma equipe comprometida, \
                    conquistamos o carinho de clientes por todo o Brasil."
                </p>
            </div>
        </section>

        <section class="container sobre-stat">
            <p class="sobre-stat__num">"+25 mil unidades"</p>
            <p class="sobre-stat__label">"todos os dias"</p>
            <p class="sobre-stat__text">
                "Uma equipe de mais de 20 profissionais alinhados em Direção, Vendas, Arte, \
                Personalização, Expedição e Injeção. Somos rápidos, sérios e apaixonados pelo \
                que fazemos."
            </p>
        </section>

        <section class="container">
            <div class="sobre-video" role="img" aria-label="Vídeo institucional da fábrica Drink Up">
                <span class="sobre-video__play" inner_html=IC_PLAY></span>
                <span class="sobre-video__cap">"[ vídeo institucional — fábrica Drink Up ]"</span>
            </div>
        </section>

        <section class="container sobre-mvv">
            <div class="sobre-mvv__col">
                <article class="mvv-card mvv-card--pink">
                    <h2 class="mvv-card__title">"Missão"</h2>
                    <p>"Nossa maior missão é brindar a vida por meio de momentos personalizados!"</p>
                </article>
                <article class="mvv-card mvv-card--dark">
                    <h2 class="mvv-card__title">"Visão"</h2>
                    <p>
                        "Ter uma linha completa de produtos personalizáveis para clientes que \
                        buscam inovação e querem guardar uma boa lembrança do seu momento ÉPICO!"
                    </p>
                </article>
            </div>
            <article class="mvv-card mvv-card--lime mvv-card--tall">
                <h2 class="mvv-card__title">"Valores"</h2>
                <p>
                    "Sabemos que todos os produtos que oferecemos fazem parte de um momento \
                    especial da vida de cada um — seja um casamento, aniversário ou até um brinde \
                    especial. Por isso disponibilizamos o melhor atendimento do mercado, com total \
                    transparência e comprometimento. Afinal, todos que fazem um produto \
                    personalizado querem deixar marcado um momento único."
                </p>
            </article>
        </section>

        <section class="container sobre-fotos">
            <div class="sobre-foto" role="img" aria-label="Equipe de produção">
                <span class="sobre-foto__cap">"[ equipe de produção ]"</span>
            </div>
            <div class="sobre-foto" role="img" aria-label="Vista interna da fábrica">
                <span class="sobre-foto__cap">"[ vista interna da fábrica ]"</span>
            </div>
        </section>

        <section class="container sobre-depo">
            <h2 class="sobre-depo__title">"O que dizem nossos clientes"</h2>
            <blockquote class="depo-card">
                <p class="depo-card__quote">
                    "“Produtos de ótima qualidade, atendimento incomparável, entregas dentro do \
                    prazo. Empresa séria e comprometida com seus clientes. Super recomendo!”"
                </p>
                <footer class="depo-card__autor">
                    "Dieferson Schaffer · Personalização Canábis"
                </footer>
            </blockquote>
        </section>
    }
}
