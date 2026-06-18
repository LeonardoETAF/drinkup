use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::api::catalogo::obter_produto;
use crate::components::Gallery;
use crate::domain::ProdutoDetalhe;

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
    let produto = Resource::new(slug, |s| async move { obter_produto(s).await });

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
    let cta = format!(
        "/contato?produto={}&pid={}",
        urlencoding::encode(&produto.nome),
        produto.id
    );

    let atributos: Vec<(&'static str, String)> = [
        produto
            .capacidade_ml
            .map(|v| ("Capacidade", format!("{v} ml"))),
        produto.material.clone().map(|v| ("Material", v)),
        produto.cor.clone().map(|v| ("Cor", v)),
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

                <a class="btn btn--pink btn--block" href=cta>
                    "Pedir orçamento ›"
                </a>
            </div>

            <div class="detalhe__voltar">
                <a class="btn btn--ghost" href="/produtos">"Ver todos os produtos"</a>
            </div>
        </section>
    }
}
