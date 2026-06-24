use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Produto em listagens (card). Apenas campos exibíveis — sem custos/dados internos.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProdutoResumo {
    pub id: Uuid,
    pub nome: String,
    pub slug: String,
    pub capacidade_ml: Option<i32>,
    pub material: Option<String>,
    pub cor: Option<String>,
    pub destaque: bool,
    pub imagem_url: Option<String>,
}

/// Imagem de produto para exibição.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProdutoImagem {
    pub url: String,
    pub alt: Option<String>,
}

/// Detalhe de produto (página de produto).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProdutoDetalhe {
    pub id: Uuid,
    pub nome: String,
    pub slug: String,
    pub descricao: Option<String>,
    pub capacidade_ml: Option<i32>,
    pub material: Option<String>,
    pub cor: Option<String>,
    pub altura_mm: Option<i32>,
    pub diametro_mm: Option<i32>,
    pub peso_g: Option<i32>,
    pub largura_base_mm: Option<i32>,
    pub largura_boca_mm: Option<i32>,
    pub personalizavel: bool,
    pub categoria_nome: Option<String>,
    pub subcategoria_nome: Option<String>,
    pub imagens: Vec<ProdutoImagem>,
}

/// Filtros de catálogo (todos opcionais) com paginação.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FiltroProdutos {
    pub categoria_slug: Option<String>,
    pub subcategoria_slug: Option<String>,
    pub material: Option<String>,
    pub cor: Option<String>,
    pub busca: Option<String>,
    pub pagina: u32,
    pub por_pagina: u32,
}

/// Página de resultados de produtos.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginaProdutos {
    pub itens: Vec<ProdutoResumo>,
    pub total: i64,
    pub pagina: u32,
    pub por_pagina: u32,
}
