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

/// Cards padrão do bento "Sua marca" (valor, rótulo), na ordem e cores fixas do
/// layout. Usados como semente no editor do painel e como fallback no render.
pub const BENTO_PADRAO: [(&str, &str); 5] = [
    ("+25K", "Unidades por dia"),
    ("+4", "Anos no mercado"),
    ("+500", "Clientes satisfeitos"),
    ("+2K", "Eventos atendidos"),
    ("100%", "Personalizável"),
];

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
