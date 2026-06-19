use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_params_map};
use uuid::Uuid;

use crate::api::parceiros_admin::{obter_parceiro_admin, salvar_parceiro};
use crate::domain::ParceiroForm;

/// Formulário de criação/edição de parceiro.
#[component]
pub fn AdminParceiroForm() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params
            .read()
            .get("id")
            .and_then(|s| Uuid::parse_str(&s).ok())
    };
    let editando = move || id().is_some();

    let nome = RwSignal::new(String::new());
    let site_url = RwSignal::new(String::new());
    let descricao = RwSignal::new(String::new());
    let ordem = RwSignal::new(String::new());
    let ativo = RwSignal::new(true);
    let logo_url = RwSignal::new(None::<String>);
    let enviando_img = RwSignal::new(false);
    let erro_img = RwSignal::new(None::<String>);
    let cor = RwSignal::new("#ff0070".to_string());
    let tagline = RwSignal::new(String::new());
    // Imagens dos produtos (URLs) exibidas como swipe na página do parceiro.
    let imagens = RwSignal::new(Vec::<String>::new());
    let enviando_prod = RwSignal::new(false);

    Effect::new(move |_| {
        let Some(pid) = id() else { return };
        spawn_local(async move {
            if let Ok(Some(f)) = obter_parceiro_admin(pid).await {
                nome.set(f.nome);
                site_url.set(f.site_url.unwrap_or_default());
                descricao.set(f.descricao.unwrap_or_default());
                ordem.set(f.ordem.to_string());
                ativo.set(f.ativo);
                logo_url.set(f.logo_url);
                cor.set(f.cor.unwrap_or_else(|| "#ff0070".to_string()));
                tagline.set(f.tagline.unwrap_or_default());
                imagens.set(f.itens);
            }
        });
    });

    let salvar = Action::new(|form: &ParceiroForm| {
        let f = form.clone();
        async move { salvar_parceiro(f).await }
    });

    let navegar = use_navigate();
    Effect::new(move |_| {
        if matches!(salvar.value().get(), Some(Ok(_))) {
            navegar("/admin/parceiros", Default::default());
        }
    });

    let erro = move || match salvar.value().get() {
        Some(Err(e)) => Some(e.to_string()),
        _ => None,
    };
    let salvando = salvar.pending();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let opt = |s: String| {
            let t = s.trim().to_string();
            (!t.is_empty()).then_some(t)
        };
        let form = ParceiroForm {
            id: id(),
            nome: nome.get_untracked().trim().to_string(),
            logo_url: logo_url.get_untracked(),
            site_url: opt(site_url.get_untracked()),
            descricao: opt(descricao.get_untracked()),
            cor: opt(cor.get_untracked()),
            tagline: opt(tagline.get_untracked()),
            itens: imagens.get_untracked(),
            ordem: ordem.get_untracked().trim().parse::<i32>().unwrap_or(0),
            ativo: ativo.get_untracked(),
        };
        salvar.dispatch(form);
    };

    view! {
        <header class="admin-head admin-head--row">
            <div>
                <h1 class="admin-head__title">
                    {move || if editando() { "Editar parceiro" } else { "Novo parceiro" }}
                </h1>
                <p class="admin-head__sub">"Logo, site e ordem de exibição"</p>
            </div>
            <a class="btn btn--ghost" href="/admin/parceiros">"Voltar"</a>
        </header>

        <form class="admin-card admin-form" on:submit=on_submit>
            <label class="field">
                <span class="field__label">"Nome"</span>
                <input
                    class="admin-input"
                    type="text"
                    prop:value=move || nome.get()
                    on:input=move |ev| nome.set(event_target_value(&ev))
                />
            </label>

            <label class="field">
                <span class="field__label">"Logo (JPG, PNG ou WEBP, até 5MB)"</span>
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
                                            Ok(url) => logo_url.set(Some(url)),
                                            Err(()) => {
                                                erro_img
                                                    .set(Some("Não foi possível enviar o logo.".to_string()))
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
                    logo_url.get().map(|u| view! { <img class="form-preview" src=u alt="Logo"/> })
                }}
                {move || erro_img.get().map(|m| view! { <span class="field__erro">{m}</span> })}
            </label>

            <div class="admin-form__grid">
                <label class="field">
                    <span class="field__label">"Site (URL)"</span>
                    <input
                        class="admin-input"
                        type="url"
                        prop:value=move || site_url.get()
                        on:input=move |ev| site_url.set(event_target_value(&ev))
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

            <label class="field">
                <span class="field__label">"Descrição"</span>
                <textarea
                    class="admin-input"
                    rows="3"
                    prop:value=move || descricao.get()
                    on:input=move |ev| descricao.set(event_target_value(&ev))
                ></textarea>
            </label>

            <p class="admin-fieldset__titulo">"Showcase da marca"</p>
            <div class="admin-form__grid">
                <label class="field">
                    <span class="field__label">"Cor da marca"</span>
                    <input
                        class="admin-input admin-input--cor"
                        type="color"
                        prop:value=move || cor.get()
                        on:input=move |ev| cor.set(event_target_value(&ev))
                    />
                </label>
                <label class="field">
                    <span class="field__label">"Segmento (tagline)"</span>
                    <input
                        class="admin-input"
                        type="text"
                        placeholder="Ex.: Tecnologia em delivery"
                        prop:value=move || tagline.get()
                        on:input=move |ev| tagline.set(event_target_value(&ev))
                    />
                </label>
            </div>
            <label class="field">
                <span class="field__label">
                    "Imagens dos produtos (swipe na página) — adicione uma por vez"
                </span>
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
                                    input.set_value("");
                                    enviando_prod.set(true);
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
                                        if let Ok(url) = r {
                                            imagens.update(|v| v.push(url));
                                        }
                                        enviando_prod.set(false);
                                    });
                                }
                            }
                        }
                        #[cfg(not(feature = "hydrate"))]
                        let _ = &ev;
                    }
                />
                {move || {
                    enviando_prod.get().then(|| view! { <span class="admin-status">"Enviando..."</span> })
                }}
                <div class="form-galeria">
                    {move || {
                        imagens
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(i, u)| {
                                view! {
                                    <div class="form-galeria__item">
                                        <img src=u alt=""/>
                                        <button
                                            type="button"
                                            class="form-galeria__rm"
                                            aria-label="Remover imagem"
                                            on:click=move |_| {
                                                imagens
                                                    .update(|v| {
                                                        if i < v.len() {
                                                            v.remove(i);
                                                        }
                                                    })
                                            }
                                        >
                                            "×"
                                        </button>
                                    </div>
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </label>

            <div class="admin-form__checks">
                <label class="login-check">
                    <input
                        type="checkbox"
                        prop:checked=move || ativo.get()
                        on:change=move |ev| ativo.set(event_target_checked(&ev))
                    />
                    <span>"Ativo"</span>
                </label>
            </div>

            {move || erro().map(|m| view! { <p class="orc-form__erro">{m}</p> })}

            <div class="admin-form__acoes">
                <a class="btn btn--ghost" href="/admin/parceiros">"Cancelar"</a>
                <button type="submit" class="btn btn--primary" prop:disabled=move || salvando.get()>
                    {move || if salvando.get() { "Salvando..." } else { "Salvar parceiro" }}
                </button>
            </div>
        </form>
    }
}
