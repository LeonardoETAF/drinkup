use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;

use super::confirmar::confirmar;
use crate::api::eventos_admin::{excluir_evento, listar_eventos_admin};
use crate::domain::EventoLista;

const IC_EDIT: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 20h9"/><path d="M16.5 3.5a2.1 2.1 0 0 1 3 3L7 19l-4 1 1-4z"/></svg>"#;
const IC_DEL: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M10 11v6M14 11v6"/></svg>"#;

type AcaoExcluir = Action<Uuid, Result<(), ServerFnError>>;

/// "Eventos" = categorias do carrossel da home (lista ordenada).
#[component]
pub fn AdminEventos() -> impl IntoView {
    let versao = RwSignal::new(0u32);
    let dados = RwSignal::new(None::<Result<Vec<EventoLista>, ServerFnError>>);

    Effect::new(move |_| {
        versao.get();
        dados.set(None);
        spawn_local(async move {
            dados.set(Some(listar_eventos_admin().await));
        });
    });

    let excluir = Action::new(|id: &Uuid| {
        let id = *id;
        async move { excluir_evento(id).await }
    });
    Effect::new(move |_| {
        if matches!(excluir.value().get(), Some(Ok(()))) {
            versao.update(|v| *v += 1);
        }
    });

    view! {
        <header class="admin-head admin-head--row">
            <div>
                <h1 class="admin-head__title">"Eventos"</h1>
                <p class="admin-head__sub">"Categorias do carrossel da home"</p>
            </div>
            <a class="btn btn--primary" href="/admin/eventos/novo">"+ Novo evento"</a>
        </header>

        <section class="admin-card">
            {move || match dados.get() {
                None => view! { <p class="admin-status">"Carregando..."</p> }.into_any(),
                Some(Err(_)) => {
                    view! { <p class="admin-status">"Não foi possível carregar."</p> }.into_any()
                }
                Some(Ok(itens)) if itens.is_empty() => {
                    view! { <p class="admin-status">"Nenhuma categoria. Crie a primeira."</p> }
                        .into_any()
                }
                Some(Ok(itens)) => lista(itens, excluir).into_any(),
            }}
        </section>
    }
}

fn lista(itens: Vec<EventoLista>, excluir: AcaoExcluir) -> AnyView {
    view! {
        <ul class="evt-lista">
            {itens
                .into_iter()
                .map(|e| {
                    let id = e.id;
                    let editar = format!("/admin/eventos/{id}");
                    let cor = e.cor.unwrap_or_else(|| "#262626".to_string());
                    let (badge, txt) = if e.ativo {
                        ("badge badge--green", "Visível")
                    } else {
                        ("badge badge--muted", "Oculto")
                    };
                    view! {
                        <li class="evt-item">
                            <span class="evt-ordem">{e.ordem}</span>
                            <span class="evt-cor" style=format!("background:{cor}")></span>
                            <span class="evt-nome">{e.titulo}</span>
                            <span class=badge>{txt}</span>
                            <div class="evt-acoes">
                                <a class="icon-btn" href=editar inner_html=IC_EDIT></a>
                                <button
                                    class="icon-btn icon-btn--danger"
                                    inner_html=IC_DEL
                                    on:click=move |_| {
                                        if confirmar("Excluir esta categoria?") {
                                            excluir.dispatch(id);
                                        }
                                    }
                                ></button>
                            </div>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
    .into_any()
}
