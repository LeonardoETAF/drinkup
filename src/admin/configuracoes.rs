use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::config::{obter_config, salvar_config};
use crate::domain::Configuracoes;

/// Configurações da loja (dados usados no site) + acesso a usuários.
#[component]
pub fn AdminConfiguracoes() -> impl IntoView {
    let nome_loja = RwSignal::new(String::new());
    let cnpj = RwSignal::new(String::new());
    let telefone = RwSignal::new(String::new());
    let endereco = RwSignal::new(String::new());
    let h_semana = RwSignal::new(String::new());
    let h_sabado = RwSignal::new(String::new());
    let h_domingo = RwSignal::new(String::new());
    let facebook = RwSignal::new(String::new());
    let instagram = RwSignal::new(String::new());
    let carregado = RwSignal::new(false);

    Effect::new(move |_| {
        spawn_local(async move {
            if let Ok(c) = obter_config().await {
                nome_loja.set(c.nome_loja);
                cnpj.set(c.cnpj);
                telefone.set(c.telefone);
                endereco.set(c.endereco);
                h_semana.set(c.horario_semana);
                h_sabado.set(c.horario_sabado);
                h_domingo.set(c.horario_domingo);
                facebook.set(c.facebook);
                instagram.set(c.instagram);
            }
            carregado.set(true);
        });
    });

    let salvar = Action::new(|cfg: &Configuracoes| {
        let c = cfg.clone();
        async move { salvar_config(c).await }
    });
    let salvando = salvar.pending();
    let salvo = move || matches!(salvar.value().get(), Some(Ok(())));
    let erro = move || match salvar.value().get() {
        Some(Err(e)) => Some(e.to_string()),
        _ => None,
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let cfg = Configuracoes {
            nome_loja: nome_loja.get_untracked(),
            cnpj: cnpj.get_untracked(),
            telefone: telefone.get_untracked(),
            endereco: endereco.get_untracked(),
            horario_semana: h_semana.get_untracked(),
            horario_sabado: h_sabado.get_untracked(),
            horario_domingo: h_domingo.get_untracked(),
            facebook: facebook.get_untracked(),
            instagram: instagram.get_untracked(),
        };
        salvar.dispatch(cfg);
    };

    view! {
        <header class="admin-head">
            <h1 class="admin-head__title">"Configurações"</h1>
            <p class="admin-head__sub">"Dados e preferências da loja"</p>
        </header>

        <form class="admin-config" on:submit=on_submit>
            <fieldset class="admin-card admin-fieldset">
                <legend class="admin-fieldset__titulo">"Informações da loja"</legend>
                <div class="admin-form__grid">
                    {campo("Nome da loja", nome_loja)} {campo("CNPJ", cnpj)}
                </div>
            </fieldset>

            <fieldset class="admin-card admin-fieldset">
                <legend class="admin-fieldset__titulo">"Contato"</legend>
                {campo("Telefone / WhatsApp", telefone)} {campo("Endereço", endereco)}
            </fieldset>

            <fieldset class="admin-card admin-fieldset">
                <legend class="admin-fieldset__titulo">"Horários de atendimento"</legend>
                <div class="admin-form__grid admin-form__grid--3">
                    {campo("Segunda a sexta", h_semana)} {campo("Sábado", h_sabado)}
                    {campo("Domingo", h_domingo)}
                </div>
            </fieldset>

            <fieldset class="admin-card admin-fieldset">
                <legend class="admin-fieldset__titulo">"Redes sociais"</legend>
                <p class="admin-head__sub">
                    "Links abertos pelos botões do rodapé. O WhatsApp usa o telefone acima."
                </p>
                <div class="admin-form__grid">
                    {campo("Facebook (URL)", facebook)} {campo("Instagram (URL)", instagram)}
                </div>
            </fieldset>

            {move || erro().map(|m| view! { <p class="orc-form__erro">{m}</p> })}
            {move || {
                (salvo() && erro().is_none())
                    .then(|| view! { <p class="admin-ok">"Alterações salvas."</p> })
            }}

            <div class="admin-form__acoes">
                <a class="btn btn--ghost" href="/admin">"Cancelar"</a>
                <button type="submit" class="btn btn--primary" prop:disabled=move || salvando.get()>
                    {move || if salvando.get() { "Salvando..." } else { "Salvar alterações" }}
                </button>
            </div>
        </form>

        <section class="admin-card admin-acesso">
            <div>
                <h2 class="admin-fieldset__titulo">"Conteúdo da Home"</h2>
                <p class="admin-head__sub">"Edite a faixa de números e a seção \"Sua marca\"."</p>
            </div>
            <a class="btn btn--ghost" href="/admin/conteudo">"Editar conteúdo"</a>
        </section>

        <section class="admin-card admin-acesso">
            <div>
                <h2 class="admin-fieldset__titulo">"Conteúdo de Quem Somos"</h2>
                <p class="admin-head__sub">
                    "Destaque, vídeo, missão/visão/valores, fotos e depoimentos."
                </p>
            </div>
            <a class="btn btn--ghost" href="/admin/conteudo-quem-somos">"Editar conteúdo"</a>
        </section>

        <section class="admin-card admin-acesso">
            <div>
                <h2 class="admin-fieldset__titulo">"Usuários do painel"</h2>
                <p class="admin-head__sub">"Gerencie quem acessa o painel e seus papéis."</p>
            </div>
            <a class="btn btn--ghost" href="/admin/usuarios">"Gerenciar usuários"</a>
        </section>
    }
}

/// Campo de texto controlado, reutilizado nas seções do formulário.
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
