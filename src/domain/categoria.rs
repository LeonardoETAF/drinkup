use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Categoria de copos para exibição/filtro. Quando `parent_id` está definido, é
/// uma subcategoria daquela categoria (hierarquia de 2 níveis).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Categoria {
    pub id: Uuid,
    pub nome: String,
    pub slug: String,
    pub descricao: Option<String>,
    pub parent_id: Option<Uuid>,
}
