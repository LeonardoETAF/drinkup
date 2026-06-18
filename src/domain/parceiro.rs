use serde::{Deserialize, Serialize};

/// Parceiro exibido na página pública (somente campos visíveis ao cliente).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParceiroPublico {
    pub nome: String,
    pub logo_url: Option<String>,
    pub site_url: Option<String>,
    pub descricao: Option<String>,
}
