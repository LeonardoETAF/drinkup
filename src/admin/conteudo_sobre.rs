use leptos::prelude::*;
use leptos::task::spawn_local;

use super::modal::ModalConfirmacao;
use crate::admin::upload_card::CartaoUpload;
use crate::api::quem_somos_admin::{obter_quem_somos_form, salvar_quem_somos};
use crate::domain::QuemSomosForm;

const IC_DEL: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M10 11v6M14 11v6"/></svg>"#;

/// Um depoimento em edição: id estável + sinais de texto e autor.
type Depoimento = (usize, RwSignal<String>, RwSignal<String>);

/// Edição do conteúdo da página "Quem Somos".
#[component]
pub fn AdminConteudoQuemSomos() -> impl IntoView {
    let stat_titulo = RwSignal::new(String::new());
    let stat_destaque = RwSignal::new(String::new());
    let stat_texto = RwSignal::new(String::new());
    let video = RwSignal::new(None::<String>);
    let missao = RwSignal::new(String::new());
    let visao = RwSignal::new(String::new());
    let valores = RwSignal::new(String::new());
    let foto1 = RwSignal::new(None::<String>);
    let foto2 = RwSignal::new(None::<String>);
    // Lista de depoimentos separados (cada um com seus próprios sinais).
    let depoimentos = RwSignal::new(Vec::<Depoimento>::new());
    let proximo_id = RwSignal::new(0usize);
    // Depoimento aguardando confirmação de remoção (id), ou None.
    let dep_pendente = RwSignal::new(None::<usize>);

    let adicionar = move |texto: String, autor: String| {
        let id = proximo_id.get_untracked();
        proximo_id.set(id + 1);
        depoimentos.update(|v| v.push((id, RwSignal::new(texto), RwSignal::new(autor))));
    };

    Effect::new(move |_| {
        spawn_local(async move {
            if let Ok(f) = obter_quem_somos_form().await {
                stat_titulo.set(f.stat_titulo);
                stat_destaque.set(f.stat_destaque);
                stat_texto.set(f.stat_texto);
                video.set(f.video_url);
                missao.set(f.missao);
                visao.set(f.visao);
                valores.set(f.valores);
                foto1.set(f.foto1_url);
                foto2.set(f.foto2_url);
                depoimentos.set(Vec::new());
                proximo_id.set(0);
                for linha in f.depoimentos.lines().filter(|l| !l.trim().is_empty()) {
                    let (t, a) = match linha.split_once('|') {
                        Some((t, a)) => (t.trim().to_string(), a.trim().to_string()),
                        None => (linha.trim().to_string(), String::new()),
                    };
                    adicionar(t, a);
                }
            }
        });
    });

    let salvar = Action::new(|f: &QuemSomosForm| {
        let f = f.clone();
        async move { salvar_quem_somos(f).await }
    });
    let salvando = salvar.pending();
    let salvo = move || matches!(salvar.value().get(), Some(Ok(())));
    let erro = move || match salvar.value().get() {
        Some(Err(e)) => Some(crate::components::mensagem_erro(&e)),
        _ => None,
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        salvar.dispatch(QuemSomosForm {
            stat_titulo: stat_titulo.get_untracked(),
            stat_destaque: stat_destaque.get_untracked(),
            stat_texto: stat_texto.get_untracked(),
            video_url: video.get_untracked(),
            missao: missao.get_untracked(),
            visao: visao.get_untracked(),
            valores: valores.get_untracked(),
            foto1_url: foto1.get_untracked(),
            foto2_url: foto2.get_untracked(),
            depoimentos: depoimentos
                .get_untracked()
                .iter()
                .map(|(_, t, a)| {
                    let t = t.get_untracked();
                    let a = a.get_untracked();
                    let (t, a) = (t.trim(), a.trim());
                    if a.is_empty() {
                        t.to_string()
                    } else {
                        format!("{t} | {a}")
                    }
                })
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>()
                .join("\n"),
        });
    };

    view! {
        <header class="admin-head">
            <h1 class="admin-head__title">"Conteúdo de Quem Somos"</h1>
            <p class="admin-head__sub">"Destaque, vídeo, missão/visão/valores, fotos e depoimentos"</p>
        </header>

        <form class="admin-config" on:submit=on_submit>
            <fieldset class="admin-card admin-fieldset">
                <legend class="admin-fieldset__titulo">"Destaque (estatística)"</legend>
                <div class="admin-form__grid">
                    {campo("Título (ex.: +25 mil unidades)", stat_titulo)}
                    {campo("Destaque (ex.: todos os dias)", stat_destaque)}
                </div>
                {area("Texto de apoio", stat_texto, 3, "Uma equipe de mais de 20 profissionais...")}
            </fieldset>

            <fieldset class="admin-card admin-fieldset">
                <legend class="admin-fieldset__titulo">"Vídeo institucional"</legend>
                {campo_opt(
                    "Link do vídeo (YouTube, Vimeo ou arquivo .mp4) — roda na página",
                    video,
                )}
            </fieldset>

            <fieldset class="admin-card admin-fieldset">
                <legend class="admin-fieldset__titulo">"Missão, Visão e Valores"</legend>
                {area("Missão", missao, 3, "Nossa maior missão é...")}
                {area("Visão", visao, 3, "Ter uma linha completa...")}
                {area("Valores", valores, 5, "Sabemos que todos os produtos...")}
            </fieldset>

            <fieldset class="admin-card admin-fieldset">
                <legend class="admin-fieldset__titulo">"Fotos"</legend>
                <div class="admin-form__grid">
                    <div class="field">
                        <span class="field__label">"Foto 1 (ex.: equipe de produção)"</span>
                        <CartaoUpload url=foto1 dica="PNG/JPG/WEBP · paisagem (4:3) · até 5MB"/>
                    </div>
                    <div class="field">
                        <span class="field__label">"Foto 2 (ex.: vista da fábrica)"</span>
                        <CartaoUpload url=foto2 dica="PNG/JPG/WEBP · paisagem (4:3) · até 5MB"/>
                    </div>
                </div>
            </fieldset>

            <fieldset class="admin-card admin-fieldset">
                <legend class="admin-fieldset__titulo">"Depoimentos de clientes"</legend>
                <p class="admin-head__sub">
                    "Cada depoimento separado. Com mais de um, vira swipe na página."
                </p>
                <div class="depo-editor">
                    {move || {
                        depoimentos
                            .get()
                            .into_iter()
                            .map(|(id, texto, autor)| {
                                view! {
                                    <div class="depo-edit">
                                        <textarea
                                            class="admin-input"
                                            rows="2"
                                            placeholder="Depoimento do cliente..."
                                            prop:value=move || texto.get()
                                            on:input=move |ev| texto.set(event_target_value(&ev))
                                        ></textarea>
                                        <div class="depo-edit__rodape">
                                            <input
                                                class="admin-input"
                                                type="text"
                                                placeholder="Autor (ex.: Nome · Empresa)"
                                                prop:value=move || autor.get()
                                                on:input=move |ev| autor.set(event_target_value(&ev))
                                            />
                                            <button
                                                type="button"
                                                class="icon-btn icon-btn--danger"
                                                title="Remover depoimento"
                                                inner_html=IC_DEL
                                                on:click=move |_| dep_pendente.set(Some(id))
                                            ></button>
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()
                    }}
                    <button
                        type="button"
                        class="btn btn--ghost depo-editor__add"
                        on:click=move |_| adicionar(String::new(), String::new())
                    >
                        "+ Adicionar depoimento"
                    </button>
                </div>
            </fieldset>

            <ModalConfirmacao
                aberto=Signal::derive(move || dep_pendente.get().is_some())
                mensagem="Deseja remover este depoimento?"
                confirmar_texto="Remover"
                ao_cancelar=Callback::new(move |()| dep_pendente.set(None))
                ao_confirmar=Callback::new(move |()| {
                    if let Some(id) = dep_pendente.get_untracked() {
                        depoimentos.update(|v| v.retain(|(i, _, _)| *i != id));
                    }
                    dep_pendente.set(None);
                })
            />

            {move || erro().map(|m| view! { <p class="orc-form__erro">{m}</p> })}
            {move || {
                (salvo() && erro().is_none())
                    .then(|| view! { <p class="admin-ok">"Conteúdo salvo."</p> })
            }}

            <div class="admin-form__acoes">
                <a class="btn btn--ghost" href="/admin">"Cancelar"</a>
                <button type="submit" class="btn btn--primary" prop:disabled=move || salvando.get()>
                    {move || if salvando.get() { "Salvando..." } else { "Salvar conteúdo" }}
                </button>
            </div>
        </form>
    }
}

fn campo(rotulo: &'static str, sinal: RwSignal<String>) -> impl IntoView {
    view! {
        <label class="field">
            <span class="field__label">{rotulo}</span>
            <input
                class="admin-input"
                type="text"
                prop:value=move || sinal.get()
                on:input=move |ev| sinal.set(event_target_value(&ev))
            />
        </label>
    }
}

/// Campo de texto que liga a um `Option<String>` (vazio = `None`).
fn campo_opt(rotulo: &'static str, sinal: RwSignal<Option<String>>) -> impl IntoView {
    view! {
        <label class="field">
            <span class="field__label">{rotulo}</span>
            <input
                class="admin-input"
                type="url"
                prop:value=move || sinal.get().unwrap_or_default()
                on:input=move |ev| {
                    let v = event_target_value(&ev);
                    sinal.set(if v.trim().is_empty() { None } else { Some(v) });
                }
            />
        </label>
    }
}

fn area(
    rotulo: &'static str,
    sinal: RwSignal<String>,
    linhas: u32,
    exemplo: &'static str,
) -> impl IntoView {
    view! {
        <label class="field">
            <span class="field__label">{rotulo}</span>
            <textarea
                class="admin-input"
                rows=linhas
                placeholder=exemplo
                prop:value=move || sinal.get()
                on:input=move |ev| sinal.set(event_target_value(&ev))
            ></textarea>
        </label>
    }
}
