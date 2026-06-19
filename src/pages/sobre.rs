use leptos::prelude::*;

use crate::api::quem_somos::obter_quem_somos;
use crate::components::Seo;
use crate::domain::QuemSomosConteudo;

const IC_PLAY: &str = r#"<svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M8 5v14l11-7z"/></svg>"#;

// Conteúdo padrão (fallback quando o painel ainda não foi preenchido).
const STAT_TITULO: &str = "+25 mil unidades";
const STAT_DESTAQUE: &str = "todos os dias";
const STAT_TEXTO: &str = "Uma equipe de mais de 20 profissionais alinhados em Direção, Vendas, Arte, \
    Personalização, Expedição e Injeção. Somos rápidos, sérios e apaixonados pelo que fazemos.";
const MISSAO: &str = "Nossa maior missão é brindar a vida por meio de momentos personalizados!";
const VISAO: &str = "Ter uma linha completa de produtos personalizáveis para clientes que buscam \
    inovação e querem guardar uma boa lembrança do seu momento ÉPICO!";
const VALORES: &str = "Sabemos que todos os produtos que oferecemos fazem parte de um momento \
    especial da vida de cada um — seja um casamento, aniversário ou até um brinde especial. Por \
    isso disponibilizamos o melhor atendimento do mercado, com total transparência e \
    comprometimento. Afinal, todos que fazem um produto personalizado querem deixar marcado um \
    momento único.";
const DEPO_TEXTO: &str = "Produtos de ótima qualidade, atendimento incomparável, entregas dentro \
    do prazo. Empresa séria e comprometida com seus clientes. Super recomendo!";
const DEPO_AUTOR: &str = "Dieferson Schaffer · Personalização Canábis";

/// Página institucional "Quem Somos" — conteúdo editável no painel.
#[component]
pub fn QuemSomosPage() -> impl IntoView {
    let conteudo = Resource::new(|| (), |_| async move { obter_quem_somos().await });

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

        <Suspense fallback=|| corpo(QuemSomosConteudo::default())>
            {move || Suspend::new(async move { corpo(conteudo.await.unwrap_or_default()) })}
        </Suspense>
    }
}

fn ou(valor: &str, padrao: &'static str) -> String {
    if valor.trim().is_empty() {
        padrao.to_string()
    } else {
        valor.to_string()
    }
}

/// Corpo da página (tudo abaixo do hero), montado a partir do conteúdo.
fn corpo(c: QuemSomosConteudo) -> AnyView {
    let depoimentos = if c.depoimentos.is_empty() {
        vec![(DEPO_TEXTO.to_string(), DEPO_AUTOR.to_string())]
    } else {
        c.depoimentos.clone()
    };

    view! {
        <section class="container sobre-stat">
            <p class="sobre-stat__num">{ou(&c.stat_titulo, STAT_TITULO)}</p>
            <p class="sobre-stat__label">{ou(&c.stat_destaque, STAT_DESTAQUE)}</p>
            <p class="sobre-stat__text">{ou(&c.stat_texto, STAT_TEXTO)}</p>
        </section>

        <section class="container">{video_view(&c.video_url)}</section>

        <section class="container sobre-mvv">
            <div class="sobre-mvv__col">
                <article class="mvv-card mvv-card--pink">
                    <h2 class="mvv-card__title">"Missão"</h2>
                    <p>{ou(&c.missao, MISSAO)}</p>
                </article>
                <article class="mvv-card mvv-card--dark">
                    <h2 class="mvv-card__title">"Visão"</h2>
                    <p>{ou(&c.visao, VISAO)}</p>
                </article>
            </div>
            <article class="mvv-card mvv-card--lime mvv-card--tall">
                <h2 class="mvv-card__title">"Valores"</h2>
                <p>{ou(&c.valores, VALORES)}</p>
            </article>
        </section>

        <section class="container sobre-fotos">
            {foto_view(&c.foto1_url, "Equipe de produção", "equipe de produção")}
            {foto_view(&c.foto2_url, "Vista interna da fábrica", "vista interna da fábrica")}
        </section>

        <section class="container sobre-depo">
            <h2 class="sobre-depo__title">"O que dizem nossos clientes"</h2>
            {depoimentos_view(depoimentos)}
        </section>
    }
    .into_any()
}

/// Embeda o vídeo na página: iframe para YouTube/Vimeo, `<video>` para arquivo
/// direto; sem link, mostra o placeholder.
fn video_view(url: &Option<String>) -> AnyView {
    let url = url.as_deref().map(str::trim).filter(|s| !s.is_empty());
    let Some(u) = url else {
        return view! {
            <div class="sobre-video" role="img" aria-label="Vídeo institucional da fábrica Drink Up">
                <span class="sobre-video__play" inner_html=IC_PLAY></span>
                <span class="sobre-video__cap">"[ vídeo institucional — fábrica Drink Up ]"</span>
            </div>
        }
        .into_any();
    };

    match url_embed(u) {
        Some(embed) => view! {
            <div class="sobre-video sobre-video--player">
                <iframe
                    class="sobre-video__frame"
                    src=embed
                    title="Vídeo institucional Drink Up"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                    allowfullscreen="true"
                ></iframe>
            </div>
        }
        .into_any(),
        None => view! {
            <div class="sobre-video sobre-video--player">
                <video class="sobre-video__frame" src=u.to_string() controls preload="metadata"></video>
            </div>
        }
        .into_any(),
    }
}

/// Converte URL de YouTube/Vimeo na URL de embed; `None` se não reconhecer.
fn url_embed(u: &str) -> Option<String> {
    if let Some(id) = youtube_id(u) {
        return Some(format!("https://www.youtube-nocookie.com/embed/{id}"));
    }
    if let Some(id) = vimeo_id(u) {
        return Some(format!("https://player.vimeo.com/video/{id}"));
    }
    None
}

fn so_id(s: &str) -> String {
    s.chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect()
}

fn youtube_id(u: &str) -> Option<String> {
    let marcas = ["v=", "youtu.be/", "/embed/", "/shorts/"];
    for m in marcas {
        if let Some(pos) = u.find(m) {
            let id = so_id(&u[pos + m.len()..]);
            if !id.is_empty() {
                return Some(id);
            }
        }
    }
    None
}

fn vimeo_id(u: &str) -> Option<String> {
    if !u.contains("vimeo.com") {
        return None;
    }
    let id: String = u
        .rsplit('/')
        .find(|seg| !seg.is_empty() && seg.chars().all(|c| c.is_ascii_digit()))
        .map(str::to_string)
        .unwrap_or_default();
    (!id.is_empty()).then_some(id)
}

fn foto_view(url: &Option<String>, label: &'static str, placeholder: &'static str) -> AnyView {
    match url {
        Some(u) => {
            let src = crate::components::responsiva(u).0;
            view! {
                <div
                    class="sobre-foto sobre-foto--img"
                    style=format!("background-image:url('{src}')")
                    role="img"
                    aria-label=label
                ></div>
            }
            .into_any()
        }
        None => view! {
            <div class="sobre-foto" role="img" aria-label=label>
                <span class="sobre-foto__cap">{format!("[ {placeholder} ]")}</span>
            </div>
        }
        .into_any(),
    }
}

fn depo_card(texto: String, autor: String) -> impl IntoView {
    view! {
        <blockquote class="depo-card">
            <p class="depo-card__quote">{format!("“{texto}”")}</p>
            <footer class="depo-card__autor">{autor}</footer>
        </blockquote>
    }
}

/// Um depoimento = card único centralizado; mais de um = swipe horizontal.
fn depoimentos_view(deps: Vec<(String, String)>) -> AnyView {
    if deps.len() == 1 {
        let (t, a) = deps.into_iter().next().unwrap();
        depo_card(t, a).into_any()
    } else {
        view! {
            <div class="sobre-depo__track" tabindex="0" aria-label="Depoimentos (arraste para ver mais)">
                {deps.into_iter().map(|(t, a)| depo_card(t, a)).collect_view()}
            </div>
        }
        .into_any()
    }
}
