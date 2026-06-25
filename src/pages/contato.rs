use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use uuid::Uuid;

use crate::api::config::obter_contato;
use crate::api::orcamento::enviar_orcamento;
use crate::components::Seo;
use crate::domain::{mascara_telefone, Configuracoes, ItemOrcamento, NovoOrcamento};

const ICON_FONE: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M22 16.9v3a2 2 0 0 1-2.2 2 19.8 19.8 0 0 1-8.6-3 19.5 19.5 0 0 1-6-6 19.8 19.8 0 0 1-3.1-8.7A2 2 0 0 1 4.1 2h3a2 2 0 0 1 2 1.7c.1 1 .4 1.9.7 2.8a2 2 0 0 1-.4 2.1L8.1 9.9a16 16 0 0 0 6 6l1.3-1.3a2 2 0 0 1 2.1-.5c.9.4 1.8.6 2.8.8a2 2 0 0 1 1.7 2z"/></svg>"#;
const ICON_PIN: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0z"/><circle cx="12" cy="10" r="3"/></svg>"#;
const ICON_RELOGIO: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="9"/><path d="M12 7v5l3 2"/></svg>"#;

/// Página de contato/orçamento. Validação no cliente (UX) + revalidação e
/// anti-spam no servidor. Pré-preenche o produto quando vem de `?produto&pid`.
#[component]
pub fn ContatoPage() -> impl IntoView {
    let q = use_query_map();
    let init = q.get_untracked();
    let pre_nome = init
        .get("produto")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty());
    let pre_pid = init.get("pid").and_then(|s| Uuid::parse_str(&s).ok());

    let nome = RwSignal::new(String::new());
    let contato = RwSignal::new(String::new());
    let mensagem = RwSignal::new(
        pre_nome
            .as_ref()
            .map(|n| format!("Olá! Tenho interesse no produto: {n}."))
            .unwrap_or_default(),
    );
    let honeypot = RwSignal::new(String::new());
    let erro_nome = RwSignal::new(false);
    let erro_contato = RwSignal::new(false);
    let erro_mensagem = RwSignal::new(false);

    let enviar = Action::new(|input: &(NovoOrcamento, String)| {
        let (dados, hp) = input.clone();
        async move { enviar_orcamento(dados, hp).await }
    });

    // Guardado em StoredValue (Copy) para que o handler de submit seja `Fn`.
    let pre = StoredValue::new((pre_pid, pre_nome.clone()));
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let n = nome.get().trim().to_string();
        let c = contato.get().trim().to_string();
        let m = mensagem.get().trim().to_string();
        erro_nome.set(n.chars().count() < 2);
        erro_contato.set(!crate::domain::telefone_valido(&c));
        erro_mensagem.set(m.is_empty());
        if erro_nome.get_untracked() || erro_contato.get_untracked() || erro_mensagem.get_untracked()
        {
            return;
        }
        let (pid, desc) = pre.get_value();
        let itens = if pid.is_some() || desc.is_some() {
            vec![ItemOrcamento {
                produto_id: pid,
                descricao: desc,
                quantidade: 1,
            }]
        } else {
            vec![]
        };
        let dados = NovoOrcamento {
            nome: n,
            contato: c,
            email: None,
            mensagem: (!m.is_empty()).then_some(m),
            itens,
        };
        enviar.dispatch((dados, honeypot.get_untracked()));
    };

    let sucesso = move || matches!(enviar.value().get(), Some(Ok(())));
    let erro_msg = move || match enviar.value().get() {
        Some(Err(e)) => Some(crate::components::mensagem_erro(&e)),
        _ => None,
    };
    let enviando = enviar.pending();

    // Dados de contato vêm das configurações (SSR; sem sessão = seguro).
    let info = Resource::new(|| (), |_| async move { obter_contato().await });

    view! {
        <Seo
            titulo="Contato e orçamento"
            descricao="Fale com a DRINK UP: peça um orçamento de copos personalizados para seu \
            evento ou tire suas dúvidas. Atendimento por WhatsApp em Maringá-PR."
            caminho="/contato"
        />
        <section class="contato-hero">
            <div class="container">
                <span class="kicker">"Fale conosco"</span>
                <h1 class="contato-hero__title">"Contato"</h1>
                <p class="contato-hero__sub">
                    "Alguma dúvida, orçamento ou parceria? Basta nos enviar uma mensagem."
                </p>
            </div>
        </section>

        <section class="container contato">
            <div class="form-card">
                <Show
                    when=sucesso
                    fallback=move || {
                        view! {
                            <form class="orc-form" on:submit=on_submit novalidate>
                                <div class="orc-grid">
                                    <label class="field">
                                        <span class="field__label">"Nome"</span>
                                        <input
                                            type="text"
                                            placeholder="Seu nome completo"
                                            class:field--erro=move || erro_nome.get()
                                            prop:value=move || nome.get()
                                            on:input=move |ev| nome.set(event_target_value(&ev))
                                        />
                                        <Show when=move || erro_nome.get()>
                                            <span class="field__erro">"Informe seu nome."</span>
                                        </Show>
                                    </label>
                                    <label class="field">
                                        <span class="field__label">"WhatsApp"</span>
                                        <input
                                            type="tel"
                                            inputmode="numeric"
                                            maxlength="15"
                                            placeholder="(00) 00000-0000"
                                            class:field--erro=move || erro_contato.get()
                                            prop:value=move || contato.get()
                                            on:input=move |ev| {
                                                contato.set(mascara_telefone(&event_target_value(&ev)))
                                            }
                                        />
                                        <Show when=move || erro_contato.get()>
                                            <span class="field__erro">
                                                "Informe um WhatsApp válido."
                                            </span>
                                        </Show>
                                    </label>
                                </div>
                                <label class="field">
                                    <span class="field__label">"Mensagem"</span>
                                    <textarea
                                        rows="5"
                                        placeholder="Conte pra gente o que você precisa..."
                                        class:field--erro=move || erro_mensagem.get()
                                        prop:value=move || mensagem.get()
                                        on:input=move |ev| mensagem.set(event_target_value(&ev))
                                    ></textarea>
                                    <Show when=move || erro_mensagem.get()>
                                        <span class="field__erro">"Escreva sua mensagem."</span>
                                    </Show>
                                </label>
                                <input
                                    class="hp-field"
                                    type="text"
                                    name="empresa_site"
                                    tabindex="-1"
                                    autocomplete="off"
                                    aria-hidden="true"
                                    prop:value=move || honeypot.get()
                                    on:input=move |ev| honeypot.set(event_target_value(&ev))
                                />
                                {move || {
                                    erro_msg().map(|m| view! { <p class="orc-form__erro">{m}</p> })
                                }}
                                <button
                                    type="submit"
                                    class="btn btn--primary btn--block"
                                    prop:disabled=move || enviando.get()
                                >
                                    {move || {
                                        if enviando.get() { "Enviando..." } else { "Enviar mensagem" }
                                    }}
                                </button>
                            </form>
                        }
                    }
                >
                    <div class="orc-sucesso">
                        <h2>"Mensagem enviada!"</h2>
                        <p>"Recebemos seu pedido e em breve entraremos em contato pelo WhatsApp."</p>
                        <a class="btn btn--ghost" href="/produtos">"Ver mais produtos"</a>
                    </div>
                </Show>
            </div>

            <div class="contato-info">
                <Suspense fallback=move || cards_contato(Configuracoes::default())>
                    {move || {
                        info.get().map(|r| cards_contato(r.unwrap_or_default()))
                    }}
                </Suspense>
            </div>
        </section>
    }
}

/// Renderiza os três cards de contato a partir das configurações da loja.
fn cards_contato(c: Configuracoes) -> impl IntoView {
    let horario_semana = format!("Seg. a Sex.: {}", c.horario_semana);
    let horario_sabado = format!("Sábado: {}", c.horario_sabado);
    let horario_domingo = format!("Domingo: {}", c.horario_domingo);
    view! {
        <div class="info-card">
            <span class="info-card__icon info-card__icon--lime" inner_html=ICON_FONE></span>
            <h3>"Telefone"</h3>
            <p>{c.telefone}</p>
            <p class="info-card__muted">"WhatsApp disponível"</p>
        </div>
        <div class="info-card">
            <span class="info-card__icon info-card__icon--pink" inner_html=ICON_PIN></span>
            <h3>"Endereço"</h3>
            <p>{c.endereco}</p>
        </div>
        <div class="info-card">
            <span class="info-card__icon info-card__icon--roxo" inner_html=ICON_RELOGIO></span>
            <h3>"Horários"</h3>
            <p>{horario_semana}</p>
            <p class="info-card__muted">{horario_sabado}</p>
            <p class="info-card__muted">{horario_domingo}</p>
        </div>
    }
}
