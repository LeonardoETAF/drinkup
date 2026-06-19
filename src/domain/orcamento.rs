use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Item de interesse num pedido de orçamento.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemOrcamento {
    pub produto_id: Option<Uuid>,
    pub descricao: Option<String>,
    pub quantidade: i32,
}

/// Dados enviados pelo cliente ao pedir um orçamento/contato.
/// A validação real e a persistência acontecem no servidor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NovoOrcamento {
    pub nome: String,
    pub contato: String,
    pub email: Option<String>,
    pub mensagem: Option<String>,
    // Codificações de form (URL) omitem coleções vazias; o default evita o erro
    // "missing field `itens`" quando o pedido não traz produtos selecionados.
    #[serde(default)]
    pub itens: Vec<ItemOrcamento>,
}
