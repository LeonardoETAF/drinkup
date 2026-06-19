use leptos::prelude::*;

/// Modal de confirmação do próprio sistema (substitui o `window.confirm` nativo).
/// Controlado por `aberto`; chama `ao_confirmar`/`ao_cancelar` nos botões.
#[component]
pub fn ModalConfirmacao(
    /// Visível quando `true`.
    aberto: Signal<bool>,
    #[prop(into)] mensagem: String,
    /// Texto do botão de confirmação (ex.: "Excluir").
    #[prop(into, default = "Confirmar".to_string())]
    confirmar_texto: String,
    ao_cancelar: Callback<()>,
    ao_confirmar: Callback<()>,
) -> impl IntoView {
    let mensagem = StoredValue::new(mensagem);
    let confirmar_texto = StoredValue::new(confirmar_texto);
    view! {
        <Show when=move || aberto.get() fallback=|| ()>
            <div class="modal-overlay" role="presentation" on:click=move |_| ao_cancelar.run(())>
                <div
                    class="modal"
                    role="dialog"
                    aria-modal="true"
                    on:click=move |ev| ev.stop_propagation()
                >
                    <p class="modal__msg">{move || mensagem.get_value()}</p>
                    <div class="modal__acoes">
                        <button
                            type="button"
                            class="btn btn--ghost"
                            on:click=move |_| ao_cancelar.run(())
                        >
                            "Cancelar"
                        </button>
                        <button
                            type="button"
                            class="btn btn--pink"
                            on:click=move |_| ao_confirmar.run(())
                        >
                            {move || confirmar_texto.get_value()}
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
