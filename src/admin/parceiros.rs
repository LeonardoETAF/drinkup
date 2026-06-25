use super::icons::IC_DEL;
use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;

use super::modal::ModalConfirmacao;
use super::paginacao::AdminPaginacao;
use crate::api::parceiros_admin::{alternar_parceiro, excluir_parceiro, listar_parceiros_admin};
use crate::domain::{Pagina, ParceiroLista, ADMIN_TABELA_POR_PAGINA};

const IC_EDIT: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 20h9"/><path d="M16.5 3.5a2.1 2.1 0 0 1 3 3L7 19l-4 1 1-4z"/></svg>"#;
const IC_EYE: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7-10-7-10-7z"/><circle cx="12" cy="12" r="3"/></svg>"#;
const IC_EYE_OFF: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>"#;

type Acao = Action<Uuid, Result<(), ServerFnError>>;

/// Grade de parceiros no painel (logo + nome + editar/excluir).
#[component]
pub fn AdminParceiros() -> impl IntoView {
    let busca = RwSignal::new(String::new());
    let pagina = RwSignal::new(1u32);
    let versao = RwSignal::new(0u32);
    let dados = RwSignal::new(None::<Result<Pagina<ParceiroLista>, ServerFnError>>);

    Effect::new(move |_| {
        let b = busca.get();
        let pag = pagina.get();
        versao.get();
        dados.set(None);
        spawn_local(async move {
            dados.set(Some(listar_parceiros_admin(b, pag).await));
        });
    });

    let total_paginas = Signal::derive(move || {
        let total = dados.get().and_then(Result::ok).map_or(0, |p| p.total).max(0);
        let por = ADMIN_TABELA_POR_PAGINA.max(1);
        u32::try_from((total + por - 1) / por).unwrap_or(1).max(1)
    });

    let excluir = Action::new(|id: &Uuid| {
        let id = *id;
        async move { excluir_parceiro(id).await }
    });
    let alternar: Acao = Action::new(|id: &Uuid| {
        let id = *id;
        async move { alternar_parceiro(id).await }
    });
    Effect::new(move |_| {
        let mudou = matches!(excluir.value().get(), Some(Ok(())))
            || matches!(alternar.value().get(), Some(Ok(())));
        if mudou {
            versao.update(|v| *v += 1);
        }
    });

    let pendente = RwSignal::new(None::<Uuid>);

    view! {
        <header class="admin-head admin-head--row">
            <div>
                <h1 class="admin-head__title">"Parceiros"</h1>
                <p class="admin-head__sub">"Marcas e clientes parceiros"</p>
            </div>
            <a class="btn btn--primary" href="/admin/parceiros/novo">"+ Novo parceiro"</a>
        </header>

        <ModalConfirmacao
            aberto=Signal::derive(move || pendente.get().is_some())
            mensagem="Excluir este parceiro?"
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
                placeholder="Buscar parceiro..."
                prop:value=move || busca.get()
                on:input=move |ev| {
                    busca.set(event_target_value(&ev));
                    pagina.set(1);
                }
            />
        </div>

        {move || match dados.get() {
            None => view! { <p class="admin-status">"Carregando..."</p> }.into_any(),
            Some(Err(_)) => {
                view! { <p class="admin-status">"Não foi possível carregar."</p> }.into_any()
            }
            Some(Ok(p)) if p.itens.is_empty() => {
                view! { <p class="admin-status">"Nenhum parceiro. Crie o primeiro."</p> }.into_any()
            }
            Some(Ok(p)) => grade(p.itens, pendente, alternar).into_any(),
        }}

        <AdminPaginacao pagina=pagina total_paginas=total_paginas/>
    }
}

fn grade(itens: Vec<ParceiroLista>, pendente: RwSignal<Option<Uuid>>, alternar: Acao) -> AnyView {
    view! {
        <div class="parceiro-grid">
            {itens
                .into_iter()
                .map(|p| {
                    let id = p.id;
                    let ativo = p.ativo;
                    let editar = format!("/admin/parceiros/{id}");
                    view! {
                        <div class="parceiro-card" class:is-inativo=!ativo>
                            <div class="parceiro-card__logo">
                                {match p.logo_url {
                                    Some(u) => {
                                        view! { <img src=u alt=p.nome.clone() loading="lazy"/> }
                                            .into_any()
                                    }
                                    None => view! { <span>{p.nome.clone()}</span> }.into_any(),
                                }}
                            </div>
                            <h3 class="parceiro-card__nome">{p.nome}</h3>
                            <div class="parceiro-card__acoes">
                                <button
                                    type="button"
                                    class=if ativo { "icon-btn" } else { "icon-btn icon-btn--off" }
                                    title=if ativo { "Desativar" } else { "Ativar" }
                                    inner_html=if ativo { IC_EYE } else { IC_EYE_OFF }
                                    on:click=move |_| {
                                        alternar.dispatch(id);
                                    }
                                ></button>
                                <a class="icon-btn" href=editar title="Editar" inner_html=IC_EDIT></a>
                                <button
                                    class="icon-btn icon-btn--danger"
                                    title="Excluir"
                                    inner_html=IC_DEL
                                    on:click=move |_| pendente.set(Some(id))
                                ></button>
                            </div>
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
    .into_any()
}
