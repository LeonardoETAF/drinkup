use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_params_map};
use uuid::Uuid;

use crate::admin::upload_card::CartaoUpload;
use crate::api::catalogo::listar_categorias;
use crate::api::produtos_admin::{obter_produto_admin, salvar_produto};
use crate::domain::{Categoria, ProdutoForm};

/// Formulário de criação/edição de produto.
#[component]
pub fn AdminProdutoForm() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params
            .read()
            .get("id")
            .and_then(|s| Uuid::parse_str(&s).ok())
    };
    let editando = move || id().is_some();

    let categorias = RwSignal::new(Vec::<Categoria>::new());
    Effect::new(move |_| {
        spawn_local(async move {
            if let Ok(c) = listar_categorias().await {
                categorias.set(c);
            }
        });
    });

    let nome = RwSignal::new(String::new());
    let categoria_id = RwSignal::new(String::new());
    let descricao = RwSignal::new(String::new());
    let capacidade = RwSignal::new(String::new());
    let material = RwSignal::new(String::new());
    let cor = RwSignal::new(String::new());
    let altura = RwSignal::new(String::new());
    let diametro = RwSignal::new(String::new());
    let personalizavel = RwSignal::new(true);
    let destaque = RwSignal::new(false);
    let ativo = RwSignal::new(true);
    let imagem_url = RwSignal::new(None::<String>);

    // Carrega os dados ao editar.
    Effect::new(move |_| {
        let Some(pid) = id() else { return };
        spawn_local(async move {
            if let Ok(Some(f)) = obter_produto_admin(pid).await {
                nome.set(f.nome);
                categoria_id.set(f.categoria_id.map(|c| c.to_string()).unwrap_or_default());
                descricao.set(f.descricao.unwrap_or_default());
                capacidade.set(f.capacidade_ml.map(|v| v.to_string()).unwrap_or_default());
                material.set(f.material.unwrap_or_default());
                cor.set(f.cor.unwrap_or_default());
                altura.set(f.altura_mm.map(|v| v.to_string()).unwrap_or_default());
                diametro.set(f.diametro_mm.map(|v| v.to_string()).unwrap_or_default());
                personalizavel.set(f.personalizavel);
                destaque.set(f.destaque);
                ativo.set(f.ativo);
                imagem_url.set(f.imagem_url);
            }
        });
    });

    let salvar = Action::new(|form: &ProdutoForm| {
        let f = form.clone();
        async move { salvar_produto(f).await }
    });

    let navegar = use_navigate();
    Effect::new(move |_| {
        if matches!(salvar.value().get(), Some(Ok(_))) {
            navegar("/admin/produtos", Default::default());
        }
    });

    let erro = move || match salvar.value().get() {
        Some(Err(e)) => Some(e.to_string()),
        _ => None,
    };
    let salvando = salvar.pending();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let parse_i = |s: String| s.trim().parse::<i32>().ok();
        let opt = |s: String| {
            let t = s.trim().to_string();
            (!t.is_empty()).then_some(t)
        };
        let cat = categoria_id.get_untracked();
        let form = ProdutoForm {
            id: id(),
            nome: nome.get_untracked().trim().to_string(),
            categoria_id: (!cat.is_empty())
                .then(|| Uuid::parse_str(&cat).ok())
                .flatten(),
            descricao: opt(descricao.get_untracked()),
            capacidade_ml: parse_i(capacidade.get_untracked()),
            material: opt(material.get_untracked()),
            cor: opt(cor.get_untracked()),
            altura_mm: parse_i(altura.get_untracked()),
            diametro_mm: parse_i(diametro.get_untracked()),
            personalizavel: personalizavel.get_untracked(),
            destaque: destaque.get_untracked(),
            ativo: ativo.get_untracked(),
            imagem_url: imagem_url.get_untracked(),
        };
        salvar.dispatch(form);
    };

    view! {
        <header class="admin-head admin-head--row">
            <div>
                <h1 class="admin-head__title">
                    {move || if editando() { "Editar produto" } else { "Novo produto" }}
                </h1>
                <p class="admin-head__sub">"Preencha os dados e envie uma imagem"</p>
            </div>
            <a class="btn btn--ghost" href="/admin/produtos">"Voltar"</a>
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
                <span class="field__label">"Categoria"</span>
                <select
                    class="admin-input"
                    prop:value=move || categoria_id.get()
                    on:change=move |ev| categoria_id.set(event_target_value(&ev))
                >
                    <option value="">"— Sem categoria —"</option>
                    {move || {
                        categorias
                            .get()
                            .into_iter()
                            .map(|c| {
                                view! { <option value=c.id.to_string()>{c.nome}</option> }
                            })
                            .collect_view()
                    }}
                </select>
            </label>

            <label class="field">
                <span class="field__label">"Descrição"</span>
                <textarea
                    class="admin-input"
                    rows="3"
                    prop:value=move || descricao.get()
                    on:input=move |ev| descricao.set(event_target_value(&ev))
                ></textarea>
            </label>

            <div class="admin-form__grid">
                <label class="field">
                    <span class="field__label">"Capacidade (ml)"</span>
                    <input
                        class="admin-input"
                        type="number"
                        prop:value=move || capacidade.get()
                        on:input=move |ev| capacidade.set(event_target_value(&ev))
                    />
                </label>
                <label class="field">
                    <span class="field__label">"Material"</span>
                    <input
                        class="admin-input"
                        type="text"
                        prop:value=move || material.get()
                        on:input=move |ev| material.set(event_target_value(&ev))
                    />
                </label>
                <label class="field">
                    <span class="field__label">"Cor"</span>
                    <input
                        class="admin-input"
                        type="text"
                        prop:value=move || cor.get()
                        on:input=move |ev| cor.set(event_target_value(&ev))
                    />
                </label>
                <label class="field">
                    <span class="field__label">"Altura (mm)"</span>
                    <input
                        class="admin-input"
                        type="number"
                        prop:value=move || altura.get()
                        on:input=move |ev| altura.set(event_target_value(&ev))
                    />
                </label>
                <label class="field">
                    <span class="field__label">"Diâmetro (mm)"</span>
                    <input
                        class="admin-input"
                        type="number"
                        prop:value=move || diametro.get()
                        on:input=move |ev| diametro.set(event_target_value(&ev))
                    />
                </label>
            </div>

            <div class="field">
                <span class="field__label">"Imagem do produto"</span>
                <CartaoUpload url=imagem_url dica="PNG/JPG/WEBP · quadrada (1:1) · até 5MB"/>
            </div>

            <div class="admin-form__checks">
                <label class="login-check">
                    <input
                        type="checkbox"
                        prop:checked=move || personalizavel.get()
                        on:change=move |ev| personalizavel.set(event_target_checked(&ev))
                    />
                    <span>"Personalizável"</span>
                </label>
                <label class="login-check">
                    <input
                        type="checkbox"
                        prop:checked=move || destaque.get()
                        on:change=move |ev| destaque.set(event_target_checked(&ev))
                    />
                    <span>"Destaque"</span>
                </label>
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
                <a class="btn btn--ghost" href="/admin/produtos">"Cancelar"</a>
                <button type="submit" class="btn btn--primary" prop:disabled=move || salvando.get()>
                    {move || if salvando.get() { "Salvando..." } else { "Salvar produto" }}
                </button>
            </div>
        </form>
    }
}
