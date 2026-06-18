use leptos::prelude::*;
use leptos_router::components::Redirect;

use crate::api::auth::login;

const ICON_EYE: &str = r#"<svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M1 12s4-7 11-7 11 7 11 7-4 7-11 7-11-7-11-7z"/><circle cx="12" cy="12" r="3"/></svg>"#;
const ICON_EYE_OFF: &str = r#"<svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M9.9 4.2A11 11 0 0 1 12 4c7 0 11 7 11 7a18 18 0 0 1-3.2 3.9M6.6 6.6A18 18 0 0 0 1 11s4 7 11 7a11 11 0 0 0 4-.8"/><path d="M9.5 9.5a3 3 0 0 0 4.2 4.2"/><path d="M1 1l22 22"/></svg>"#;

#[cfg(feature = "hydrate")]
const CHAVE_EMAIL: &str = "drinkup_email";

/// Acesso ao localStorage (somente no cliente).
#[cfg(feature = "hydrate")]
fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

/// Lê o e-mail lembrado (cliente).
#[cfg(feature = "hydrate")]
fn email_lembrado() -> Option<String> {
    storage()?.get_item(CHAVE_EMAIL).ok().flatten()
}

/// Salva ou apaga o e-mail lembrado conforme o checkbox (cliente).
#[cfg(feature = "hydrate")]
fn guardar_email(lembrar: bool, email: &str) {
    if let Some(s) = storage() {
        if lembrar && !email.is_empty() {
            let _ = s.set_item(CHAVE_EMAIL, email);
        } else {
            let _ = s.remove_item(CHAVE_EMAIL);
        }
    }
}

/// Tela de login do painel (sem cabeçalho/rodapé do site).
#[component]
pub fn AdminLoginPage() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let senha = RwSignal::new(String::new());
    let mostrar = RwSignal::new(false);
    let lembrar = RwSignal::new(false);

    // Após a hidratação, pré-preenche o e-mail lembrado (dado local do usuário).
    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        if let Some(e) = email_lembrado() {
            email.set(e);
            lembrar.set(true);
        }
    });

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
        let e = email.get_untracked();
        #[cfg(feature = "hydrate")]
        guardar_email(lembrar.get_untracked(), e.trim());
        entrar.dispatch((e, senha.get_untracked()));
    };

    view! {
        <Show
            when=ok
            fallback=move || {
                view! {
                    <section class="admin-login">
                        <div class="admin-login__brand">
                            <img
                                class="admin-login__logo"
                                src="/brand/logo-branca.png"
                                alt="DRINK UP"
                                width="180"
                                height="40"
                            />
                            <span class="admin-login__subtitle">"Painel administrativo"</span>
                        </div>

                        <form class="admin-login__card" on:submit=on_submit>
                            <label class="field">
                                <span class="field__label">"E-mail"</span>
                                <input
                                    type="email"
                                    autocomplete="username"
                                    placeholder="voce@drinkup.com.br"
                                    prop:value=move || email.get()
                                    on:input=move |ev| email.set(event_target_value(&ev))
                                />
                            </label>

                            <label class="field">
                                <span class="field__label">"Senha"</span>
                                <div class="field-senha">
                                    <input
                                        type=move || if mostrar.get() { "text" } else { "password" }
                                        autocomplete="current-password"
                                        prop:value=move || senha.get()
                                        on:input=move |ev| senha.set(event_target_value(&ev))
                                    />
                                    <button
                                        type="button"
                                        class="field-senha__toggle"
                                        aria-label="Mostrar ou ocultar a senha"
                                        on:click=move |_| mostrar.update(|m| *m = !*m)
                                        inner_html=move || {
                                            if mostrar.get() { ICON_EYE_OFF } else { ICON_EYE }
                                        }
                                    ></button>
                                </div>
                            </label>

                            <div class="login-row">
                                <label class="login-check">
                                    <input
                                        type="checkbox"
                                        prop:checked=move || lembrar.get()
                                        on:change=move |ev| lembrar.set(event_target_checked(&ev))
                                    />
                                    <span>"Lembrar e-mail"</span>
                                </label>
                                <a class="admin-login__forgot" href="/admin/recuperar-senha">
                                    "Esqueceu a senha?"
                                </a>
                            </div>

                            {move || erro().map(|m| view! { <p class="orc-form__erro">{m}</p> })}

                            <button
                                type="submit"
                                class="btn btn--primary btn--block"
                                prop:disabled=move || entrando.get()
                            >
                                {move || if entrando.get() { "Entrando..." } else { "Entrar" }}
                            </button>
                        </form>

                        <p class="admin-login__footer">"© 2026 DrinkUp · Acesso restrito"</p>
                    </section>
                }
            }
        >
            <Redirect path="/admin"/>
        </Show>
    }
}
