use serde::{Deserialize, Serialize};

/// Conteúdo editável da página "Quem Somos", já resolvido para exibição.
/// `depoimentos` = pares (texto, autor).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuemSomosConteudo {
    pub stat_titulo: String,
    pub stat_destaque: String,
    pub stat_texto: String,
    pub video_url: Option<String>,
    pub missao: String,
    pub visao: String,
    pub valores: String,
    pub foto1_url: Option<String>,
    pub foto2_url: Option<String>,
    pub depoimentos: Vec<(String, String)>,
}

/// Formulário do painel (campos crus). `depoimentos` é texto com um por linha
/// no formato "texto | autor".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuemSomosForm {
    pub stat_titulo: String,
    pub stat_destaque: String,
    pub stat_texto: String,
    pub video_url: Option<String>,
    pub missao: String,
    pub visao: String,
    pub valores: String,
    pub foto1_url: Option<String>,
    pub foto2_url: Option<String>,
    pub depoimentos: String,
}
