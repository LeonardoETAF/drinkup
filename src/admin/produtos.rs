use super::icons::IC_DEL;
use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;

use super::modal::ModalConfirmacao;
use super::modal_categorias::ModalCategorias;
use super::paginacao::AdminPaginacao;
use crate::api::produtos_admin::{alternar_produto, excluir_produto, listar_produtos_admin};
use crate::domain::{PaginaProdutosLista, ProdutoLista, PRODUTOS_ADMIN_POR_PAGINA};

type Acao = Action<Uuid, Result<(), ServerFnError>>;

const IC_EDIT: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 20h9"/><path d="M16.5 3.5a2.1 2.1 0 0 1 3 3L7 19l-4 1 1-4z"/></svg>"#;

/// Lista de produtos no painel (buscar, novo, editar, excluir).
#[component]
pub fn AdminProdutos() -> impl IntoView {
    let busca = RwSignal::new(String::new());
    let pagina = RwSignal::new(1u32);
    let versao = RwSignal::new(0u32);
    let dados = RwSignal::new(None::<Result<PaginaProdutosLista, ServerFnError>>);

    Effect::new(move |_| {
        let b = busca.get();
        let pag = pagina.get();
        versao.get();
        dados.set(None);
        spawn_local(async move {
            dados.set(Some(listar_produtos_admin(b, pag).await));
        });
    });

    // Total de páginas a partir do total devolvido (arredonda para cima).
    let total_paginas = Signal::derive(move || {
        let total = dados
            .get()
            .and_then(Result::ok)
            .map_or(0, |p| p.total)
            .max(0);
        let por = PRODUTOS_ADMIN_POR_PAGINA.max(1);
        u32::try_from((total + por - 1) / por).unwrap_or(1).max(1)
    });

    let excluir = Action::new(|id: &Uuid| {
        let id = *id;
        async move { excluir_produto(id).await }
    });
    let alternar: Acao = Action::new(|id: &Uuid| {
        let id = *id;
        async move { alternar_produto(id).await }
    });
    Effect::new(move |_| {
        let mudou = matches!(excluir.value().get(), Some(Ok(())))
            || matches!(alternar.value().get(), Some(Ok(())));
        if mudou {
            versao.update(|v| *v += 1);
        }
    });

    let pendente = RwSignal::new(None::<Uuid>);
    let cats_aberto = RwSignal::new(false);

    view! {
        <header class="admin-head admin-head--row">
            <div>
                <h1 class="admin-head__title">"Produtos"</h1>
                <p class="admin-head__sub">"Gerencie o catálogo de copos"</p>
            </div>
            <div class="admin-head__acoes">
                <button
                    type="button"
                    class="btn btn--ghost"
                    on:click=move |_| cats_aberto.set(true)
                >
                    "Categorias"
                </button>
                <a class="btn btn--primary" href="/admin/produtos/novo">"+ Novo produto"</a>
            </div>
        </header>

        <ModalCategorias aberto=cats_aberto/>

        <ModalConfirmacao
            aberto=Signal::derive(move || pendente.get().is_some())
            mensagem="Excluir este produto?"
            confirmar_texto="Excluir"
            ao_cancelar=Callback::new(move |()| pendente.set(None))
            ao_confirmar=Callback::new(move |()| {
                if let Some(id) = pendente.get_untracked() {
                    excluir.dispatch(id);
                }
                pendente.set(None);
            })
        />

        <div class="admin-toolbar">
            <input
                class="admin-input"
                type="search"
                placeholder="Buscar produto..."
                prop:value=move || busca.get()
                on:input=move |ev| {
                    busca.set(event_target_value(&ev));
                    pagina.set(1);
                }
            />
        </div>

        <section class="admin-card">
            {move || match dados.get() {
                None => view! { <p class="admin-status">"Carregando..."</p> }.into_any(),
                Some(Err(_)) => {
                    view! { <p class="admin-status">"Não foi possível carregar."</p> }.into_any()
                }
                Some(Ok(p)) if p.itens.is_empty() => {
                    view! { <p class="admin-status">"Nenhum produto. Crie o primeiro."</p> }
                        .into_any()
                }
                Some(Ok(p)) => tabela(p.itens, pendente, alternar).into_any(),
            }}
        </section>

        <AdminPaginacao pagina=pagina total_paginas=total_paginas/>
    }
}

fn tabela(itens: Vec<ProdutoLista>, pendente: RwSignal<Option<Uuid>>, alternar: Acao) -> AnyView {
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
                            let (badge, txt, titulo_btn) = if p.ativo {
                                ("badge badge--green badge--btn", "Ativo", "Desativar")
                            } else {
                                ("badge badge--muted badge--btn", "Inativo", "Ativar")
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
                                        <button
                                            type="button"
                                            class=badge
                                            title=titulo_btn
                                            on:click=move |_| {
                                                alternar.dispatch(id);
                                            }
                                        >
                                            {txt}
                                        </button>
                                    </td>
                                    <td class="col-acoes">
                                        <a class="icon-btn" href=editar title="Editar" inner_html=IC_EDIT></a>
                                        <button
                                            class="icon-btn icon-btn--danger"
                                            inner_html=IC_DEL
                                            on:click=move |_| pendente.set(Some(id))
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
