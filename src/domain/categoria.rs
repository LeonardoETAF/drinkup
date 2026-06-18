use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Categoria de copos para exibição/filtro.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Categoria {
    pub id: Uuid,
    pub nome: String,
    pub slug: String,
    pub descricao: Option<String>,
}
