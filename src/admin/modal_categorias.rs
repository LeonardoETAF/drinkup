use super::icons::IC_DEL;
use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;

use super::modal::ModalConfirmacao;
use crate::api::categorias_admin::{
    criar_categoria, excluir_categoria, listar_categorias_admin, renomear_categoria,
};
use crate::domain::Categoria;

const IC_EDIT: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 20h9"/><path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4 12.5-12.5z"/></svg>"#;
const IC_OK: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><polyline points="20 6 9 17 4 12"/></svg>"#;
const IC_X: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>"#;

/// Modal de gerência de categorias e subcategorias (listar, adicionar, renomear,
/// excluir).
#[component]
pub fn ModalCategorias(aberto: RwSignal<bool>) -> impl IntoView {
    let versao = RwSignal::new(0u32);
    let cats = RwSignal::new(None::<Vec<Categoria>>);
    let novo = RwSignal::new(String::new());
    // Categoria-pai selecionada (vazio = criar categoria principal).
    let pai = RwSignal::new(String::new());
    // Item em edição (renomeando), com o texto sendo digitado.
    let editando = RwSignal::new(None::<Uuid>);
    let edit_nome = RwSignal::new(String::new());
    // Categoria/subcategoria aguardando confirmação de exclusão, ou None.
    let cat_pendente = RwSignal::new(None::<Uuid>);

    Effect::new(move |_| {
        if !aberto.get() {
            return;
        }
        versao.get();
        spawn_local(async move {
            cats.set(listar_categorias_admin().await.ok());
        });
    });

    let criar = Action::new(|args: &(String, Option<Uuid>)| {
        let (nome, parent) = args.clone();
        async move { criar_categoria(nome, parent).await }
    });
    let renomear = Action::new(|args: &(Uuid, String)| {
        let (id, nome) = (args.0, args.1.clone());
        async move { renomear_categoria(id, nome).await }
    });
    let excluir = Action::new(|id: &Uuid| {
        let id = *id;
        async move { excluir_categoria(id).await }
    });
    Effect::new(move |_| {
        let mudou = matches!(criar.value().get(), Some(Ok(())))
            || matches!(renomear.value().get(), Some(Ok(())))
            || matches!(excluir.value().get(), Some(Ok(())));
        if mudou {
            novo.set(String::new());
            pai.set(String::new());
            editando.set(None);
            versao.update(|v| *v += 1);
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let n = novo.get_untracked().trim().to_string();
        if n.is_empty() {
            return;
        }
        let p = pai.get_untracked();
        let parent = (!p.is_empty()).then(|| Uuid::parse_str(&p).ok()).flatten();
        criar.dispatch((n, parent));
    };

    // Salva o nome em edição (se válido) para o item informado.
    let salvar_edicao = move |id: Uuid| {
        let n = edit_nome.get_untracked().trim().to_string();
        if !n.is_empty() {
            renomear.dispatch((id, n));
        }
    };

    // Uma linha da lista (categoria ou subcategoria), com edição inline.
    let fila = move |id: Uuid, nome: String, is_sub: bool| {
        let classe = if is_sub {
            "cat-item cat-item--sub"
        } else {
            "cat-item"
        };
        view! {
            <li class=classe>
                {move || {
                    if editando.get() == Some(id) {
                        view! {
                            <input
                                class="admin-input cat-item__edit"
                                type="text"
                                prop:value=move || edit_nome.get()
                                on:input=move |ev| edit_nome.set(event_target_value(&ev))
                                on:keydown=move |ev| {
                                    match ev.key().as_str() {
                                        "Enter" => {
                                            ev.prevent_default();
                                            salvar_edicao(id);
                                        }
                                        "Escape" => editando.set(None),
                                        _ => {}
                                    }
                                }
                            />
                            <div class="cat-item__acoes">
                                <button
                                    type="button"
                                    class="icon-btn"
                                    title="Salvar"
                                    inner_html=IC_OK
                                    on:click=move |_| salvar_edicao(id)
                                ></button>
                                <button
                                    type="button"
                                    class="icon-btn"
                                    title="Cancelar"
                                    inner_html=IC_X
                                    on:click=move |_| editando.set(None)
                                ></button>
                            </div>
                        }
                            .into_any()
                    } else {
                        let nome_edit = nome.clone();
                        view! {
                            <span>{nome.clone()}</span>
                            <div class="cat-item__acoes">
                                <button
                                    type="button"
                                    class="icon-btn"
                                    title="Renomear"
                                    inner_html=IC_EDIT
                                    on:click=move |_| {
                                        edit_nome.set(nome_edit.clone());
                                        editando.set(Some(id));
                                    }
                                ></button>
                                <button
                                    type="button"
                                    class="icon-btn icon-btn--danger"
                                    title="Excluir"
                                    inner_html=IC_DEL
                                    on:click=move |_| cat_pendente.set(Some(id))
                                ></button>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </li>
        }
    };

    view! {
        <Show when=move || aberto.get() fallback=|| ()>
            <div class="modal-overlay" role="presentation" on:click=move |_| aberto.set(false)>
                <div
                    class="modal modal--lista"
                    role="dialog"
                    aria-modal="true"
                    on:click=move |ev| ev.stop_propagation()
                >
                    <h3 class="modal__titulo">"Categorias e subcategorias"</h3>
                    <form class="cat-add" on:submit=on_submit>
                        <input
                            class="admin-input"
                            type="text"
                            placeholder="Nome..."
                            prop:value=move || novo.get()
                            on:input=move |ev| novo.set(event_target_value(&ev))
                        />
                        <select
                            class="admin-input"
                            prop:value=move || pai.get()
                            on:change=move |ev| pai.set(event_target_value(&ev))
                        >
                            <option value="">"— Categoria principal —"</option>
                            {move || {
                                cats.get()
                                    .unwrap_or_default()
                                    .into_iter()
                                    .filter(|c| c.parent_id.is_none())
                                    .map(|c| {
                                        view! { <option value=c.id.to_string()>{c.nome}</option> }
                                    })
                                    .collect_view()
                            }}
                        </select>
                        <button type="submit" class="btn btn--primary">
                            "Adicionar"
                        </button>
                    </form>

                    <ul class="cat-lista">
                        {move || match cats.get() {
                            None => {
                                view! { <li class="admin-status">"Carregando..."</li> }.into_any()
                            }
                            Some(v) if v.is_empty() => {
                                view! { <li class="admin-status">"Nenhuma categoria ainda."</li> }
                                    .into_any()
                            }
                            Some(v) => {
                                v.iter()
                                    .filter(|c| c.parent_id.is_none())
                                    .map(|c| {
                                        let id = c.id;
                                        let subs: Vec<Categoria> = v
                                            .iter()
                                            .filter(|s| s.parent_id == Some(id))
                                            .cloned()
                                            .collect();
                                        view! {
                                            {fila(id, c.nome.clone(), false)}
                                            {subs
                                                .into_iter()
                                                .map(|s| fila(s.id, s.nome, true))
                                                .collect_view()}
                                        }
                                    })
                                    .collect_view()
                                    .into_any()
                            }
                        }}
                    </ul>

                    <div class="modal__acoes">
                        <button
                            type="button"
                            class="btn btn--ghost"
                            on:click=move |_| aberto.set(false)
                        >
                            "Fechar"
                        </button>
                    </div>
                </div>
            </div>
        </Show>
        <ModalConfirmacao
            aberto=Signal::derive(move || cat_pendente.get().is_some())
            mensagem="Excluir? As subcategorias e o vínculo dos produtos também serão removidos."
            confirmar_texto="Excluir"
            ao_cancelar=Callback::new(move |()| cat_pendente.set(None))
            ao_confirmar=Callback::new(move |()| {
                if let Some(id) = cat_pendente.get_untracked() {
                    excluir.dispatch(id);
                }
                cat_pendente.set(None);
            })
        />
    }
}
