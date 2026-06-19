use leptos::prelude::*;
use leptos::task::spawn_local;

use super::editor_pares::{carregar, serializar, EditorPares, Par};
use crate::admin::upload_card::CartaoUpload;
use crate::api::home_admin::{obter_home_form, salvar_home};
use crate::domain::HomeForm;

/// Edição do conteúdo da home: faixa de números + bento "Sua marca".
#[component]
pub fn AdminConteudoHome() -> impl IntoView {
    let numeros = RwSignal::new(Vec::<Par>::new());
    let numeros_id = RwSignal::new(0usize);
    let marca_titulo = RwSignal::new(String::new());
    let marca_sub = RwSignal::new(String::new());
    let bento = RwSignal::new(Vec::<Par>::new());
    let bento_id = RwSignal::new(0usize);
    let foto1 = RwSignal::new(None::<String>);
    let foto2 = RwSignal::new(None::<String>);

    Effect::new(move |_| {
        spawn_local(async move {
            if let Ok(f) = obter_home_form().await {
                carregar(numeros, numeros_id, &f.numeros);
                marca_titulo.set(f.marca_titulo);
                marca_sub.set(f.marca_sub);
                carregar(bento, bento_id, &f.bento);
                foto1.set(f.foto1_url);
                foto2.set(f.foto2_url);
            }
        });
    });

    let salvar = Action::new(|f: &HomeForm| {
        let f = f.clone();
        async move { salvar_home(f).await }
    });
    let salvando = salvar.pending();
    let salvo = move || matches!(salvar.value().get(), Some(Ok(())));
    let erro = move || match salvar.value().get() {
        Some(Err(e)) => Some(e.to_string()),
        _ => None,
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        salvar.dispatch(HomeForm {
            numeros: serializar(numeros),
            marca_titulo: marca_titulo.get_untracked(),
            marca_sub: marca_sub.get_untracked(),
            bento: serializar(bento),
            foto1_url: foto1.get_untracked(),
            foto2_url: foto2.get_untracked(),
        });
    };

    view! {
        <header class="admin-head">
            <h1 class="admin-head__title">"Conteúdo da Home"</h1>
            <p class="admin-head__sub">"Faixa de números e seção \"Sua marca\""</p>
        </header>

        <form class="admin-config" on:submit=on_submit>
            <fieldset class="admin-card admin-fieldset">
                <legend class="admin-fieldset__titulo">"Faixa de números"</legend>
                <p class="admin-head__sub">"Cada número com seu valor e rótulo."</p>
                <EditorPares
                    itens=numeros
                    proximo_id=numeros_id
                    ph_valor="Valor (ex.: +500)"
                    ph_rotulo="Rótulo (ex.: Clientes satisfeitos)"
                    add_texto="+ Adicionar número"
                />
            </fieldset>

            <fieldset class="admin-card admin-fieldset">
                <legend class="admin-fieldset__titulo">"Seção \"Sua marca\""</legend>
                <div class="admin-form__grid">
                    {campo("Título", marca_titulo)} {campo("Subtítulo", marca_sub)}
                </div>
                <div class="field">
                    <span class="field__label">"Números do bento"</span>
                    <EditorPares
                        itens=bento
                        proximo_id=bento_id
                        ph_valor="Valor (ex.: +25K)"
                        ph_rotulo="Rótulo (ex.: Unidades por dia)"
                        fixo=true
                    />
                </div>
                <div class="admin-form__grid">
                    <div class="field">
                        <span class="field__label">"Foto 1 (ex.: growler)"</span>
                        <CartaoUpload url=foto1 dica="PNG/JPG/WEBP · paisagem (16:9) · até 5MB"/>
                    </div>
                    <div class="field">
                        <span class="field__label">"Foto 2 (ex.: caneca)"</span>
                        <CartaoUpload url=foto2 dica="PNG/JPG/WEBP · quadrada (1:1) · até 5MB"/>
                    </div>
                </div>
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
