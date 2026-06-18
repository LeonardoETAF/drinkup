use serde::{Deserialize, Serialize};

/// Parceiro exibido na página pública (somente campos visíveis ao cliente).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParceiroPublico {
    pub nome: String,
    pub logo_url: Option<String>,
    pub site_url: Option<String>,
    pub descricao: Option<String>,
    /// Cor da marca (showcase). `None` => cor padrão cíclica.
    pub cor: Option<String>,
    /// Segmento/assinatura curta (ex.: "Tecnologia em delivery").
    pub tagline: Option<String>,
    /// Produtos-exemplo (nomes) exibidos na vitrine da marca.
    pub itens: Vec<String>,
}
