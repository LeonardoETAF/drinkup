use leptos::prelude::*;

use crate::api::recuperacao::solicitar_redefinicao;

const ICON_VOLTAR: &str = r#"<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M19 12H5"/><path d="M12 19l-7-7 7-7"/></svg>"#;

/// Tela "Recuperar senha": pede o e-mail cadastrado e dispara o envio do link.
/// A resposta é sempre genérica (não revela se o e-mail existe).
#[component]
pub fn AdminRecuperarSenhaPage() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let erro = RwSignal::new(false);

    let solicitar = Action::new(|email: &String| {
        let email = email.clone();
        async move { solicitar_redefinicao(email).await }
    });
    // Sucesso assim que a ação concluir (independente do resultado: genérico).
    let enviado = move || solicitar.value().get().is_some();
    let enviando = solicitar.pending();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let e = email.get_untracked().trim().to_string();
        if !e.contains('@') || e.len() < 5 {
            erro.set(true);
            return;
        }
        erro.set(false);
        solicitar.dispatch(e);
    };

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

            <div class="admin-login__card">
                <a class="admin-back" href="/admin/login">
                    <span class="admin-back__icon" inner_html=ICON_VOLTAR></span>
                    "Voltar ao login"
                </a>

                <Show
                    when=enviado
                    fallback=move || {
                        view! {
                            <form class="recuperar-form" on:submit=on_submit>
                                <div class="recuperar-head">
                                    <h1 class="recuperar__title">"Recuperar senha"</h1>
                                    <p class="recuperar__sub">
                                        "Informe o e-mail cadastrado e enviaremos um link para você redefinir sua senha."
                                    </p>
                                </div>
                                <label class="field">
                                    <span class="field__label">"E-mail"</span>
                                    <input
                                        type="email"
                                        autocomplete="email"
                                        placeholder="voce@exemplo.com"
                                        class:field--erro=move || erro.get()
                                        prop:value=move || email.get()
                                        on:input=move |ev| email.set(event_target_value(&ev))
                                    />
                                    <Show when=move || erro.get()>
                                        <span class="field__erro">"Informe um e-mail válido."</span>
                                    </Show>
                                </label>
                                <button
                                    type="submit"
                                    class="btn btn--primary btn--block"
                                    prop:disabled=move || enviando.get()
                                >
                                    {move || if enviando.get() { "Enviando..." } else { "Enviar link" }}
                                </button>
                            </form>
                        }
                    }
                >
                    <div class="recuperar-head">
                        <h1 class="recuperar__title">"Verifique seu e-mail"</h1>
                        <p class="recuperar__sub">
                            "Se este e-mail estiver cadastrado, você receberá um link para redefinir sua senha. O link expira em 1 hora."
                        </p>
                        <a class="btn btn--primary btn--block" href="/admin/login">
                            "Voltar ao login"
                        </a>
                    </div>
                </Show>
            </div>

            <p class="admin-login__footer">"© 2026 DrinkUp · Acesso restrito"</p>
        </section>
    }
}
