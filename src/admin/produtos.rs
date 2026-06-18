use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;

use super::confirmar::confirmar;
use crate::api::produtos_admin::{excluir_produto, listar_produtos_admin};
use crate::domain::ProdutoLista;

const IC_EDIT: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 20h9"/><path d="M16.5 3.5a2.1 2.1 0 0 1 3 3L7 19l-4 1 1-4z"/></svg>"#;
const IC_DEL: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M10 11v6M14 11v6"/></svg>"#;

type AcaoExcluir = Action<Uuid, Result<(), ServerFnError>>;

/// Lista de produtos no painel (buscar, novo, editar, excluir).
#[component]
pub fn AdminProdutos() -> impl IntoView {
    let busca = RwSignal::new(String::new());
    let versao = RwSignal::new(0u32);
    let dados = RwSignal::new(None::<Result<Vec<ProdutoLista>, ServerFnError>>);

    Effect::new(move |_| {
        let b = busca.get();
        versao.get();
        dados.set(None);
        spawn_local(async move {
            dados.set(Some(listar_produtos_admin(b).await));
        });
    });

    let excluir = Action::new(|id: &Uuid| {
        let id = *id;
        async move { excluir_produto(id).await }
    });
    Effect::new(move |_| {
        if matches!(excluir.value().get(), Some(Ok(()))) {
            versao.update(|v| *v += 1);
        }
    });

    view! {
        <header class="admin-head admin-head--row">
            <div>
                <h1 class="admin-head__title">"Produtos"</h1>
                <p class="admin-head__sub">"Gerencie o catálogo de copos"</p>
            </div>
            <a class="btn btn--primary" href="/admin/produtos/novo">"+ Novo produto"</a>
        </header>

        <div class="admin-toolbar">
            <input
                class="admin-input"
                type="search"
                placeholder="Buscar produto..."
                prop:value=move || busca.get()
                on:input=move |ev| busca.set(event_target_value(&ev))
            />
        </div>

        <section class="admin-card">
            {move || match dados.get() {
                None => view! { <p class="admin-status">"Carregando..."</p> }.into_any(),
                Some(Err(_)) => {
                    view! { <p class="admin-status">"Não foi possível carregar."</p> }.into_any()
                }
                Some(Ok(itens)) if itens.is_empty() => {
                    view! { <p class="admin-status">"Nenhum produto. Crie o primeiro."</p> }
                        .into_any()
                }
                Some(Ok(itens)) => tabela(itens, excluir).into_any(),
            }}
        </section>
    }
}

fn tabela(itens: Vec<ProdutoLista>, excluir: AcaoExcluir) -> AnyView {
    view! {
        <div class="table-wrap">
            <table class="admin-table">
                <thead>
                    <tr>
                        <th>"Produto"</th>
                        <th>"Categoria"</th>
                        <th>"Volume"</th>
                        <th>"Status"</th>
                        <th class="col-acoes">"Ações"</th>
                    </tr>
                </thead>
                <tbody>
                    {itens
                        .into_iter()
                        .map(|p| {
                            let id = p.id;
                            let editar = format!("/admin/produtos/{id}");
                            let (badge, txt) = if p.ativo {
                                ("badge badge--green", "Ativo")
                            } else {
                                ("badge badge--muted", "Inativo")
                            };
                            let vol = p
                                .capacidade_ml
                                .map(|v| format!("{v} ml"))
                                .unwrap_or_else(|| "—".to_string());
                            let cat = p.categoria.unwrap_or_else(|| "—".to_string());
                            view! {
                                <tr>
                                    <td class="lead-nome">
                                        {match p.imagem_url {
                                            Some(u) => {
                                                view! {
                                                    <img class="prod-thumb" src=u alt="" loading="lazy"/>
                                                }
                                                    .into_any()
                                            }
                                            None => {
                                                view! { <span class="prod-thumb prod-thumb--ph"></span> }
                                                    .into_any()
                                            }
                                        }}
                                        {p.nome}
                                    </td>
                                    <td>{cat}</td>
                                    <td>{vol}</td>
                                    <td>
                                        <span class=badge>{txt}</span>
                                    </td>
                                    <td class="col-acoes">
                                        <a class="icon-btn" href=editar inner_html=IC_EDIT></a>
                                        <button
                                            class="icon-btn icon-btn--danger"
                                            inner_html=IC_DEL
                                            on:click=move |_| {
                                                let confirmado = confirmar(
                                                    "Excluir este produto?",
                                                );
                                                if confirmado {
                                                    excluir.dispatch(id);
                                                }
                                            }
                                        ></button>
                                    </td>
                                </tr>
                            }
                        })
                        .collect_view()}
                </tbody>
            </table>
        </div>
    }
    .into_any()
}
