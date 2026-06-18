use serde::{Deserialize, Serialize};

/// Categoria do carrossel "Do seu jeito" da home (visível ao cliente).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventoCarrossel {
    pub titulo: String,
    pub slug: String,
    pub cor: Option<String>,
    pub imagem_url: Option<String>,
}
