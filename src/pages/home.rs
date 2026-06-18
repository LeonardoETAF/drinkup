use leptos::prelude::*;

use crate::api::eventos::listar_eventos;
use crate::api::parceiros::listar_parceiros;
use crate::components::Seo;
use crate::domain::{EventoCarrossel, ParceiroPublico};

const IC_WHATS: &str = r#"<svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M.057 24l1.687-6.163a11.867 11.867 0 0 1-1.587-5.946C.16 5.335 5.495 0 12.05 0a11.821 11.821 0 0 1 8.413 3.488 11.824 11.824 0 0 1 3.48 8.414c-.003 6.557-5.338 11.892-11.893 11.892a11.9 11.9 0 0 1-5.688-1.448L.057 24zm6.597-3.807c1.676.995 3.276 1.591 5.392 1.592 5.448 0 9.886-4.434 9.889-9.885.002-5.462-4.415-9.89-9.881-9.892-5.452 0-9.887 4.434-9.889 9.884a9.86 9.86 0 0 0 1.51 5.26l-.999 3.648 3.742-.981zm11.387-5.464c-.074-.124-.272-.198-.57-.347-.297-.149-1.758-.868-2.031-.967-.272-.099-.47-.149-.669.149-.198.297-.768.967-.941 1.165-.173.198-.347.223-.644.074-.297-.149-1.255-.462-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.297-.347.446-.521.151-.172.2-.296.3-.495.099-.198.05-.372-.025-.521-.075-.148-.669-1.611-.916-2.206-.242-.579-.487-.501-.669-.51l-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.626.712.226 1.36.194 1.872.118.571-.085 1.758-.719 2.006-1.413.248-.695.248-1.29.173-1.414z"/></svg>"#;
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
                    <img
                        class="hero__slogan"
                        src="/brand/cheers-to-life.png"
                        alt="Cheers to life"
                        loading="lazy"
                    />
                </div>
                <img class="hero__mark" src="/brand/logo-mark.png" alt="" aria-hidden="true"/>
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
        Some(u) => format!("background-image:url('{}')", crate::components::responsiva(u).0),
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
