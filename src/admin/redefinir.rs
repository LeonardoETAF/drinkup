use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::api::recuperacao::redefinir_senha;

/// Tela de definição de nova senha, acessada pelo link enviado por e-mail
/// (`/admin/redefinir-senha?token=...`).
#[component]
pub fn AdminRedefinirSenhaPage() -> impl IntoView {
    let query = use_query_map();
    let token = move || query.read().get("token").unwrap_or_default();
    let tem_token = move || !token().is_empty();

    let senha = RwSignal::new(String::new());
    let confirmar = RwSignal::new(String::new());
    let erro_local = RwSignal::new(None::<&'static str>);

    let redefinir = Action::new(|args: &(String, String)| {
        let (token, senha) = args.clone();
        async move { redefinir_senha(token, senha).await }
    });
    let salvando = redefinir.pending();
    let sucesso = move || matches!(redefinir.value().get(), Some(Ok(())));
    let erro_servidor = move || match redefinir.value().get() {
        Some(Err(e)) => Some(e.to_string()),
        _ => None,
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let s = senha.get_untracked();
        let c = confirmar.get_untracked();
        if s.chars().count() < 8 {
            erro_local.set(Some("A senha deve ter ao menos 8 caracteres."));
            return;
        }
        if s != c {
            erro_local.set(Some("As senhas não conferem."));
            return;
        }
        erro_local.set(None);
        redefinir.dispatch((token(), s));
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
                <Show
                    when=sucesso
                    fallback=move || {
                        view! {
                            <Show
                                when=tem_token
                                fallback=|| {
                                    view! {
                                        <div class="recuperar-head">
                                            <h1 class="recuperar__title">"Link inválido"</h1>
                                            <p class="recuperar__sub">
                                                "Este link de redefinição é inválido. Solicite um novo."
                                            </p>
                                            <a
                                                class="btn btn--primary btn--block"
                                                href="/admin/recuperar-senha"
                                            >
                                                "Recuperar senha"
                                            </a>
                                        </div>
                                    }
                                }
                            >
                                <form class="recuperar-form" on:submit=on_submit>
                                    <div class="recuperar-head">
                                        <h1 class="recuperar__title">"Nova senha"</h1>
                                        <p class="recuperar__sub">
                                            "Escolha uma nova senha para o seu acesso."
                                        </p>
                                    </div>
                                    <label class="field">
                                        <span class="field__label">"Nova senha"</span>
                                        <input
                                            type="password"
                                            autocomplete="new-password"
                                            prop:value=move || senha.get()
                                            on:input=move |ev| senha.set(event_target_value(&ev))
                                        />
                                    </label>
                                    <label class="field">
                                        <span class="field__label">"Confirmar senha"</span>
                                        <input
                                            type="password"
                                            autocomplete="new-password"
                                            prop:value=move || confirmar.get()
                                            on:input=move |ev| confirmar.set(event_target_value(&ev))
                                        />
                                    </label>
                                    {move || {
                                        erro_local
                                            .get()
                                            .map(|m| view! { <span class="field__erro">{m}</span> })
                                    }}
                                    {move || {
                                        erro_servidor()
                                            .map(|m| view! { <span class="field__erro">{m}</span> })
                                    }}
                                    <button
                                        type="submit"
                                        class="btn btn--primary btn--block"
                                        prop:disabled=move || salvando.get()
                                    >
                                        {move || {
                                            if salvando.get() { "Salvando..." } else { "Redefinir senha" }
                                        }}
                                    </button>
                                </form>
                            </Show>
                        }
                    }
                >
                    <div class="recuperar-head">
                        <h1 class="recuperar__title">"Senha redefinida!"</h1>
                        <p class="recuperar__sub">
                            "Sua senha foi atualizada. Você já pode entrar com a nova senha."
                        </p>
                        <a class="btn btn--primary btn--block" href="/admin/login">"Ir para o login"</a>
                    </div>
                </Show>
            </div>

            <p class="admin-login__footer">"© 2026 DrinkUp · Acesso restrito"</p>
        </section>
    }
}
