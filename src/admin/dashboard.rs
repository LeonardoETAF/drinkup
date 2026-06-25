use leptos::prelude::*;
use leptos::task::spawn_local;

use super::util::{status_classe, status_label};
use crate::api::admin::resumo_dashboard;
use crate::domain::{DashboardResumo, DiaAcesso, ItemRanking, LeadResumo};

const IC_OLHO: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>"#;
const IC_CHAT: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/><path d="M8 9h8M8 13h5"/></svg>"#;
const IC_COPO: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M6 4h12l-1.2 15A2 2 0 0 1 14.8 21H9.2a2 2 0 0 1-2-1.9zM5 4h14"/></svg>"#;
const IC_TREND: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 17l6-6 4 4 8-8"/><path d="M17 7h4v4"/></svg>"#;

const MES_NOME: [&str; 12] = [
    "Janeiro",
    "Fevereiro",
    "Março",
    "Abril",
    "Maio",
    "Junho",
    "Julho",
    "Agosto",
    "Setembro",
    "Outubro",
    "Novembro",
    "Dezembro",
];

/// Dias no mês (com bissexto), para limitar o seletor de dia.
fn dias_no_mes(ano: i32, mes: i32) -> i32 {
    match mes {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if (ano % 4 == 0 && ano % 100 != 0) || ano % 400 == 0 => 29,
        2 => 28,
        _ => 31,
    }
}

/// Dashboard do painel: filtro de período + KPIs + gráficos + leads recentes,
/// tudo em tempo real. Dados buscados no cliente (após a hidratação).
#[component]
pub fn AdminDashboard() -> impl IntoView {
    let dados = RwSignal::new(None::<Result<DashboardResumo, ServerFnError>>);
    // 0 = ainda não resolvido (o servidor devolve o período padrão = mês atual).
    let ano = RwSignal::new(0i32);
    let mes = RwSignal::new(None::<i32>);
    let dia = RwSignal::new(None::<i32>);
    let ano_atual = RwSignal::new(0i32);

    // Carga inicial (uma vez): resolve o período padrão no servidor.
    Effect::new(move |_| {
        if ano.get_untracked() != 0 {
            return;
        }
        spawn_local(async move {
            match resumo_dashboard(None, None, None).await {
                Ok(res) => {
                    mes.set(res.sel_mes);
                    dia.set(res.sel_dia);
                    ano_atual.set(res.ano_atual);
                    ano.set(res.sel_ano); // ≠ 0 → dispara a carga pelo efeito de filtro
                }
                Err(e) => dados.set(Some(Err(e))),
            }
        });
    });

    // Mantém o dia válido para o mês/ano escolhidos.
    Effect::new(move |_| {
        let a = ano.get();
        match mes.get() {
            Some(m) => {
                if let Some(d) = dia.get_untracked() {
                    let max = dias_no_mes(a, m);
                    if d > max {
                        dia.set(Some(max));
                    }
                }
            }
            None => {
                if dia.get_untracked().is_some() {
                    dia.set(None);
                }
            }
        }
    });

    // Recarrega os dados sempre que o filtro muda (após a resolução inicial).
    Effect::new(move |_| {
        let a = ano.get();
        let m = mes.get();
        let d = dia.get();
        if a == 0 {
            return;
        }
        dados.set(None);
        spawn_local(async move {
            dados.set(Some(resumo_dashboard(Some(a), m, d).await));
        });
    });

    // ---- Tempo real: polling silencioso a cada 15s ----
    let tick = RwSignal::new(0u32);
    // Timer só no cliente (Effects não rodam no SSR); um único intervalo,
    // encerrado quando o componente sai de tela.
    Effect::new(move |_| {
        if let Ok(handle) = set_interval_with_handle(
            move || tick.update(|t| *t = t.wrapping_add(1)),
            std::time::Duration::from_secs(15),
        ) {
            on_cleanup(move || handle.clear());
        }
    });
    // A cada tick, recarrega EM SILÊNCIO (sem "Carregando"), mantendo o período
    // atual; em caso de erro, preserva os dados já exibidos.
    Effect::new(move |_| {
        tick.get();
        let a = ano.get_untracked();
        if a == 0 {
            return;
        }
        let (m, d) = (mes.get_untracked(), dia.get_untracked());
        spawn_local(async move {
            if let Ok(res) = resumo_dashboard(Some(a), m, d).await {
                dados.set(Some(Ok(res)));
            }
        });
    });

    view! {
        <header class="admin-head">
            <h1 class="admin-head__title">"Dashboard"</h1>
            <p class="admin-head__sub">
                "Visão geral do negócio · "
                <span class="dash-live">
                    <span class="dash-live__dot"></span>
                    "ao vivo"
                </span>
            </p>
        </header>

        {move || {
            let amax = ano_atual.get();
            (amax != 0)
                .then(|| {
                    view! {
                        <div class="dash-filtro">
                            <select
                                class="admin-input"
                                prop:value=move || ano.get().to_string()
                                on:change=move |ev| {
                                    if let Ok(y) = event_target_value(&ev).parse::<i32>() {
                                        ano.set(y);
                                    }
                                }
                            >
                                {move || {
                                    let amax = ano_atual.get();
                                    ((amax - 4)..=amax)
                                        .rev()
                                        .map(|y| view! { <option value=y.to_string()>{y}</option> })
                                        .collect_view()
                                }}
                            </select>
                            <select
                                class="admin-input"
                                prop:value=move || {
                                    mes.get().map(|m| m.to_string()).unwrap_or_default()
                                }
                                on:change=move |ev| {
                                    let m = event_target_value(&ev).parse::<i32>().ok();
                                    mes.set(m);
                                    if m.is_none() {
                                        dia.set(None);
                                    }
                                }
                            >
                                <option value="">"Todos os meses"</option>
                                {(1..=12)
                                    .map(|m| {
                                        view! {
                                            <option value=m.to_string()>{MES_NOME[m as usize - 1]}</option>
                                        }
                                    })
                                    .collect_view()}
                            </select>
                            <select
                                class="admin-input"
                                prop:disabled=move || mes.get().is_none()
                                prop:value=move || {
                                    dia.get().map(|d| d.to_string()).unwrap_or_default()
                                }
                                on:change=move |ev| {
                                    dia.set(event_target_value(&ev).parse::<i32>().ok());
                                }
                            >
                                <option value="">"Todos os dias"</option>
                                {move || {
                                    let qt = mes.get().map(|m| dias_no_mes(ano.get(), m)).unwrap_or(0);
                                    (1..=qt)
                                        .map(|d| view! { <option value=d.to_string()>{d}</option> })
                                        .collect_view()
                                }}
                            </select>
                        </div>
                    }
                })
        }}

        {move || match dados.get() {
            None => view! { <p class="admin-status">"Carregando..."</p> }.into_any(),
            Some(Err(_)) => {
                view! { <p class="admin-status">"Não foi possível carregar o dashboard."</p> }
                    .into_any()
            }
            Some(Ok(r)) => painel(r),
        }}
    }
}

fn painel(r: DashboardResumo) -> AnyView {
    let total: i64 = r.acessos_serie.iter().map(|d| d.total).sum();
    let (acessos_lbl, chart_meta): (&str, &str) = match (r.sel_mes, r.sel_dia) {
        (_, Some(_)) => ("Acessos no dia", "Por hora"),
        (Some(_), None) => ("Acessos no mês", "Por dia"),
        (None, _) => ("Acessos no ano", "Por mês"),
    };
    view! {
        <div class="dash-cards">
            {card(IC_OLHO, &fmt_milhar(r.acessos_mes), acessos_lbl, badge_delta(r.acessos_delta))}
            {card(IC_CHAT, &fmt_milhar(r.total_leads), "Leads", badge_delta(r.leads_delta))}
            {card(
                IC_COPO,
                &fmt_milhar(r.produtos_total),
                "Produtos cadastrados",
                badge_neutro(&format!("{} ativos", r.produtos_ativos)),
            )}
            {card(
                IC_TREND,
                &fmt_pct1(r.taxa_conversao),
                "Taxa de conversão",
                badge_delta(r.conversao_delta),
            )}
        </div>

        <section class="admin-card dash-chart">
            <div class="dash-chart__head">
                <div>
                    <h2 class="dash-card__head-title">"Acessos ao site"</h2>
                    <p class="admin-card__meta">{chart_meta}</p>
                </div>
                <span class="dash-chart__total">{fmt_milhar(total)}</span>
            </div>
            {grafico_barras(r.acessos_serie)}
        </section>

        <div class="dash-row">
            <section class="admin-card">
                <h2 class="dash-card__head-title">"Produtos mais vistos"</h2>
                <ProdutosVistos itens=r.produtos_vistos/>
            </section>
        </div>

        <section class="admin-card">
            <h2 class="dash-card__head-title">"Leads recentes"</h2>
            {tabela_recentes(r.recentes)}
        </section>
    }
    .into_any()
}

fn card(icone: &'static str, valor: &str, rotulo: &str, badge: AnyView) -> AnyView {
    let valor = valor.to_string();
    let rotulo = rotulo.to_string();
    view! {
        <div class="dash-card">
            <div class="dash-card__top">
                <span class="dash-card__ic" inner_html=icone></span>
                {badge}
            </div>
            <span class="dash-card__num">{valor}</span>
            <span class="dash-card__lbl">{rotulo}</span>
        </div>
    }
    .into_any()
}

fn badge_delta(d: Option<i32>) -> AnyView {
    match d {
        Some(v) if v >= 0 => {
            view! { <span class="dash-badge dash-badge--up">{format!("+{v}%")}</span> }.into_any()
        }
        Some(v) => {
            view! { <span class="dash-badge dash-badge--down">{format!("{v}%")}</span> }.into_any()
        }
        None => view! { <span class="dash-badge">"—"</span> }.into_any(),
    }
}

fn badge_neutro(texto: &str) -> AnyView {
    let texto = texto.to_string();
    view! { <span class="dash-badge">{texto}</span> }.into_any()
}

fn grafico_barras(dias: Vec<DiaAcesso>) -> AnyView {
    let max = dias.iter().map(|d| d.total).max().unwrap_or(0).max(1);
    let muitos = dias.len() > 12;
    view! {
        <div class="chart-bars" class:chart-bars--densa=muitos>
            {dias
                .into_iter()
                .map(|d| {
                    let alta = d.total == max && d.total > 0;
                    let h = (d.total as f64 / max as f64 * 100.0).max(3.0);
                    let total = d.total;
                    view! {
                        <div class="chart-col">
                            <div class="chart-col__track">
                                <div class="chart-col__fill" style=format!("height:{h:.1}%")>
                                    <span class="chart-col__v">{fmt_milhar(total)}</span>
                                    <span class="chart-col__bar" class:is-alta=alta></span>
                                </div>
                            </div>
                            <span class="chart-col__d">{d.rotulo}</span>
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
    .into_any()
}

/// Lista de "produtos mais vistos" paginada (10 por página).
#[component]
fn ProdutosVistos(itens: Vec<ItemRanking>) -> impl IntoView {
    const POR_PAGINA: usize = 10;
    if itens.is_empty() {
        return vazio();
    }
    let n_paginas = itens.len().div_ceil(POR_PAGINA);
    let itens = StoredValue::new(itens);
    let pagina = RwSignal::new(0usize);

    view! {
        <ul class="prod-list">
            {move || {
                let ini = pagina.get() * POR_PAGINA;
                itens
                    .with_value(|v| {
                        v.iter()
                            .skip(ini)
                            .take(POR_PAGINA)
                            .enumerate()
                            .map(|(i, it)| {
                                let cor = (ini + i) % 4;
                                view! {
                                    <li class="prod-row">
                                        <span class=format!("prod-row__sw prod-row__sw--{cor}")></span>
                                        <span class="prod-row__info">
                                            <strong>{it.rotulo.clone()}</strong>
                                            <span class="prod-row__sub">
                                                {format!("{} visualizações", fmt_milhar(it.total))}
                                            </span>
                                        </span>
                                    </li>
                                }
                            })
                            .collect_view()
                    })
            }}
        </ul>
        {move || {
            (n_paginas > 1)
                .then(|| {
                    view! {
                        <div class="prod-paginacao">
                            <button
                                type="button"
                                class="btn btn--ghost btn--sm"
                                prop:disabled=move || pagina.get() == 0
                                on:click=move |_| pagina.update(|p| *p = p.saturating_sub(1))
                            >
                                "‹ Anterior"
                            </button>
                            <span class="prod-paginacao__info">
                                {move || format!("{} de {}", pagina.get() + 1, n_paginas)}
                            </span>
                            <button
                                type="button"
                                class="btn btn--ghost btn--sm"
                                prop:disabled=move || pagina.get() + 1 >= n_paginas
                                on:click=move |_| {
                                    pagina
                                        .update(|p| {
                                            if *p + 1 < n_paginas {
                                                *p += 1;
                                            }
                                        })
                                }
                            >
                                "Próximo ›"
                            </button>
                        </div>
                    }
                })
        }}
    }
    .into_any()
}

fn vazio() -> AnyView {
    view! { <p class="admin-status">"Sem dados no período."</p> }.into_any()
}

fn tabela_recentes(itens: Vec<LeadResumo>) -> AnyView {
    if itens.is_empty() {
        return view! { <p class="admin-status">"Nenhum lead no período."</p> }.into_any();
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

/// Inteiro com separador de milhar (pt-BR: 8.420).
fn fmt_milhar(n: i64) -> String {
    let neg = n < 0;
    let digitos: Vec<char> = n.abs().to_string().chars().collect();
    let mut rev = String::new();
    for (i, c) in digitos.iter().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            rev.push('.');
        }
        rev.push(*c);
    }
    let mut s: String = rev.chars().rev().collect();
    if neg {
        s.insert(0, '-');
    }
    s
}

/// Percentual com uma casa decimal (pt-BR: 6,1%).
fn fmt_pct1(v: f64) -> String {
    format!("{v:.1}%").replace('.', ",")
}
