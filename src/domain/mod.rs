//! Tipos compartilhados entre cliente e servidor (DTOs de exibição).
//! Sem lógica de negócio sensível e sem segredos — só o que pode ir ao cliente.
mod categoria;
mod orcamento;
mod produto;

pub use categoria::Categoria;
pub use orcamento::{ItemOrcamento, NovoOrcamento};
pub use produto::{FiltroProdutos, PaginaProdutos, ProdutoDetalhe, ProdutoImagem, ProdutoResumo};
