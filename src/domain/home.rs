use serde::{Deserialize, Serialize};

/// Conteúdo editável da home (faixa de números + bento "Sua marca"), já
/// resolvido para exibição. Pares = (valor, rótulo).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HomeConteudo {
    pub numeros: Vec<(String, String)>,
    pub marca_titulo: String,
    pub marca_sub: String,
    pub bento: Vec<(String, String)>,
    pub foto1_url: Option<String>,
    pub foto2_url: Option<String>,
}

/// Formulário do painel (campos crus). `numeros`/`bento` são textos com um item
/// por linha no formato "valor | rótulo".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HomeForm {
    pub numeros: String,
    pub marca_titulo: String,
    pub marca_sub: String,
    pub bento: String,
    pub foto1_url: Option<String>,
    pub foto2_url: Option<String>,
}
