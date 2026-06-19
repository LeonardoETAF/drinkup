use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identidade do usuário autenticado, segura para o cliente.
/// Nunca contém hash de senha nem dados sensíveis.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsuarioSessao {
    pub id: Uuid,
    pub nome: String,
    pub papel: String,
    /// Menus do painel que o usuário pode acessar.
    pub menus: Vec<String>,
}
