use leptos::prelude::*;
use leptos_router::components::Redirect;

use crate::api::auth::login;

/// Tela de login do painel (sem cabeçalho/rodapé do site).
#[component]
pub fn AdminLoginPage() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let senha = RwSignal::new(String::new());

    let entrar = Action::new(|(e, s): &(String, String)| {
        let (e, s) = (e.clone(), s.clone());
        async move { login(e, s).await }
    });

    let erro = move || match entrar.value().get() {
        Some(Err(e)) => Some(e.to_string()),
        _ => None,
    };
    let ok = move || matches!(entrar.value().get(), Some(Ok(())));
    let entrando = entrar.pending();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        entrar.dispatch((email.get_untracked(), senha.get_untracked()));
    };

    view! {
        <Show
            when=ok
            fallback=move || {
                view! {
                    <section class="admin-login">
                        <form class="admin-login__card" on:submit=on_submit>
                            <img
                                class="admin-login__logo"
                                src="/brand/logo-branca.png"
                                alt="DRINK UP"
                                width="160"
                                height="36"
                            />
                            <h1 class="admin-login__title">"Painel administrativo"</h1>
                            <label class="field">
                                <span class="field__label">"E-mail"</span>
                                <input
                                    type="email"
                                    autocomplete="username"
                                    prop:value=move || email.get()
                                    on:input=move |ev| email.set(event_target_value(&ev))
                                />
                            </label>
                            <label class="field">
                                <span class="field__label">"Senha"</span>
                                <input
                                    type="password"
                                    autocomplete="current-password"
                                    prop:value=move || senha.get()
                                    on:input=move |ev| senha.set(event_target_value(&ev))
                                />
                            </label>
                            {move || {
                                erro().map(|m| view! { <p class="orc-form__erro">{m}</p> })
                            }}
                            <button
                                type="submit"
                                class="btn btn--primary btn--block"
                                prop:disabled=move || entrando.get()
                            >
                                {move || if entrando.get() { "Entrando..." } else { "Entrar" }}
                            </button>
                        </form>
                    </section>
                }
            }
        >
            <Redirect path="/admin"/>
        </Show>
    }
}
