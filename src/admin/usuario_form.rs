use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_params_map};
use uuid::Uuid;

use crate::api::usuarios_admin::{obter_usuario_admin, salvar_usuario};
use crate::domain::UsuarioForm;

/// Menus (chave, rótulo) que podem ser liberados por usuário.
const MENU_OPCOES: [(&str, &str); 9] = [
    ("dashboard", "Dashboard"),
    ("produtos", "Produtos"),
    ("leads", "Leads"),
    ("novidades", "Novidades"),
    ("parceiros", "Parceiros"),
    ("eventos", "Eventos"),
    ("conteudo", "Conteúdo Home"),
    ("quem-somos", "Quem Somos"),
    ("configuracoes", "Configurações"),
];

/// Formulário de criação/edição de usuário do painel.
#[component]
pub fn AdminUsuarioForm() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params
            .read()
            .get("id")
            .and_then(|s| Uuid::parse_str(&s).ok())
    };
    let editando = move || id().is_some();

    let nome = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let papel = RwSignal::new("editor".to_string());
    let ativo = RwSignal::new(true);
    let senha = RwSignal::new(String::new());
    // Novo usuário começa com acesso a todos os menus (admin restringe se quiser).
    let menus =
        RwSignal::new(MENU_OPCOES.iter().map(|(k, _)| k.to_string()).collect::<Vec<String>>());

    Effect::new(move |_| {
        let Some(uid) = id() else { return };
        spawn_local(async move {
            if let Ok(Some(f)) = obter_usuario_admin(uid).await {
                nome.set(f.nome);
                email.set(f.email);
                papel.set(f.papel);
                ativo.set(f.ativo);
                menus.set(f.menus);
            }
        });
    });

    let salvar = Action::new(|form: &UsuarioForm| {
        let f = form.clone();
        async move { salvar_usuario(f).await }
    });

    let navegar = use_navigate();
    Effect::new(move |_| {
        if matches!(salvar.value().get(), Some(Ok(_))) {
            navegar("/admin/usuarios", Default::default());
        }
    });

    let erro = move || match salvar.value().get() {
        Some(Err(e)) => Some(crate::components::mensagem_erro(&e)),
        _ => None,
    };
    let salvando = salvar.pending();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let s = senha.get_untracked().trim().to_string();
        let form = UsuarioForm {
            id: id(),
            nome: nome.get_untracked().trim().to_string(),
            email: email.get_untracked().trim().to_string(),
            papel: papel.get_untracked(),
            ativo: ativo.get_untracked(),
            senha: (!s.is_empty()).then_some(s),
            menus: menus.get_untracked(),
        };
        salvar.dispatch(form);
    };

    view! {
        <header class="admin-head admin-head--row">
            <div>
                <h1 class="admin-head__title">
                    {move || if editando() { "Editar usuário" } else { "Novo usuário" }}
                </h1>
                <p class="admin-head__sub">"Dados de acesso ao painel"</p>
            </div>
            <a class="btn btn--ghost" href="/admin/usuarios">"Voltar"</a>
        </header>

        <form class="admin-card admin-form" on:submit=on_submit>
            <div class="admin-form__grid">
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
                    <span class="field__label">"E-mail"</span>
                    <input
                        class="admin-input"
                        type="email"
                        autocomplete="off"
                        prop:value=move || email.get()
                        on:input=move |ev| email.set(event_target_value(&ev))
                    />
                </label>
                <label class="field">
                    <span class="field__label">"Papel"</span>
                    <select
                        class="admin-input"
                        prop:value=move || papel.get()
                        on:change=move |ev| papel.set(event_target_value(&ev))
                    >
                        <option value="admin">"Administrador"</option>
                        <option value="gerente">"Gerente"</option>
                        <option value="editor">"Editor"</option>
                        <option value="visualizador">"Visualizador (só leitura)"</option>
                    </select>
                </label>
                <label class="field">
                    <span class="field__label">
                        {move || {
                            if editando() {
                                "Senha (deixe em branco para manter)"
                            } else {
                                "Senha"
                            }
                        }}
                    </span>
                    <input
                        class="admin-input"
                        type="password"
                        autocomplete="new-password"
                        prop:value=move || senha.get()
                        on:input=move |ev| senha.set(event_target_value(&ev))
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
                    <span>"Ativo"</span>
                </label>
            </div>

            <p class="admin-fieldset__titulo">"Acesso aos menus"</p>
            <div class="admin-form__checks admin-form__checks--menus">
                {MENU_OPCOES
                    .iter()
                    .map(|&(k, label)| menu_check(k, label, menus))
                    .collect_view()}
            </div>

            {move || erro().map(|m| view! { <p class="orc-form__erro">{m}</p> })}

            <div class="admin-form__acoes">
                <a class="btn btn--ghost" href="/admin/usuarios">"Cancelar"</a>
                <button type="submit" class="btn btn--primary" prop:disabled=move || salvando.get()>
                    {move || if salvando.get() { "Salvando..." } else { "Salvar usuário" }}
                </button>
            </div>
        </form>
    }
}

/// Checkbox de permissão de um menu (entra/sai da lista de menus do usuário).
fn menu_check(
    key: &'static str,
    label: &'static str,
    menus: RwSignal<Vec<String>>,
) -> impl IntoView {
    view! {
        <label class="login-check">
            <input
                type="checkbox"
                prop:checked=move || menus.get().iter().any(|m| m == key)
                on:change=move |ev| {
                    let on = event_target_checked(&ev);
                    menus
                        .update(|v| {
                            if on {
                                if !v.iter().any(|m| m == key) {
                                    v.push(key.to_string());
                                }
                            } else {
                                v.retain(|m| m != key);
                            }
                        });
                }
            />
            <span>{label}</span>
        </label>
    }
}
