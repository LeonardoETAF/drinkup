use leptos::prelude::*;

use super::modal::ModalConfirmacao;

const IC_DEL: &str = r#"<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M10 11v6M14 11v6"/></svg>"#;

/// Par editável: id estável + sinais de valor e rótulo.
pub type Par = (usize, RwSignal<String>, RwSignal<String>);

/// Cria um novo par com sinais próprios e o adiciona à lista.
fn novo_par(itens: RwSignal<Vec<Par>>, proximo_id: RwSignal<usize>, valor: String, rotulo: String) {
    let id = proximo_id.get_untracked();
    proximo_id.set(id + 1);
    itens.update(|v| v.push((id, RwSignal::new(valor), RwSignal::new(rotulo))));
}

/// Substitui a lista pelos pares parseados de `texto` (uma linha "valor | rótulo").
pub fn carregar(itens: RwSignal<Vec<Par>>, proximo_id: RwSignal<usize>, texto: &str) {
    itens.set(Vec::new());
    proximo_id.set(0);
    for linha in texto.lines().filter(|l| !l.trim().is_empty()) {
        let (v, r) = match linha.split_once('|') {
            Some((v, r)) => (v.trim().to_string(), r.trim().to_string()),
            None => (linha.trim().to_string(), String::new()),
        };
        novo_par(itens, proximo_id, v, r);
    }
}

/// Serializa os pares de volta ao formato "valor | rótulo" por linha.
#[must_use]
pub fn serializar(itens: RwSignal<Vec<Par>>) -> String {
    itens
        .get_untracked()
        .iter()
        .map(|(_, v, r)| {
            let v = v.get_untracked();
            let r = r.get_untracked();
            let (v, r) = (v.trim(), r.trim());
            if r.is_empty() {
                v.to_string()
            } else {
                format!("{v} | {r}")
            }
        })
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Editor de pares "valor | rótulo": cada item em sua própria linha de campos,
/// com botão de adicionar e remover (este com confirmação).
#[component]
pub fn EditorPares(
    /// Lista (com ids estáveis) editada in-place.
    itens: RwSignal<Vec<Par>>,
    /// Contador de ids (compartilhado com `carregar`).
    proximo_id: RwSignal<usize>,
    #[prop(into)] ph_valor: String,
    #[prop(into)] ph_rotulo: String,
    #[prop(into, default = "+ Adicionar".to_string())] add_texto: String,
    /// Quando `true`, oculta os botões de adicionar e remover (lista fixa).
    #[prop(optional)] fixo: bool,
) -> impl IntoView {
    let pendente = RwSignal::new(None::<usize>);
    let ph_valor = StoredValue::new(ph_valor);
    let ph_rotulo = StoredValue::new(ph_rotulo);
    let add_texto = StoredValue::new(add_texto);

    view! {
        <div class="pares-editor">
            {move || {
                itens
                    .get()
                    .into_iter()
                    .map(|(id, valor, rotulo)| {
                        view! {
                            <div class="pares-edit">
                                <input
                                    class="admin-input pares-edit__valor"
                                    type="text"
                                    placeholder=ph_valor.get_value()
                                    prop:value=move || valor.get()
                                    on:input=move |ev| valor.set(event_target_value(&ev))
                                />
                                <input
                                    class="admin-input"
                                    type="text"
                                    placeholder=ph_rotulo.get_value()
                                    prop:value=move || rotulo.get()
                                    on:input=move |ev| rotulo.set(event_target_value(&ev))
                                />
                                {(!fixo)
                                    .then(|| {
                                        view! {
                                            <button
                                                type="button"
                                                class="icon-btn icon-btn--danger"
                                                title="Remover"
                                                inner_html=IC_DEL
                                                on:click=move |_| pendente.set(Some(id))
                                            ></button>
                                        }
                                    })}
                            </div>
                        }
                    })
                    .collect_view()
            }}
            {(!fixo)
                .then(|| {
                    view! {
                        <button
                            type="button"
                            class="btn btn--ghost pares-editor__add"
                            on:click=move |_| {
                                novo_par(itens, proximo_id, String::new(), String::new())
                            }
                        >
                            {move || add_texto.get_value()}
                        </button>
                    }
                })}
        </div>
        {(!fixo)
            .then(|| {
                view! {
                    <ModalConfirmacao
                        aberto=Signal::derive(move || pendente.get().is_some())
                        mensagem="Deseja remover este item?"
                        confirmar_texto="Remover"
                        ao_cancelar=Callback::new(move |()| pendente.set(None))
                        ao_confirmar=Callback::new(move |()| {
                            if let Some(id) = pendente.get_untracked() {
                                itens.update(|v| v.retain(|(i, _, _)| *i != id));
                            }
                            pendente.set(None);
                        })
                    />
                }
            })}
    }
}
