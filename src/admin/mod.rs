//! Painel administrativo (rotas protegidas). A autorização é SEMPRE verificada
//! no servidor (sessão + RBAC); a UI aqui é apresentação, não controle de acesso.
mod confirmar;
mod dashboard;
mod em_construcao;
mod evento_form;
mod eventos;
mod layout;
mod leads;
mod login;
mod parceiro_form;
mod parceiros;
mod produto_form;
mod produtos;
mod recuperar;
mod util;

pub use dashboard::AdminDashboard;
pub use em_construcao::AdminEmBreve;
pub use evento_form::AdminEventoForm;
pub use eventos::AdminEventos;
pub use layout::AdminLayout;
pub use leads::AdminLeads;
pub use login::AdminLoginPage;
pub use parceiro_form::AdminParceiroForm;
pub use parceiros::AdminParceiros;
pub use produto_form::AdminProdutoForm;
pub use produtos::AdminProdutos;
pub use recuperar::AdminRecuperarSenhaPage;
