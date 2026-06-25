use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_params_map};
use uuid::Uuid;

use super::modal::ModalConfirmacao;
use crate::api::catalogo::listar_categorias;
use crate::api::produtos_admin::{obter_produto_admin, salvar_produto};
use crate::domain::{cm_para_mm, mm_para_cm, Categoria, ProdutoForm};

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
    let subcategoria_id = RwSignal::new(String::new());
    let descricao = RwSignal::new(String::new());
    let capacidade = RwSignal::new(String::new());
    let material = RwSignal::new(String::new());
    let cor = RwSignal::new(String::new());
    let altura = RwSignal::new(String::new());
    let diametro = RwSignal::new(String::new());
    let peso = RwSignal::new(String::new());
    let largura_base = RwSignal::new(String::new());
    let largura_boca = RwSignal::new(String::new());
    let personalizavel = RwSignal::new(true);
    let destaque = RwSignal::new(false);
    let ativo = RwSignal::new(true);
    let imagens = RwSignal::new(Vec::<String>::new());
    // Índice da imagem da galeria aguardando confirmação de remoção, ou None.
    let img_pendente = RwSignal::new(None::<usize>);
    let enviando_prod = RwSignal::new(false);

    // Carrega os dados ao editar.
    Effect::new(move |_| {
        let Some(pid) = id() else { return };
        spawn_local(async move {
            if let Ok(Some(f)) = obter_produto_admin(pid).await {
                nome.set(f.nome);
                categoria_id.set(f.categoria_id.map(|c| c.to_string()).unwrap_or_default());
                subcategoria_id.set(f.subcategoria_id.map(|c| c.to_string()).unwrap_or_default());
                descricao.set(f.descricao.unwrap_or_default());
                capacidade.set(f.capacidade_ml.map(|v| v.to_string()).unwrap_or_default());
                material.set(f.material.unwrap_or_default());
                cor.set(f.cor.unwrap_or_default());
                altura.set(f.altura_mm.map(mm_para_cm).unwrap_or_default());
                diametro.set(f.diametro_mm.map(mm_para_cm).unwrap_or_default());
                peso.set(f.peso_g.map(|v| v.to_string()).unwrap_or_default());
                largura_base.set(f.largura_base_mm.map(mm_para_cm).unwrap_or_default());
                largura_boca.set(f.largura_boca_mm.map(mm_para_cm).unwrap_or_default());
                personalizavel.set(f.personalizavel);
                destaque.set(f.destaque);
                ativo.set(f.ativo);
                imagens.set(f.imagens);
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
        Some(Err(e)) => Some(crate::components::mensagem_erro(&e)),
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
            subcategoria_id: {
                let s = subcategoria_id.get_untracked();
                (!s.is_empty()).then(|| Uuid::parse_str(&s).ok()).flatten()
            },
            descricao: opt(descricao.get_untracked()),
            capacidade_ml: parse_i(capacidade.get_untracked()),
            material: opt(material.get_untracked()),
            cor: opt(cor.get_untracked()),
            altura_mm: cm_para_mm(&altura.get_untracked()),
            diametro_mm: cm_para_mm(&diametro.get_untracked()),
            peso_g: parse_i(peso.get_untracked()),
            largura_base_mm: cm_para_mm(&largura_base.get_untracked()),
            largura_boca_mm: cm_para_mm(&largura_boca.get_untracked()),
            personalizavel: personalizavel.get_untracked(),
            destaque: destaque.get_untracked(),
            ativo: ativo.get_untracked(),
            imagens: imagens.get_untracked(),
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
                    on:change=move |ev| {
                        categoria_id.set(event_target_value(&ev));
                        subcategoria_id.set(String::new());
                    }
                >
                    <option value="">"— Sem categoria —"</option>
                    {move || {
                        categorias
                            .get()
                            .into_iter()
                            .filter(|c| c.parent_id.is_none())
                            .map(|c| {
                                view! { <option value=c.id.to_string()>{c.nome}</option> }
                            })
                            .collect_view()
                    }}
                </select>
            </label>

            {move || {
                let cat = categoria_id.get();
                let subs: Vec<_> = categorias
                    .get()
                    .into_iter()
                    .filter(|c| c.parent_id.map(|p| p.to_string()).as_deref() == Some(cat.as_str()))
                    .collect();
                (!subs.is_empty())
                    .then(|| {
                        view! {
                            <label class="field">
                                <span class="field__label">"Subcategoria"</span>
                                <select
                                    class="admin-input"
                                    prop:value=move || subcategoria_id.get()
                                    on:change=move |ev| subcategoria_id.set(event_target_value(&ev))
                                >
                                    <option value="">"— Sem subcategoria —"</option>
                                    {subs
                                        .into_iter()
                                        .map(|c| {
                                            view! { <option value=c.id.to_string()>{c.nome}</option> }
                                        })
                                        .collect_view()}
                                </select>
                            </label>
                        }
                    })
            }}

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
                    <span class="field__label">"Cores"</span>
                    <input
                        class="admin-input"
                        type="text"
                        prop:value=move || cor.get()
                        on:input=move |ev| cor.set(event_target_value(&ev))
                    />
                </label>
                <label class="field">
                    <span class="field__label">"Altura (cm)"</span>
                    <input
                        class="admin-input"
                        type="text"
                        inputmode="decimal"
                        placeholder="ex.: 20,1"
                        prop:value=move || altura.get()
                        on:input=move |ev| altura.set(event_target_value(&ev))
                    />
                </label>
                <label class="field">
                    <span class="field__label">"Diâmetro (cm)"</span>
                    <input
                        class="admin-input"
                        type="text"
                        inputmode="decimal"
                        placeholder="ex.: 7,1"
                        prop:value=move || diametro.get()
                        on:input=move |ev| diametro.set(event_target_value(&ev))
                    />
                </label>
                <label class="field">
                    <span class="field__label">"Peso unitário (g)"</span>
                    <input
                        class="admin-input"
                        type="number"
                        prop:value=move || peso.get()
                        on:input=move |ev| peso.set(event_target_value(&ev))
                    />
                </label>
                <label class="field">
                    <span class="field__label">"Largura da base (cm)"</span>
                    <input
                        class="admin-input"
                        type="text"
                        inputmode="decimal"
                        placeholder="ex.: 6,1"
                        prop:value=move || largura_base.get()
                        on:input=move |ev| largura_base.set(event_target_value(&ev))
                    />
                </label>
                <label class="field">
                    <span class="field__label">"Largura da boca (cm)"</span>
                    <input
                        class="admin-input"
                        type="text"
                        inputmode="decimal"
                        placeholder="ex.: 7,1"
                        prop:value=move || largura_boca.get()
                        on:input=move |ev| largura_boca.set(event_target_value(&ev))
                    />
                </label>
            </div>

            <div class="field">
                <span class="field__label">
                    "Imagens do produto — selecione uma ou várias (PNG/JPG/WEBP, quadradas 1:1, até 5MB cada)"
                </span>
                <div class="form-galeria">
                    {move || {
                        imagens
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(i, u)| {
                                view! {
                                    <div class="form-galeria__item">
                                        <img src=u alt=""/>
                                        <button
                                            type="button"
                                            class="form-galeria__rm"
                                            aria-label="Remover imagem"
                                            on:click=move |_| img_pendente.set(Some(i))
                                        >
                                            "×"
                                        </button>
                                    </div>
                                }
                            })
                            .collect_view()
                    }}
                    <label class="form-galeria__add">
                        <input
                            class="upload-card__input"
                            type="file"
                            accept="image/png,image/jpeg,image/webp"
                            multiple=true
                            on:change=move |ev| {
                                #[cfg(feature = "hydrate")]
                                {
                                    use wasm_bindgen::JsCast;
                                    if let Some(input) = ev
                                        .target()
                                        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                                    {
                                        // Coleta todos os arquivos selecionados (ordem mantida).
                                        let mut selecionados = Vec::new();
                                        if let Some(files) = input.files() {
                                            for i in 0..files.length() {
                                                if let Some(f) = files.get(i) {
                                                    selecionados.push(f);
                                                }
                                            }
                                        }
                                        input.set_value("");
                                        if selecionados.is_empty() {
                                            return;
                                        }
                                        enviando_prod.set(true);
                                        leptos::task::spawn_local(async move {
                                            // Envia um a um, preservando a ordem da seleção.
                                            for file in selecionados {
                                                let fd = web_sys::FormData::new().unwrap();
                                                let _ = fd.append_with_blob("imagem", &file);
                                                let r = async {
                                                    let req = gloo_net::http::Request::post(
                                                            "/upload-imagem",
                                                        )
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
                                                if let Ok(url) = r {
                                                    imagens.update(|v| v.push(url));
                                                }
                                            }
                                            enviando_prod.set(false);
                                        });
                                    }
                                }
                                #[cfg(not(feature = "hydrate"))]
                                let _ = &ev;
                            }
                        />
                        <span class="form-galeria__add-mais">
                            {move || if enviando_prod.get() { "…" } else { "+" }}
                        </span>
                        <span class="form-galeria__add-txt">"Adicionar"</span>
                    </label>
                </div>
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

            <ModalConfirmacao
                aberto=Signal::derive(move || img_pendente.get().is_some())
                mensagem="Deseja remover esta imagem?"
                confirmar_texto="Remover"
                ao_cancelar=Callback::new(move |()| img_pendente.set(None))
                ao_confirmar=Callback::new(move |()| {
                    if let Some(i) = img_pendente.get_untracked() {
                        imagens.update(|v| {
                            if i < v.len() {
                                v.remove(i);
                            }
                        });
                    }
                    img_pendente.set(None);
                })
            />
        </form>
    }
}
