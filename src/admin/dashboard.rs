use leptos::prelude::*;
use leptos::task::spawn_local;

use super::util::{status_classe, status_label};
use crate::api::admin::resumo_dashboard;
use crate::domain::{DashboardResumo, LeadResumo};

/// Dashboard do painel: KPIs + leads recentes. Os dados são buscados no cliente
/// (após a hidratação), evitando acesso a sessão/banco durante o SSR.
#[component]
pub fn AdminDashboard() -> impl IntoView {
    let dados = RwSignal::new(None::<Result<DashboardResumo, ServerFnError>>);
    Effect::new(move |_| {
        spawn_local(async move {
            dados.set(Some(resumo_dashboard().await));
        });
    });

    view! {
        <header class="admin-head">
            <h1 class="admin-head__title">"Dashboard"</h1>
            <p class="admin-head__sub">"Visão geral do DRINK UP"</p>
        </header>

        {move || match dados.get() {
            None => view! { <p class="admin-status">"Carregando..."</p> }.into_any(),
            Some(Err(_)) => {
                view! { <p class="admin-status">"Não foi possível carregar o dashboard."</p> }
                    .into_any()
            }
            Some(Ok(r)) => {
                view! {
                    <div class="kpi-grid">
                        <Kpi rotulo="Produtos" valor=r.total_produtos/>
                        <Kpi rotulo="Leads" valor=r.total_leads/>
                        <Kpi rotulo="Leads novos" valor=r.leads_novos destaque=true/>
                        <Kpi rotulo="Eventos" valor=r.total_eventos/>
                    </div>
                    <section class="admin-card">
                        <h2 class="admin-card__title">"Leads recentes"</h2>
                        {tabela_recentes(r.recentes)}
                    </section>
                }
                    .into_any()
            }
        }}
    }
}

#[component]
fn Kpi(rotulo: &'static str, valor: i64, #[prop(optional)] destaque: bool) -> impl IntoView {
    view! {
        <div class="kpi" class:kpi--destaque=destaque>
            <span class="kpi__valor">{valor}</span>
            <span class="kpi__rotulo">{rotulo}</span>
        </div>
    }
}

fn tabela_recentes(itens: Vec<LeadResumo>) -> AnyView {
    if itens.is_empty() {
        return view! { <p class="admin-status">"Nenhum lead ainda."</p> }.into_any();
    }
    view! {
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
                    {itens
                        .into_iter()
                        .map(|l| {
                            view! {
                                <tr>
                                    <td>{l.nome}</td>
                                    <td>{l.contato}</td>
                                    <td>{l.origem}</td>
                                    <td>{l.inscricao}</td>
                                    <td>
                                        <span class=status_classe(&l.status)>
                                            {status_label(&l.status)}
                                        </span>
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
