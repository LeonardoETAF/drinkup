use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;

use super::confirmar::confirmar;
use crate::api::parceiros_admin::{excluir_parceiro, listar_parceiros_admin};
use crate::domain::ParceiroLista;

const IC_DEL: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M10 11v6M14 11v6"/></svg>"#;

type AcaoExcluir = Action<Uuid, Result<(), ServerFnError>>;

/// Grade de parceiros no painel (logo + nome + editar/excluir).
#[component]
pub fn AdminParceiros() -> impl IntoView {
    let busca = RwSignal::new(String::new());
    let versao = RwSignal::new(0u32);
    let dados = RwSignal::new(None::<Result<Vec<ParceiroLista>, ServerFnError>>);

    Effect::new(move |_| {
        let b = busca.get();
        versao.get();
        dados.set(None);
        spawn_local(async move {
            dados.set(Some(listar_parceiros_admin(b).await));
        });
    });

    let excluir = Action::new(|id: &Uuid| {
        let id = *id;
        async move { excluir_parceiro(id).await }
    });
    Effect::new(move |_| {
        if matches!(excluir.value().get(), Some(Ok(()))) {
            versao.update(|v| *v += 1);
        }
    });

    view! {
        <header class="admin-head admin-head--row">
            <div>
                <h1 class="admin-head__title">"Parceiros"</h1>
                <p class="admin-head__sub">"Marcas e clientes parceiros"</p>
            </div>
            <a class="btn btn--primary" href="/admin/parceiros/novo">"+ Novo parceiro"</a>
        </header>

        <div class="admin-toolbar">
            <input
                class="admin-input"
                type="search"
                placeholder="Buscar parceiro..."
                prop:value=move || busca.get()
                on:input=move |ev| busca.set(event_target_value(&ev))
            />
        </div>

        {move || match dados.get() {
            None => view! { <p class="admin-status">"Carregando..."</p> }.into_any(),
            Some(Err(_)) => {
                view! { <p class="admin-status">"Não foi possível carregar."</p> }.into_any()
            }
            Some(Ok(itens)) if itens.is_empty() => {
                view! { <p class="admin-status">"Nenhum parceiro. Crie o primeiro."</p> }.into_any()
            }
            Some(Ok(itens)) => grade(itens, excluir).into_any(),
        }}
    }
}

fn grade(itens: Vec<ParceiroLista>, excluir: AcaoExcluir) -> AnyView {
    view! {
        <div class="parceiro-grid">
            {itens
                .into_iter()
                .map(|p| {
                    let id = p.id;
                    let editar = format!("/admin/parceiros/{id}");
                    view! {
                        <div class="parceiro-card" class:is-inativo=!p.ativo>
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
                                <a class="btn btn--ghost" href=editar>"Editar"</a>
                                <button
                                    class="icon-btn icon-btn--danger"
                                    inner_html=IC_DEL
                                    on:click=move |_| {
                                        if confirmar("Excluir este parceiro?") {
                                            excluir.dispatch(id);
                                        }
                                    }
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
