use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;

use super::paginacao::AdminPaginacao;
use super::util::iniciais;
use crate::api::admin::{atualizar_status_lead, listar_leads};
use crate::domain::{PaginaLeads, ADMIN_TABELA_POR_PAGINA};

/// Página de leads: lista (busca + filtro) e alteração de status. Dados buscados
/// no cliente (após a hidratação).
#[component]
pub fn AdminLeads() -> impl IntoView {
    let busca = RwSignal::new(String::new());
    let status_f = RwSignal::new(String::new());
    let pagina = RwSignal::new(1u32);
    let versao = RwSignal::new(0u32);
    let dados = RwSignal::new(None::<Result<PaginaLeads, ServerFnError>>);

    // Filtro como par (busca, status) — argumentos separados do server fn.
    let filtro = Memo::new(move |_| {
        let b = busca.get();
        let s = status_f.get();
        (
            (!b.trim().is_empty()).then(|| b.trim().to_string()),
            (!s.is_empty()).then_some(s),
        )
    });

    // Mudar o filtro volta para a página 1 (não "esconder" resultados).
    Effect::new(move |_| {
        filtro.track();
        if pagina.get_untracked() != 1 {
            pagina.set(1);
        }
    });

    // Busca (re)disparada por mudança de filtro, página ou após alterar status.
    Effect::new(move |_| {
        let (b, s) = filtro.get();
        let pag = pagina.get();
        versao.get();
        dados.set(None);
        spawn_local(async move {
            dados.set(Some(listar_leads(b, s, pag).await));
        });
    });

    let total_paginas = Signal::derive(move || {
        let total = dados.get().and_then(Result::ok).map_or(0, |p| p.total).max(0);
        let por = ADMIN_TABELA_POR_PAGINA.max(1);
        u32::try_from((total + por - 1) / por).unwrap_or(1).max(1)
    });

    let mudar = Action::new(|(id, st): &(Uuid, String)| {
        let (id, st) = (*id, st.clone());
        async move { atualizar_status_lead(id, st).await }
    });
    Effect::new(move |_| {
        if matches!(mudar.value().get(), Some(Ok(()))) {
            versao.update(|v| *v += 1);
        }
    });

    view! {
        <header class="admin-head">
            <h1 class="admin-head__title">"Leads"</h1>
            <p class="admin-head__sub">"Orçamentos e contatos recebidos"</p>
        </header>

        <div class="admin-toolbar">
            <input
                class="admin-input"
                type="search"
                placeholder="Buscar por nome ou WhatsApp..."
                prop:value=move || busca.get()
                on:input=move |ev| busca.set(event_target_value(&ev))
            />
            <select class="admin-input" on:change=move |ev| status_f.set(event_target_value(&ev))>
                <option value="">"Todos os status"</option>
                <option value="novo">"Novo"</option>
                <option value="em_atendimento">"Em atendimento"</option>
                <option value="convertido">"Convertido"</option>
                <option value="perdido">"Perdido"</option>
            </select>
        </div>

        <section class="admin-card">
            {move || match dados.get() {
                None => view! { <p class="admin-status">"Carregando leads..."</p> }.into_any(),
                Some(Err(_)) => {
                    view! { <p class="admin-status">"Não foi possível carregar."</p> }.into_any()
                }
                Some(Ok(p)) => tabela_leads(p, mudar).into_any(),
            }}
        </section>

        <AdminPaginacao pagina=pagina total_paginas=total_paginas/>
    }
}

type AcaoStatus = Action<(Uuid, String), Result<(), ServerFnError>>;

fn tabela_leads(p: PaginaLeads, mudar: AcaoStatus) -> AnyView {
    if p.itens.is_empty() {
        return view! { <p class="admin-status">"Nenhum lead encontrado."</p> }.into_any();
    }
    view! {
        <p class="admin-card__meta">{format!("{} lead(s)", p.total)}</p>
        <div class="table-wrap">
            <table class="admin-table">
                <thead>
                    <tr>
                        <th>"Nome"</th>
                        <th>"WhatsApp"</th>
                        <th>"Origem"</th>
                        <th>"Data"</th>
                        <th>"Status"</th>
                    </tr>
                </thead>
                <tbody>
                    {p
                        .itens
                        .into_iter()
                        .map(|l| {
                            let id = l.id;
                            let st = l.status.clone();
                            let avatar = iniciais(&l.nome);
                            view! {
                                <tr>
                                    <td class="lead-nome">
                                        <span class="admin-avatar">{avatar}</span>
                                        {l.nome}
                                    </td>
                                    <td>{l.contato}</td>
                                    <td>{l.origem}</td>
                                    <td>{l.inscricao}</td>
                                    <td>
                                        <select
                                            class="status-select"
                                            on:change=move |ev| {
                                                mudar.dispatch((id, event_target_value(&ev)));
                                            }
                                        >
                                            <option value="novo" selected=st == "novo">
                                                "Novo"
                                            </option>
                                            <option
                                                value="em_atendimento"
                                                selected=st == "em_atendimento"
                                            >
                                                "Em atendimento"
                                            </option>
                                            <option value="convertido" selected=st == "convertido">
                                                "Convertido"
                                            </option>
                                            <option value="perdido" selected=st == "perdido">
                                                "Perdido"
                                            </option>
                                        </select>
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
