use leptos::prelude::*;

use crate::api::eventos::listar_eventos;
use crate::api::parceiros::listar_parceiros;
use crate::components::Seo;
use crate::domain::{EventoCarrossel, ParceiroPublico};

const IC_WHATS: &str = r#"<svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor" aria-hidden="true"><path d="M20 11.9a8 8 0 0 1-11.9 7L4 20l1.1-4A8 8 0 1 1 20 11.9zM12 5.5A6.4 6.4 0 0 0 6.6 15.5l-.6 2.2 2.3-.6A6.4 6.4 0 1 0 12 5.5zm3.6 8c-.2.5-1 .9-1.4 1-.4 0-.8.2-2.6-.6-2.2-1-3.6-3.2-3.7-3.4-.1-.2-.9-1.2-.9-2.2s.5-1.5.7-1.7c.2-.2.4-.2.5-.2h.4c.2 0 .3 0 .5.4l.6 1.5c0 .1 0 .3-.1.4l-.3.4-.3.3c-.1.1-.2.3-.1.5.1.2.6 1 1.3 1.6.9.8 1.6 1 1.8 1.1.2.1.3.1.5-.1l.6-.7c.2-.2.3-.2.5-.1l1.5.7c.2.1.3.2.4.3 0 .1 0 .6-.2 1z"/></svg>"#;
const IC_PREV: &str = r#"<svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M15 18l-6-6 6-6"/></svg>"#;
const IC_NEXT: &str = r#"<svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M9 18l6-6-6-6"/></svg>"#;

/// Passos do "Passo a passo" (número, título, descrição).
const PASSOS: [(&str, &str, &str); 4] = [
    (
        "01",
        "Escolha o produto",
        "Navegue pelo catálogo e encontre o copo ideal pelo estilo e tamanho certos para o seu evento.",
    ),
    (
        "02",
        "Envie sua arte",
        "Mande seu logo ou arte. Nossa equipe cria o layout perfeito, sem custo adicional.",
    ),
    (
        "03",
        "Aprovação",
        "Você aprova a arte final antes de produzirmos qualquer unidade — garantia total.",
    ),
    (
        "04",
        "Entrega rápida",
        "Produção ágil e entrega para todo o Brasil. Seu evento não vai esperar.",
    ),
];

/// Números da faixa de estatísticas (valor, rótulo).
const NUMEROS: [(&str, &str); 4] = [
    ("+500", "Clientes satisfeitos"),
    ("+25 mil", "Unidades por dia"),
    ("+4", "Anos no mercado"),
    ("+2 mil", "Eventos atendidos"),
];

/// Home da vitrine.
#[component]
pub fn HomePage() -> impl IntoView {
    let eventos = Resource::new(|| (), |_| async move { listar_eventos().await });
    let parceiros = Resource::new(|| (), |_| async move { listar_parceiros().await });

    view! {
        <Seo
            titulo="Copos personalizados para eventos — Maringá-PR"
            descricao="Copos acrílicos personalizados para formaturas, casamentos e eventos. \
            Fábrica própria em Maringá-PR. Sua marca em cada momento. Peça um orçamento."
            caminho="/"
        />

        <Hero/>
        <DoSeuJeito eventos/>
        <FaixaLogos parceiros/>
        <PassoAPasso/>
        <Numeros/>
        <Emocao/>
        <SuaMarca/>
        <Depoimentos/>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="container hero__inner">
                <div class="hero__conteudo">
                    <span class="hero__kicker">"● Maringá, PR · Fábrica própria"</span>
                    <h1 class="hero__title">"Perso"<span class="accent">"nalize"</span></h1>
                    <p class="hero__text">
                        "Copos acrílicos personalizados para formaturas, casamentos, eventos e \
                        muito mais. Sua marca em cada momento especial."
                    </p>
                    <div class="hero__cta">
                        <a href="/contato" class="btn btn--primary">
                            <span class="btn__ic" inner_html=IC_WHATS></span>
                            "Quero um orçamento"
                        </a>
                        <a href="/produtos" class="btn btn--link">"Ver catálogo →"</a>
                    </div>
                    <span class="hero__slogan">"Cheers to life"</span>
                </div>
                <span class="hero__mark" aria-hidden="true">
                    <span class="hero__slash hero__slash--lime"></span>
                    <span class="hero__slash hero__slash--white"></span>
                    <span class="hero__slash hero__slash--cyan"></span>
                </span>
            </div>
        </section>
    }
}

#[component]
fn DoSeuJeito(eventos: Resource<Result<Vec<EventoCarrossel>, ServerFnError>>) -> impl IntoView {
    let trilho = NodeRef::<leptos::html::Div>::new();
    let rolar = move |dx: f64| {
        let _ = dx;
        #[cfg(feature = "hydrate")]
        if let Some(el) = trilho.get() {
            el.scroll_by_with_x_and_y(dx, 0.0);
        }
    };

    view! {
        <section class="jeito">
            <div class="container jeito__head">
                <span class="kicker kicker--center">"Do seu jeito"</span>
                <h2 class="secao__titulo">"Para cada momento"</h2>
            </div>
            <div class="jeito__wrap container">
                <button
                    class="carrossel-seta carrossel-seta--prev"
                    aria-label="Anterior"
                    inner_html=IC_PREV
                    on:click=move |_| rolar(-320.0)
                ></button>
                <div class="jeito__trilho" node_ref=trilho>
                    <Suspense fallback=move || {
                        view! { <p class="catalog-status">"Carregando..."</p> }
                    }>
                        {move || Suspend::new(async move {
                            match eventos.await {
                                Ok(itens) if !itens.is_empty() => {
                                    itens.into_iter().map(carrossel_card).collect_view().into_any()
                                }
                                _ => ().into_any(),
                            }
                        })}
                    </Suspense>
                </div>
                <button
                    class="carrossel-seta carrossel-seta--next"
                    aria-label="Próximo"
                    inner_html=IC_NEXT
                    on:click=move |_| rolar(320.0)
                ></button>
            </div>
        </section>
    }
}

fn carrossel_card(e: EventoCarrossel) -> impl IntoView {
    let cor = e.cor.clone().unwrap_or_else(|| "#262626".to_string());
    let estilo = match &e.imagem_url {
        Some(u) => format!("background-image:url('{u}')"),
        None => format!("background:{cor}"),
    };
    view! {
        <a class="evt-card" href="/contato" style=estilo>
            <span class="evt-card__label" style=format!("--cor:{cor}")>{e.titulo}</span>
        </a>
    }
}

#[component]
fn FaixaLogos(parceiros: Resource<Result<Vec<ParceiroPublico>, ServerFnError>>) -> impl IntoView {
    view! {
        <section class="faixa-logos">
            <Suspense fallback=|| ()>
                {move || Suspend::new(async move {
                    match parceiros.await {
                        Ok(itens) if !itens.is_empty() => {
                            itens
                                .into_iter()
                                .map(|p| match p.logo_url {
                                    Some(url) => {
                                        view! {
                                            <img class="faixa-logos__img" src=url alt=p.nome loading="lazy"/>
                                        }
                                            .into_any()
                                    }
                                    None => {
                                        view! { <span class="faixa-logos__nome">{p.nome}</span> }
                                            .into_any()
                                    }
                                })
                                .collect_view()
                                .into_any()
                        }
                        _ => ().into_any(),
                    }
                })}
            </Suspense>
        </section>
    }
}

#[component]
fn PassoAPasso() -> impl IntoView {
    view! {
        <section class="passos container">
            <span class="kicker kicker--center">"Passo a passo"</span>
            <h2 class="secao__titulo">"Personalização"</h2>
            <div class="passos__grid">
                {PASSOS
                    .iter()
                    .map(|(num, titulo, texto)| {
                        view! {
                            <article class="passo">
                                <span class="passo__num">{*num}</span>
                                <h3 class="passo__titulo">{*titulo}</h3>
                                <p class="passo__texto">{*texto}</p>
                                <span class="passo__marca">{*num}</span>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
            <div class="passos__cta">
                <a class="btn btn--primary" href="/contato">"Quero um orçamento"</a>
            </div>
        </section>
    }
}

#[component]
fn Numeros() -> impl IntoView {
    view! {
        <section class="numeros">
            <div class="container numeros__grid">
                {NUMEROS
                    .iter()
                    .map(|(valor, rotulo)| {
                        view! {
                            <div class="numero">
                                <span class="numero__valor">{*valor}</span>
                                <span class="numero__rotulo">{*rotulo}</span>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}

#[component]
fn Emocao() -> impl IntoView {
    view! {
        <section class="emocao">
            <div class="container emocao__inner">
                <h2 class="emocao__titulo">
                    "Mais que um copo"<br/>
                    <span class="accent">"é reviver cada emoção"</span>
                </h2>
                <p class="emocao__texto">
                    "Cada copo carrega uma história. Sua marca permanente na memória de quem celebra."
                </p>
                <a class="btn btn--pink" href="/contato">"Quero um orçamento"</a>
            </div>
        </section>
    }
}

#[component]
fn SuaMarca() -> impl IntoView {
    view! {
        <section class="marca-bento container">
            <header class="marca-bento__head">
                <div>
                    <h2 class="secao__titulo secao__titulo--left">"Sua marca"</h2>
                    <p class="marca-bento__sub">"No olhar e na memória"</p>
                </div>
                <a class="btn btn--ghost" href="/produtos">"Personalize agora"</a>
            </header>

            <div class="bento">
                <div class="bento__foto bento__foto--grande" role="img" aria-label="Growler personalizado">
                    <span class="bento__cap">"[ growler personalizado ]"</span>
                </div>
                <div class="bento__stat bento__stat--lime">
                    <span class="bento__num">"+25K"</span>
                    <span class="bento__lbl">"Unidades por dia"</span>
                </div>
                <div class="bento__foto bento__foto--orange" role="img" aria-label="Caneca personalizada">
                    <span class="bento__cap">"[ caneca personalizada ]"</span>
                </div>
                <div class="bento__stat bento__stat--cyan">
                    <span class="bento__num">"+4"</span>
                    <span class="bento__lbl">"Anos no mercado"</span>
                </div>
                <div class="bento__stat bento__stat--pink">
                    <span class="bento__num">"+500"</span>
                    <span class="bento__lbl">"Clientes satisfeitos"</span>
                </div>
                <div class="bento__stat bento__stat--dark">
                    <span class="bento__num">"+2K"</span>
                    <span class="bento__lbl">"Eventos atendidos"</span>
                </div>
                <div class="bento__stat bento__stat--dark">
                    <span class="bento__num">"100%"</span>
                    <span class="bento__lbl">"Personalizável"</span>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Depoimentos() -> impl IntoView {
    view! {
        <section class="container depo-secao">
            <span class="kicker kicker--center">"Satisfação"</span>
            <h2 class="secao__titulo">"Que se compartilha"</h2>
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
