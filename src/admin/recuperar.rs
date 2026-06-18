use leptos::prelude::*;

const ICON_VOLTAR: &str = r#"<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M19 12H5"/><path d="M12 19l-7-7 7-7"/></svg>"#;

/// Tela "Recuperar senha": pede o WhatsApp para envio de um código.
/// (UI; o envio real do código depende da integração de WhatsApp — pendente.)
#[component]
pub fn AdminRecuperarSenhaPage() -> impl IntoView {
    let whatsapp = RwSignal::new(String::new());
    let erro = RwSignal::new(false);
    let enviado = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let digitos = whatsapp
            .get_untracked()
            .chars()
            .filter(char::is_ascii_digit)
            .count();
        if digitos < 10 {
            erro.set(true);
            return;
        }
        erro.set(false);
        enviado.set(true);
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
                    when=move || enviado.get()
                    fallback=move || {
                        view! {
                            <form class="recuperar-form" on:submit=on_submit>
                                <div class="recuperar-head">
                                    <h1 class="recuperar__title">"Recuperar senha"</h1>
                                    <p class="recuperar__sub">
                                        "Informe o WhatsApp cadastrado e enviaremos um código para você redefinir sua senha."
                                    </p>
                                </div>
                                <label class="field">
                                    <span class="field__label">"WhatsApp"</span>
                                    <input
                                        type="tel"
                                        placeholder="(00) 00000-0000"
                                        class:field--erro=move || erro.get()
                                        prop:value=move || whatsapp.get()
                                        on:input=move |ev| whatsapp.set(event_target_value(&ev))
                                    />
                                    <Show when=move || erro.get()>
                                        <span class="field__erro">"Informe um WhatsApp válido."</span>
                                    </Show>
                                </label>
                                <button type="submit" class="btn btn--primary btn--block">
                                    "Enviar código"
                                </button>
                            </form>
                        }
                    }
                >
                    <div class="recuperar-head">
                        <h1 class="recuperar__title">"Tudo certo!"</h1>
                        <p class="recuperar__sub">
                            "Se este WhatsApp estiver cadastrado, você receberá um código para redefinir sua senha."
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
