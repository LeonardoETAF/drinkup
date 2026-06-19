use leptos::prelude::*;

const IC_UPLOAD: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="M17 8l-5-5-5 5"/><path d="M12 3v12"/></svg>"#;

/// Card de upload de imagem reutilizado no painel: mostra a dica de formato/tamanho,
/// abre o seletor ao clicar e exibe a prévia da imagem enviada dentro do próprio card.
#[component]
pub fn CartaoUpload(
    /// URL da imagem atual (`None` = vazio).
    url: RwSignal<Option<String>>,
    /// Dica de formato e tamanho ideal (ex.: "PNG/JPG/WEBP · 1200×800px · até 5MB").
    #[prop(into)]
    dica: String,
) -> impl IntoView {
    let enviando = RwSignal::new(false);
    let erro = RwSignal::new(false);

    view! {
        <label class="upload-card" class:is-cheio=move || url.get().is_some()>
            <input
                class="upload-card__input"
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
                                enviando.set(true);
                                erro.set(false);
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
                                        Ok(u) => url.set(Some(u)),
                                        Err(()) => erro.set(true),
                                    }
                                    enviando.set(false);
                                });
                            }
                        }
                    }
                    #[cfg(not(feature = "hydrate"))]
                    let _ = &ev;
                }
            />
            {move || {
                url.get()
                    .map(|u| view! { <img class="upload-card__img" src=u alt="Imagem enviada"/> })
            }}
            <span class="upload-card__conteudo">
                <span class="upload-card__icone" inner_html=IC_UPLOAD></span>
                <span class="upload-card__acao">
                    {move || {
                        if url.get().is_some() { "Trocar imagem" } else { "Clique para enviar" }
                    }}
                </span>
                <span class="upload-card__dica">{dica}</span>
            </span>
            {move || {
                enviando.get().then(|| view! { <span class="upload-card__status">"Enviando..."</span> })
            }}
            {move || {
                erro.get()
                    .then(|| view! { <span class="upload-card__erro">"Falha no envio."</span> })
            }}
        </label>
    }
}
