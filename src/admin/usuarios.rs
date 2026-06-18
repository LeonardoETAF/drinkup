use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;

use super::confirmar::confirmar;
use crate::api::usuarios_admin::{excluir_usuario, listar_usuarios};
use crate::domain::UsuarioLista;

const IC_EDIT: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 20h9"/><path d="M16.5 3.5a2.1 2.1 0 0 1 3 3L7 19l-4 1 1-4z"/></svg>"#;
const IC_DEL: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M10 11v6M14 11v6"/></svg>"#;

type AcaoExcluir = Action<Uuid, Result<(), ServerFnError>>;

/// Lista de usuários do painel (somente admin).
#[component]
pub fn AdminUsuarios() -> impl IntoView {
    let versao = RwSignal::new(0u32);
    let dados = RwSignal::new(None::<Result<Vec<UsuarioLista>, ServerFnError>>);

    Effect::new(move |_| {
        versao.get();
        dados.set(None);
        spawn_local(async move {
            dados.set(Some(listar_usuarios().await));
        });
    });

    let excluir = Action::new(|id: &Uuid| {
        let id = *id;
        async move { excluir_usuario(id).await }
    });
    Effect::new(move |_| {
        if matches!(excluir.value().get(), Some(Ok(()))) {
            versao.update(|v| *v += 1);
        }
    });

    view! {
        <header class="admin-head admin-head--row">
            <div>
                <a class="admin-voltar" href="/admin/configuracoes">"‹ Configurações"</a>
                <h1 class="admin-head__title">"Usuários do painel"</h1>
                <p class="admin-head__sub">"Acesso e papéis (somente admin)"</p>
            </div>
            <a class="btn btn--primary" href="/admin/usuarios/novo">"+ Novo usuário"</a>
        </header>

        <section class="admin-card">
            {move || match dados.get() {
                None => view! { <p class="admin-status">"Carregando..."</p> }.into_any(),
                Some(Err(e)) => {
                    view! { <p class="admin-status">{e.to_string()}</p> }.into_any()
                }
                Some(Ok(itens)) => tabela(itens, excluir).into_any(),
            }}
        </section>
    }
}

fn tabela(itens: Vec<UsuarioLista>, excluir: AcaoExcluir) -> AnyView {
    view! {
        <div class="table-wrap">
            <table class="admin-table">
                <thead>
                    <tr>
                        <th>"Nome"</th>
                        <th>"E-mail"</th>
                        <th>"Papel"</th>
                        <th>"Status"</th>
                        <th>"Último acesso"</th>
                        <th class="col-acoes">"Ações"</th>
                    </tr>
                </thead>
                <tbody>
                    {itens
                        .into_iter()
                        .map(|u| {
                            let id = u.id;
                            let editar = format!("/admin/usuarios/{id}");
                            let (badge, txt) = if u.ativo {
                                ("badge badge--green", "Ativo")
                            } else {
                                ("badge badge--muted", "Inativo")
                            };
                            let acesso = u.ultimo_login.unwrap_or_else(|| "—".to_string());
                            view! {
                                <tr>
                                    <td>{u.nome}</td>
                                    <td>{u.email}</td>
                                    <td>
                                        <span class="badge badge--lime badge--cap">{u.papel}</span>
                                    </td>
                                    <td>
                                        <span class=badge>{txt}</span>
                                    </td>
                                    <td>{acesso}</td>
                                    <td class="col-acoes">
                                        <a class="icon-btn" href=editar inner_html=IC_EDIT></a>
                                        <button
                                            class="icon-btn icon-btn--danger"
                                            inner_html=IC_DEL
                                            on:click=move |_| {
                                                if confirmar("Excluir este usuário?") {
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
