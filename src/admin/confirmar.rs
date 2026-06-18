//! Confirmação nativa do navegador (executa só no cliente).

#[cfg(feature = "hydrate")]
pub(crate) fn confirmar(msg: &str) -> bool {
    web_sys::window()
        .and_then(|w| w.confirm_with_message(msg).ok())
        .unwrap_or(false)
}

#[cfg(not(feature = "hydrate"))]
pub(crate) fn confirmar(_msg: &str) -> bool {
    false
}
