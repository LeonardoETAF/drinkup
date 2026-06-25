use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;

use super::modal::ModalConfirmacao;
use super::paginacao::AdminPaginacao;
use crate::api::novidades::{classificar_inscrito, excluir_inscrito, listar_inscritos};
use crate::domain::{PaginaInscritos, ADMIN_TABELA_POR_PAGINA};

/// Opções de classificação (valor no banco, rótulo na tela).
const CLASSIFICACOES: [(&str, &str); 4] = [
    ("novo", "Novo"),
    ("cliente", "Cliente"),
    ("potencial", "Potencial"),
    ("inativo", "Inativo"),
];

type AcaoClassificar = Action<(Uuid, String), Result<(), ServerFnError>>;

const IC_DEL: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M10 11v6M14 11v6"/></svg>"#;

/// Página de inscritos em "Novidades": lista (busca) e remoção. Dados buscados
/// no cliente (após a hidratação).
#[component]
pub fn AdminNovidades() -> impl IntoView {
    let busca = RwSignal::new(String::new());
    let pagina = RwSignal::new(1u32);
    let versao = RwSignal::new(0u32);
    let dados = RwSignal::new(None::<Result<PaginaInscritos, ServerFnError>>);
    let pendente = RwSignal::new(None::<Uuid>);

    let filtro = Memo::new(move |_| {
        let b = busca.get();
        (!b.trim().is_empty()).then(|| b.trim().to_string())
    });

    // Mudar a busca volta para a página 1.
    Effect::new(move |_| {
        filtro.track();
        if pagina.get_untracked() != 1 {
            pagina.set(1);
        }
    });

    Effect::new(move |_| {
        let f = filtro.get();
        let pag = pagina.get();
        versao.get();
        dados.set(None);
        spawn_local(async move {
            dados.set(Some(listar_inscritos(f, pag).await));
        });
    });

    let total_paginas = Signal::derive(move || {
        let total = dados.get().and_then(Result::ok).map_or(0, |p| p.total).max(0);
        let por = ADMIN_TABELA_POR_PAGINA.max(1);
        u32::try_from((total + por - 1) / por).unwrap_or(1).max(1)
    });

    let excluir = Action::new(|id: &Uuid| {
        let id = *id;
        async move { excluir_inscrito(id).await }
    });
    Effect::new(move |_| {
        if matches!(excluir.value().get(), Some(Ok(()))) {
            versao.update(|v| *v += 1);
        }
    });

    let classificar = Action::new(|(id, cl): &(Uuid, String)| {
        let (id, cl) = (*id, cl.clone());
        async move { classificar_inscrito(id, cl).await }
    });

    view! {
        <header class="admin-head">
            <h1 class="admin-head__title">"Novidades"</h1>
            <p class="admin-head__sub">"Inscritos para receber novidades no WhatsApp"</p>
        </header>

        <div class="admin-toolbar">
            <input
                class="admin-input"
                type="search"
                placeholder="Buscar por WhatsApp..."
                prop:value=move || busca.get()
                on:input=move |ev| busca.set(event_target_value(&ev))
            />
        </div>

        <section class="admin-card">
            {move || match dados.get() {
                None => view! { <p class="admin-status">"Carregando inscritos..."</p> }.into_any(),
                Some(Err(_)) => {
                    view! { <p class="admin-status">"Não foi possível carregar."</p> }.into_any()
                }
                Some(Ok(p)) => tabela(p, pendente, classificar).into_any(),
            }}
        </section>

        <AdminPaginacao pagina=pagina total_paginas=total_paginas/>

        <ModalConfirmacao
            aberto=Signal::derive(move || pendente.get().is_some())
            mensagem="Deseja remover este inscrito?"
            confirmar_texto="Remover"
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

fn tabela(
    p: PaginaInscritos,
    pendente: RwSignal<Option<Uuid>>,
    classificar: AcaoClassificar,
) -> AnyView {
    if p.itens.is_empty() {
        return view! { <p class="admin-status">"Nenhum inscrito ainda."</p> }.into_any();
    }
    view! {
        <p class="admin-card__meta">{format!("{} inscrito(s)", p.total)}</p>
        <div class="table-wrap">
            <table class="admin-table">
                <thead>
                    <tr>
                        <th>"WhatsApp"</th>
                        <th>"Classificação"</th>
                        <th>"Inscrição"</th>
                        <th>"Ações"</th>
                    </tr>
                </thead>
                <tbody>
                    {p
                        .itens
                        .into_iter()
                        .map(|i| {
                            let id = i.id;
                            let atual = i.classificacao.clone();
                            view! {
                                <tr>
                                    <td>{i.telefone}</td>
                                    <td>
                                        <select
                                            class="status-select"
                                            on:change=move |ev| {
                                                classificar.dispatch((id, event_target_value(&ev)));
                                            }
                                        >
                                            {CLASSIFICACOES
                                                .iter()
                                                .map(|&(valor, rotulo)| {
                                                    view! {
                                                        <option value=valor selected=atual == valor>
                                                            {rotulo}
                                                        </option>
                                                    }
                                                })
                                                .collect_view()}
                                        </select>
                                    </td>
                                    <td>{i.inscricao}</td>
                                    <td>
                                        <button
                                            type="button"
                                            class="icon-btn icon-btn--danger"
                                            title="Remover inscrito"
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
