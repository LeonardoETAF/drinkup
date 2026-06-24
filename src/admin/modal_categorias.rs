use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;

use super::modal::ModalConfirmacao;
use crate::api::categorias_admin::{criar_categoria, excluir_categoria, listar_categorias_admin};
use crate::domain::Categoria;

const IC_DEL: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M10 11v6M14 11v6"/></svg>"#;

/// Modal de gerência de categorias e subcategorias (listar, adicionar, excluir).
#[component]
pub fn ModalCategorias(aberto: RwSignal<bool>) -> impl IntoView {
    let versao = RwSignal::new(0u32);
    let cats = RwSignal::new(None::<Vec<Categoria>>);
    let novo = RwSignal::new(String::new());
    // Categoria-pai selecionada (vazio = criar categoria principal).
    let pai = RwSignal::new(String::new());
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
    let excluir = Action::new(|id: &Uuid| {
        let id = *id;
        async move { excluir_categoria(id).await }
    });
    Effect::new(move |_| {
        let mudou = matches!(criar.value().get(), Some(Ok(())))
            || matches!(excluir.value().get(), Some(Ok(())));
        if mudou {
            novo.set(String::new());
            pai.set(String::new());
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
                                        let nome = c.nome.clone();
                                        let subs: Vec<Categoria> = v
                                            .iter()
                                            .filter(|s| s.parent_id == Some(id))
                                            .cloned()
                                            .collect();
                                        view! {
                                            <li class="cat-item">
                                                <span>{nome}</span>
                                                <button
                                                    type="button"
                                                    class="icon-btn icon-btn--danger"
                                                    title="Excluir"
                                                    inner_html=IC_DEL
                                                    on:click=move |_| cat_pendente.set(Some(id))
                                                ></button>
                                            </li>
                                            {subs
                                                .into_iter()
                                                .map(|s| {
                                                    let sid = s.id;
                                                    view! {
                                                        <li class="cat-item cat-item--sub">
                                                            <span>{s.nome}</span>
                                                            <button
                                                                type="button"
                                                                class="icon-btn icon-btn--danger"
                                                                title="Excluir"
                                                                inner_html=IC_DEL
                                                                on:click=move |_| {
                                                                    cat_pendente.set(Some(sid))
                                                                }
                                                            ></button>
                                                        </li>
                                                    }
                                                })
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
