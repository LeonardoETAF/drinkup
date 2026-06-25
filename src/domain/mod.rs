//! Tipos compartilhados entre cliente e servidor (DTOs de exibição).
//! Sem lógica de negócio sensível e sem segredos — só o que pode ir ao cliente.
mod admin;
mod categoria;
mod evento;
mod home;
mod medidas;
mod orcamento;
mod parceiro;
mod produto;
mod quem_somos;
mod telefone;
mod usuario;

pub use admin::{
    Configuracoes, DashboardResumo, DiaAcesso, EventoForm, EventoLista, FiltroLeads,
    InscritoResumo, ItemRanking, LeadResumo, Pagina, PaginaInscritos, PaginaLeads,
    PaginaProdutosLista, ParceiroForm, ParceiroLista, ProdutoForm, ProdutoLista, UsuarioForm,
    UsuarioLista,
    ADMIN_TABELA_POR_PAGINA, PRODUTOS_ADMIN_POR_PAGINA,
};
pub use categoria::Categoria;
pub use evento::EventoCarrossel;
pub use home::{HomeConteudo, HomeForm, BENTO_PADRAO};
pub use medidas::{cm_para_mm, mm_para_cm};
pub use orcamento::{ItemOrcamento, NovoOrcamento};
pub use parceiro::ParceiroPublico;
pub use produto::{FiltroProdutos, PaginaProdutos, ProdutoDetalhe, ProdutoImagem, ProdutoResumo};
pub use quem_somos::{QuemSomosConteudo, QuemSomosForm};
pub use telefone::{link_whatsapp, mascara_telefone, telefone_valido, whatsapp_valido};
pub use usuario::UsuarioSessao;
