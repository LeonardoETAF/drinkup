//! Tipos compartilhados entre cliente e servidor (DTOs de exibição).
//! Sem lógica de negócio sensível e sem segredos — só o que pode ir ao cliente.
mod admin;
mod categoria;
mod evento;
mod home;
mod orcamento;
mod parceiro;
mod produto;
mod quem_somos;
mod telefone;
mod usuario;

pub use admin::{
    Configuracoes, DashboardResumo, EventoForm, EventoLista, FiltroLeads, LeadResumo, PaginaLeads,
    ParceiroForm, ParceiroLista, ProdutoForm, ProdutoLista, UsuarioForm, UsuarioLista,
};
pub use categoria::Categoria;
pub use evento::EventoCarrossel;
pub use home::{HomeConteudo, HomeForm};
pub use orcamento::{ItemOrcamento, NovoOrcamento};
pub use parceiro::ParceiroPublico;
pub use produto::{FiltroProdutos, PaginaProdutos, ProdutoDetalhe, ProdutoImagem, ProdutoResumo};
pub use quem_somos::{QuemSomosConteudo, QuemSomosForm};
pub use telefone::{link_whatsapp, mascara_telefone};
pub use usuario::UsuarioSessao;
