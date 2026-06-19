use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::api::catalogo::obter_produto;
use crate::api::config::obter_contato;
use crate::components::{Gallery, Seo};
use crate::domain::{link_whatsapp, ProdutoDetalhe};

/// Página de detalhe do produto.
#[component]
pub fn ProdutoPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || {
        params
            .read()
            .get("slug")
            .map(|s| s.to_string())
            .unwrap_or_default()
    };
    // Blocking: o SSR aguarda o produto para que `<title>`/meta entrem no <head>.
    let produto = Resource::new_blocking(slug, |s| async move { obter_produto(s).await });

    view! {
        <Suspense fallback=move || {
            view! { <div class="container detalhe-status">"Carregando..."</div> }
        }>
            {move || Suspend::new(async move {
                match produto.await {
                    Ok(Some(p)) => view! { <DetalheProduto produto=p/> }.into_any(),
                    Ok(None) => {
                        view! {
                            <div class="container detalhe-status">
                                <h1>"Produto não encontrado"</h1>
                                <a class="btn btn--ghost" href="/produtos">"Voltar ao catálogo"</a>
                            </div>
                        }
                            .into_any()
                    }
                    Err(_) => {
                        view! {
                            <div class="container detalhe-status">
                                "Não foi possível carregar o produto."
                            </div>
                        }
                            .into_any()
                    }
                }
            })}
        </Suspense>
    }
}

#[component]
fn DetalheProduto(produto: ProdutoDetalhe) -> impl IntoView {
    let categoria = produto
        .categoria_nome
        .clone()
        .unwrap_or_else(|| "Catálogo".to_string());

    // Botão de orçamento abre o WhatsApp cadastrado com mensagem do produto
    // (fallback: página de contato com o produto pré-selecionado).
    let info = Resource::new(|| (), |_| async move { obter_contato().await });
    let nome_cta = produto.nome.clone();
    let id_cta = produto.id;
    let cta = move || {
        let msg = format!("Olá! Tenho interesse no produto {nome_cta}.");
        match info
            .get()
            .and_then(Result::ok)
            .and_then(|c| link_whatsapp(&c.telefone))
        {
            Some(wa) => format!("{wa}?text={}", urlencoding::encode(&msg)),
            None => format!(
                "/contato?produto={}&pid={}",
                urlencoding::encode(&nome_cta),
                id_cta
            ),
        }
    };

    // SEO: computado antes de mover `imagens`/`descricao` para o restante da view.
    let seo_titulo = produto.nome.clone();
    let seo_caminho = format!("/produtos/{}", produto.slug);
    let seo_imagem = produto
        .imagens
        .first()
        .map(|i| crate::components::responsiva(&i.url).0);
    let seo_descricao = produto.descricao.clone().unwrap_or_else(|| {
        format!(
            "{} — copo acrílico personalizável da DRINK UP. Peça seu orçamento.",
            produto.nome
        )
    });

    let atributos: Vec<(&'static str, String)> = [
        produto
            .capacidade_ml
            .map(|v| ("Capacidade", format!("{v} ml"))),
        produto.material.clone().map(|v| ("Material", v)),
        produto.cor.clone().map(|v| ("Cores", v)),
        produto.altura_mm.map(|v| ("Altura", format!("{v} mm"))),
        produto.diametro_mm.map(|v| ("Diâmetro", format!("{v} mm"))),
        Some((
            "Personalizável",
            if produto.personalizavel {
                "Sim"
            } else {
                "Não"
            }
            .to_string(),
        )),
    ]
    .into_iter()
    .flatten()
    .collect();

    view! {
        <Seo
            titulo=seo_titulo
            descricao=seo_descricao
            caminho=seo_caminho
            imagem=seo_imagem
        />
        <nav class="breadcrumb container" aria-label="Trilha de navegação">
            <a href="/">"Início"</a>
            <span aria-hidden="true">"/"</span>
            <a href="/produtos">{categoria.clone()}</a>
            <span aria-hidden="true">"/"</span>
            <span class="breadcrumb__current">{produto.nome.clone()}</span>
        </nav>

        <section class="detalhe container">
            <Gallery imagens=produto.imagens nome=produto.nome.clone()/>

            <h2 class="detalhe__heading">"Detalhes do produto"</h2>

            <div class="detalhe__card">
                <span class="kicker">{categoria}</span>
                <h1 class="detalhe__title">{produto.nome.clone()}</h1>
                {produto.descricao.map(|d| view! { <p class="detalhe__desc">{d}</p> })}

                <dl class="atributos">
                    {atributos
                        .into_iter()
                        .map(|(label, valor)| {
                            view! {
                                <div class="atributo">
                                    <dt>{label}</dt>
                                    <dd>{valor}</dd>
                                </div>
                            }
                        })
                        .collect_view()}
                </dl>

                <a
                    class="btn btn--pink btn--block"
                    href=cta
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    "Pedir orçamento ›"
                </a>
            </div>

            <div class="detalhe__voltar">
                <a class="btn btn--ghost" href="/produtos">"Ver todos os produtos"</a>
            </div>
        </section>
    }
}
