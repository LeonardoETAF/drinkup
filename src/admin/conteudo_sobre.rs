use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::admin::upload_card::CartaoUpload;
use crate::api::quem_somos_admin::{obter_quem_somos_form, salvar_quem_somos};
use crate::domain::QuemSomosForm;

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
    let depoimentos = RwSignal::new(String::new());

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
                depoimentos.set(f.depoimentos);
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
        Some(Err(e)) => Some(e.to_string()),
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
            depoimentos: depoimentos.get_untracked(),
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
                <label class="field">
                    <span class="field__label">
                        "Um por linha — formato: texto | autor. Com mais de um, vira swipe na página."
                    </span>
                    <textarea
                        class="admin-input"
                        rows="6"
                        placeholder="Produtos de ótima qualidade... | Dieferson Schaffer · Personalização Canábis"
                        prop:value=move || depoimentos.get()
                        on:input=move |ev| depoimentos.set(event_target_value(&ev))
                    ></textarea>
                </label>
            </fieldset>

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
