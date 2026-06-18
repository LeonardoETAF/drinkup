use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_params_map};
use uuid::Uuid;

use crate::api::eventos_admin::{obter_evento_admin, salvar_evento};
use crate::domain::EventoForm;

/// Formulário de criação/edição de categoria do carrossel.
#[component]
pub fn AdminEventoForm() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params
            .read()
            .get("id")
            .and_then(|s| Uuid::parse_str(&s).ok())
    };
    let editando = move || id().is_some();

    let titulo = RwSignal::new(String::new());
    let cor = RwSignal::new("#c8d400".to_string());
    let ordem = RwSignal::new("0".to_string());
    let ativo = RwSignal::new(true);
    let imagem_url = RwSignal::new(None::<String>);
    let enviando_img = RwSignal::new(false);
    let erro_img = RwSignal::new(None::<String>);

    Effect::new(move |_| {
        let Some(eid) = id() else { return };
        spawn_local(async move {
            if let Ok(Some(f)) = obter_evento_admin(eid).await {
                titulo.set(f.titulo);
                cor.set(f.cor.unwrap_or_else(|| "#c8d400".to_string()));
                ordem.set(f.ordem.to_string());
                ativo.set(f.ativo);
                imagem_url.set(f.imagem_url);
            }
        });
    });

    let salvar = Action::new(|form: &EventoForm| {
        let f = form.clone();
        async move { salvar_evento(f).await }
    });

    let navegar = use_navigate();
    Effect::new(move |_| {
        if matches!(salvar.value().get(), Some(Ok(_))) {
            navegar("/admin/eventos", Default::default());
        }
    });

    let erro = move || match salvar.value().get() {
        Some(Err(e)) => Some(e.to_string()),
        _ => None,
    };
    let salvando = salvar.pending();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let form = EventoForm {
            id: id(),
            titulo: titulo.get_untracked().trim().to_string(),
            cor: Some(cor.get_untracked()),
            imagem_url: imagem_url.get_untracked(),
            ordem: ordem.get_untracked().trim().parse::<i32>().unwrap_or(0),
            ativo: ativo.get_untracked(),
        };
        salvar.dispatch(form);
    };

    view! {
        <header class="admin-head admin-head--row">
            <div>
                <h1 class="admin-head__title">
                    {move || if editando() { "Editar categoria" } else { "Nova categoria" }}
                </h1>
                <p class="admin-head__sub">"Categoria do carrossel da home"</p>
            </div>
            <a class="btn btn--ghost" href="/admin/eventos">"Voltar"</a>
        </header>

        <form class="admin-card admin-form" on:submit=on_submit>
            <label class="field">
                <span class="field__label">"Nome"</span>
                <input
                    class="admin-input"
                    type="text"
                    prop:value=move || titulo.get()
                    on:input=move |ev| titulo.set(event_target_value(&ev))
                />
            </label>

            <label class="field">
                <span class="field__label">"Imagem do card (JPG, PNG ou WEBP, até 5MB)"</span>
                <input
                    class="admin-input"
                    type="file"
                    accept="image/png,image/jpeg,image/webp"
                    on:change=move |ev| {
                        #[cfg(feature = "hydrate")]
                        {
                            use wasm_bindgen::JsCast;
                            if let Some(input) = ev
                                .target()
                                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                            {
                                if let Some(file) = input.files().and_then(|f| f.get(0)) {
                                    let fd = web_sys::FormData::new().unwrap();
                                    let _ = fd.append_with_blob("imagem", &file);
                                    enviando_img.set(true);
                                    erro_img.set(None);
                                    leptos::task::spawn_local(async move {
                                        let r = async {
                                            let req = gloo_net::http::Request::post("/upload-imagem")
                                                .body(fd)
                                                .map_err(|_| ())?;
                                            let resp = req.send().await.map_err(|_| ())?;
                                            if resp.ok() {
                                                resp.text().await.map_err(|_| ())
                                            } else {
                                                Err(())
                                            }
                                        }
                                            .await;
                                        match r {
                                            Ok(url) => imagem_url.set(Some(url)),
                                            Err(()) => {
                                                erro_img
                                                    .set(Some("Não foi possível enviar a imagem.".to_string()))
                                            }
                                        }
                                        enviando_img.set(false);
                                    });
                                }
                            }
                        }
                        #[cfg(not(feature = "hydrate"))]
                        let _ = &ev;
                    }
                />
                {move || {
                    enviando_img.get().then(|| view! { <span class="admin-status">"Enviando..."</span> })
                }}
                {move || {
                    imagem_url
                        .get()
                        .map(|u| view! { <img class="form-preview" src=u alt="Prévia"/> })
                }}
                {move || erro_img.get().map(|m| view! { <span class="field__erro">{m}</span> })}
            </label>

            <div class="admin-form__grid">
                <label class="field">
                    <span class="field__label">"Cor"</span>
                    <input
                        class="admin-input admin-input--cor"
                        type="color"
                        prop:value=move || cor.get()
                        on:input=move |ev| cor.set(event_target_value(&ev))
                    />
                </label>
                <label class="field">
                    <span class="field__label">"Ordem"</span>
                    <input
                        class="admin-input"
                        type="number"
                        prop:value=move || ordem.get()
                        on:input=move |ev| ordem.set(event_target_value(&ev))
                    />
                </label>
            </div>

            <div class="admin-form__checks">
                <label class="login-check">
                    <input
                        type="checkbox"
                        prop:checked=move || ativo.get()
                        on:change=move |ev| ativo.set(event_target_checked(&ev))
                    />
                    <span>"Visível no site"</span>
                </label>
            </div>

            {move || erro().map(|m| view! { <p class="orc-form__erro">{m}</p> })}

            <div class="admin-form__acoes">
                <a class="btn btn--ghost" href="/admin/eventos">"Cancelar"</a>
                <button type="submit" class="btn btn--primary" prop:disabled=move || salvando.get()>
                    {move || if salvando.get() { "Salvando..." } else { "Salvar categoria" }}
                </button>
            </div>
        </form>
    }
}
