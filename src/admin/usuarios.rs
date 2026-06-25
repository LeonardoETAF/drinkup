use super::icons::IC_DEL;
use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;

use super::modal::ModalConfirmacao;
use super::paginacao::AdminPaginacao;
use crate::api::usuarios_admin::{excluir_usuario, listar_usuarios};
use crate::domain::{Pagina, UsuarioLista, ADMIN_TABELA_POR_PAGINA};

const IC_EDIT: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 20h9"/><path d="M16.5 3.5a2.1 2.1 0 0 1 3 3L7 19l-4 1 1-4z"/></svg>"#;

/// Lista de usuários do painel (somente admin).
#[component]
pub fn AdminUsuarios() -> impl IntoView {
    let pagina = RwSignal::new(1u32);
    let versao = RwSignal::new(0u32);
    let dados = RwSignal::new(None::<Result<Pagina<UsuarioLista>, ServerFnError>>);

    Effect::new(move |_| {
        let pag = pagina.get();
        versao.get();
        dados.set(None);
        spawn_local(async move {
            dados.set(Some(listar_usuarios(pag).await));
        });
    });

    let total_paginas = Signal::derive(move || {
        let total = dados.get().and_then(Result::ok).map_or(0, |p| p.total).max(0);
        let por = ADMIN_TABELA_POR_PAGINA.max(1);
        u32::try_from((total + por - 1) / por).unwrap_or(1).max(1)
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

    let pendente = RwSignal::new(None::<Uuid>);

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
                    view! { <p class="admin-status">{crate::components::mensagem_erro(&e)}</p> }.into_any()
                }
                Some(Ok(p)) => tabela(p.itens, pendente).into_any(),
            }}
        </section>

        <AdminPaginacao pagina=pagina total_paginas=total_paginas/>

        <ModalConfirmacao
            aberto=Signal::derive(move || pendente.get().is_some())
            mensagem="Excluir este usuário?"
            confirmar_texto="Excluir"
            ao_cancelar=Callback::new(move |()| pendente.set(None))
            ao_confirmar=Callback::new(move |()| {
                if let Some(id) = pendente.get_untracked() {
                    excluir.dispatch(id);
                }
                pendente.set(None);
            })
        />
    }
}

fn tabela(itens: Vec<UsuarioLista>, pendente: RwSignal<Option<Uuid>>) -> AnyView {
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
